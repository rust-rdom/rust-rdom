use anyhow::anyhow;
use lazy_static::lazy_static;
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
}

#[derive(Clone)]
enum MixinSpec {
    ParentNode,
    Node,
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
            MixinSpec::Node => {
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
            }
            _ => Vec::new(),
        }
    }

    fn get_fn_impls(&self, _template: &Template) -> Option<TokenStream> {
        match self {
            MixinSpec::Node => Some(quote! {
                fn foo(&self) {
                }
            }),
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
}

impl Template {
    fn new(args: &[&'static str], signature: &'static str, mixins: &[MixinSpec]) -> Self {
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
        }
    }
}

fn get_templates() -> HashMap<String, Template> {
    let mut m = HashMap::new();
    m.insert(
        "food".to_owned(),
        Template::new(&[], "Fibb", &[MixinSpec::Node]),
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
            args: _,
        } = template;

        let mut postlude = TokenStream::new();
        for mixin in mixins {
            if let Some(tokens) = mixin.get_trait_impls(template) {
                postlude.extend(tokens);
            }
        }

        let mut fields = TokenStream::new();
        for mixin in mixins {
            for field in mixin.get_fields() {
                let FieldSpec { ident, ty, vis } = field;
                fields.extend(quote! {
                    #vis #ident: #ty,
                });
            }
        }

        let fn_impls = mixins.iter().map(|m| m.get_fn_impls(template));
        if fn_impls.clone().any(|imp| imp.is_some()) {
            let mut stream = TokenStream::new();
            for fn_impl in fn_impls {
                stream.extend(fn_impl);
            }

            postlude.extend(quote! {
                impl #ident#gen {
                    #stream
                }
            })
        }

        Ok(Some(quote! {
            // #item
            #NewLine
            struct #ident#gen {
                #fields
            }
            #postlude
        }))
    }
}

// fn sandbox_member(item: &syn::ItemStruct, args: Vec<String>) -> Result<TokenStream, anyhow::Error> {
//     let ident = &item.ident;
//     let (impl_generics, ty_generics, where_clause) = &item.generics.split_for_impl();

//     let mut args = args.into_iter();

//     let field: TokenStream = match (args.next(), args.next()) {
//         (Some(field), None) => Ok(field),
//         _ => Err(anyhow!(
//             "Correct usage for SandboxMember: SandboxMember context"
//         )),
//     }?
//     .parse()
//     .unwrap();

//     Ok(quote! {
//         #[sourcegen::generated]
//         impl #impl_generics #ident #ty_generics #where_clause{
//             /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
//             pub fn get_context(&self) -> Weak<Sandbox> {
//                 self.#field.clone()
//             }
//         }
//         #NewLine
//         #[sourcegen::generated]
//         impl #impl_generics SandboxMemberBehavior for #ident #ty_generics #where_clause{
//             fn get_context(&self) -> Weak<Sandbox> {
//                 self.get_context()
//             }
//         }
//     })
// }
