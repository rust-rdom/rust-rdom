use anyhow::anyhow;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use sourcegen_cli::tokens::NewLine;
use sourcegen_cli::SourceGenerator;
use std::collections::HashMap;
use syn::{Generics, Ident, Lit, Meta, MetaNameValue, NestedMeta, Type, Visibility};

#[derive(Clone)]
struct Template {
    args: Vec<&'static str>,
    ident: Ident,
    gen: Option<Generics>,
    mixins: Vec<MixinSpec>,
    force_struct_declaration: bool,
}

#[derive(Clone)]
enum MixinSpec {
    ElementStoreOuter,
    ParentNode,
    NodeStorage,
    Node { node_type: String },
    SandboxMember { field: String },
}

struct FieldSpec {
    ident: Ident,
    ty: Type,
    vis: Visibility,
}

impl MixinSpec {
    fn get_fields(&self) -> Vec<FieldSpec> {
        match self {
            MixinSpec::NodeStorage => {
                let mut v = Vec::new();
                v.push(FieldSpec {
                    ident: Ident::new("parent_node", Span::call_site()),
                    ty: syn::parse_str("Option<AnyNodeWeak>").unwrap(),
                    vis: syn::parse_str("").unwrap(),
                });
                v.push(FieldSpec {
                    ident: Ident::new("left_sibling", Span::call_site()),
                    ty: syn::parse_str("Option<AnyNodeWeak>").unwrap(),
                    vis: syn::parse_str("").unwrap(),
                });
                v.push(FieldSpec {
                    ident: Ident::new("right_sibling", Span::call_site()),
                    ty: syn::parse_str("Option<AnyNodeWeak>").unwrap(),
                    vis: syn::parse_str("").unwrap(),
                });
                v.push(FieldSpec {
                    ident: Ident::new("child_nodes", Span::call_site()),
                    ty: syn::parse_str("RwLock<Vec<AnyNodeArc>>").unwrap(),
                    vis: syn::parse_str("").unwrap(),
                });

                v
            },
            MixinSpec::ElementStoreOuter => {
                let mut v = Vec::new();
                v.push(FieldSpec {
                    ident: Ident::new("inner", Span::call_site()),
                    ty: syn::parse_str("ElementStore").unwrap(),
                    vis: syn::parse_str("").unwrap(),
                });

                v
            }
            _ => Vec::new(),
        }
    }

    fn get_fn_impls(&self, template: &Template) -> Option<TokenStream> {
        let Template { ident, gen, .. } = template.clone();

        let gen = gen.unwrap_or(Default::default());
        let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

        match self {
            MixinSpec::Node { node_type } => {
                let ident_store = {
                    let store_name = format!("{}Store", node_type);
                    Ident::new(store_name.as_ref(), Span::call_site())
                };
                Some(quote! {
                    pub(crate) fn new(context: Weak<Sandbox>, contents: Arc<#ident_store>) ->
                    ConcreteNodeArc<#ident_store> {
                        let common = Arc::new_cyclic(|construction_weak| NodeCommon {
                            context,
                        });

                        ConcreteNodeArc { contents, common }
                    }
                })
            },
            _ => None,
        }
    }

    fn get_trait_impls(&self, template: &Template) -> Option<TokenStream> {
        match self {
            MixinSpec::SandboxMember { field } => {
                let Template { ident, gen, .. } = template.clone();

                let gen = gen.unwrap_or(Default::default());
                let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

                let field = Ident::new(field, Span::call_site());

                Some(quote! {
                    #NewLine
                    #[sourcegen::generated]
                    impl #impl_generics #ident #ty_generics #where_clause{
                        /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
                        pub fn get_context(&self) -> Weak<Sandbox> {
                            self.#field.clone()
                        }
                    }
                    #NewLine
                    #[sourcegen::generated]
                    impl #impl_generics SandboxMemberBehavior for #ident #ty_generics #where_clause{
                        /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
                        fn get_context(&self) -> Weak<Sandbox> {
                            self.get_context()
                        }
                    }
                })
            }
            _ => None,
        }
    }

    fn get_accessory_types(&self, template: &Template) -> Option<TokenStream> {
        let Template { ident, gen, .. } = template.clone();

        let gen = gen.unwrap_or(Default::default());
        let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

        match self {
            MixinSpec::Node { node_type } => {
                let ident_store = Ident::new(&format!("{}Store", node_type), Span::call_site());
                let ident_weak = Ident::new(&format!("{}NodeWeak", node_type), Span::call_site());
                let ident_arc = Ident::new(&format!("{}NodeArc", node_type), Span::call_site());
                Some(quote! {
                    #[sourcegen::generated]
                    impl AnyNodeStore for #ident_store {}

                    #[doc = "Convenience alias for a strong reference to a(n) " #ident " node"]
                    #[sourcegen::generated]
                    pub type #ident_arc = ConcreteNodeArc<#ident_store>;

                    #[doc = "Convenience alias for a weak reference to a(n) " $name " node"]
                    #[sourcegen::generated]
                    pub type #ident_weak = ConcreteNodeWeak<#ident_store>;
                })
            },
            _ => None,
        }
    }
}

impl Template {
    fn new(args: &[&'static str], signature: &'static str, mixins: &[MixinSpec], force_struct_declaration: bool) -> Self {
        let (ident_raw, gen_raw) = signature
            .split_once("<")
            .map(|(ty, rest)| (ty, rest.strip_suffix(">").expect("Unclosed <>")))
            .unwrap_or((signature, ""));

        let ident = Ident::new(&ident_raw, Span::call_site());
        let gen: Option<Generics> = if gen_raw == "" {
            None
        } else {
            Some(syn::parse_str(&format!("<{}>", gen_raw)).expect("Could not parse generics"))
        };

        Self {
            args: args.into(),
            ident,
            gen,
            mixins: mixins.into(),
            force_struct_declaration,
        }
    }
}

fn get_templates() -> HashMap<String, Template> {
    let mut m = HashMap::new();
    m.insert(
        "element_store".to_owned(),
        Template::new(
            &[],
            "ElementStoreOuter",
            &[
                MixinSpec::NodeStorage,
                MixinSpec::ElementStoreOuter,
            ],
            false,
        ),
    );
    m.insert(
        "concrete_node_element".to_owned(),
        Template::new(
            &[],
            "ConcreteNodeArc<ElementStore>",
            &[
                MixinSpec::Node {
                    node_type: "Element".to_owned(),
                },
            ],
            false,
        ),
    );
    m
}

pub struct InjectionGenerator;

impl SourceGenerator for InjectionGenerator {
    fn generate_struct(
        &self,
        args: syn::AttributeArgs,
        item: &syn::ItemStruct,
    ) -> Result<Option<TokenStream>, anyhow::Error> {
        let template_name = {
            let mut template = None;
            for arg in args.into_iter() {
                if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = arg {
                    let path = path
                        .get_ident()
                        .expect("Could not coerce path to an identifier");
                    match path.to_string().as_str() {
                        "template" => {
                            if let Lit::Str(s) = lit {
                                match template {
                                    None => {
                                        template = Some(s.value());
                                    }
                                    Some(_) => {
                                        return Err(anyhow!(
                                            "duplicate template argument specified"
                                        ))
                                    }
                                }
                            }
                        }
                        "generator" => {}
                        unknown => {
                            return Err(anyhow!(format!(
                                "unknown argument specified to inject: {}",
                                unknown
                            )))
                        }
                    }
                } else {
                    return Err(anyhow!(format!("invalid inject arg")));
                }
            }

            if template.is_none() {
                return Err(anyhow!("no template specified for inject"));
            }

            template
        }
        .unwrap();

        let templates = get_templates();
        let template = templates.get(&template_name).expect("Nonexistent template");

        let Template {
            ident,
            gen,
            mixins,
            ..
        } = template.clone();

        let mut postlude = TokenStream::new();
        for mixin in mixins.clone() {
            if let Some(tokens) = mixin.get_trait_impls(template) {
                postlude.extend(tokens);
            }

            if let Some(tokens) = mixin.get_accessory_types(template) {
                postlude.extend(tokens);
            }
        }

        let mut fields = TokenStream::new();
        let mut has_fields = false;
        for mixin in mixins.clone() {
            for field in mixin.get_fields() {
                let FieldSpec { ident, ty, vis } = field;
                fields.extend(quote! {
                    #vis #ident: #ty,
                });
                has_fields = true;
            }
        }

        let fn_impls = mixins.iter().map(|m| m.get_fn_impls(template));
        if fn_impls.clone().any(|imp| imp.is_some()) {
            let mut stream = TokenStream::new();
            for fn_impl in fn_impls {
                stream.extend(fn_impl);
            }

            let gen = gen.clone().unwrap_or(Default::default());
            let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

            postlude.extend(quote! {
                #NewLine
                #[sourcegen::generated]
                impl #ident #ty_generics #where_clause {
                    #stream
                }
            })
        }


        if has_fields || template.force_struct_declaration {
            Ok(Some(quote! {
                #NewLine
                #[sourcegen::generated]
                struct #ident#gen {
                    #fields
                }
                #postlude
            }))
        } else {
            Ok(Some(quote! {
                #NewLine
                #postlude
            }))
        }

    }
}
