//! Macros to support rdom.

#![deny(
    missing_docs,
    // missing_debug_implementations,
    // missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

extern crate proc_macro;

use std::convert::{TryFrom, TryInto};

use quote::quote;
use proc_macro::{TokenStream};
use syn::{Attribute, DeriveInput, Expr, Item, ItemConst, ItemImpl, ItemStruct, Meta, NestedMeta, Stmt, parse::{Parse, ParseStream}, parse_macro_input, token::Struct};

#[derive(Debug)]
enum DerivableClasses {
    Node,
    Element,
    ParentNode
}

impl TryFrom<String> for DerivableClasses {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "Node" {
            Ok(DerivableClasses::Node)
        } else if value == "Element" {
            Ok(DerivableClasses::Element)
        } else if value == "ParentNode" {
            Ok(DerivableClasses::ParentNode)
        } else {
            Err(())
        }
    }
}


struct NodeImplDecl();

impl Parse for NodeImplDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut deriving_classes = Vec::new();
        let decl: DeriveInput = input.parse()?;
        for attr in decl.attrs {
            let path = attr.clone().path;
            if path.segments.len() == 1 {
                let first_seg = path.segments.first().unwrap().clone();
                let first_ident = first_seg.ident.to_string();

                if first_ident == "derive" {
                    let derive_decl = NodeDeriveDecl::from(attr)?;
                    deriving_classes.extend(derive_decl.deriving_classes)
                }// else if first_ident == "core" {
                //    let key_value = TypeRelationDecl::from(attr)?;
                //    // TODO use core = Whatever
                //}
            }
        }
        // input.parse::<Token![(]>()?;
        // let declared_type: Type = input.parse()?;
        // {
        //     let keyword = input.parse::<Ident::peek_any>()?;
        //     assert!(keyword == "core");
        //     input.parse::<Token![:]>()?;
        // }
        // input.parse::<Token![)]>()?;

        // (
        //     TextNode,
        //     core: node::TextNode,
        //     blurb: "text",
        //     link: "Text",
        //     impl {}
        // )
        Ok(NodeImplDecl())
    }
}

struct NodeDecl {

};

impl NodeDecl {
    fn visit_struct(&mut self, block: &ItemStruct) {
//        attrs: Vec<Attribute>
//vis: Visibility
//struct_token: Struct
//ident: Ident
//generics: Generics
//fields: Fields
//semi_token: Option<Semi>`
        if let Visibility::Public(_) = block.vis {
            if block.attrs {

            }
        } else {

        }
    }

    fn visit_impl(&mut self, block: &ItemImpl) {

    }
}

impl Parse for NodeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut result = NodeDecl {};
        let decl: ItemConst = input.parse()?;
        if let Expr::Block(block) = *decl.expr {
            let block = block.block;
            if block.stmts.len() != 2 {
                return Err(syn::Error::new_spanned(
                    block,
                    "A #[derive_node] block must contain at least two items"
                ))
            }
            let struct_stmt = block.stmts.get(0);;
            let impl_stmt = block.stmts.get(1);

            if let Some(Stmt::Item(Item::Struct(struct_block))) = struct_stmt {
                result.visit_struct(struct_block);
            } else {
                return Err(syn::Error::new_spanned(
                    struct_stmt,
                    "A #[derive_node] block must have a first item that is a struct"
                ))
            }

            if let Some(Stmt::Item(Item::Impl(impl_block))) = impl_stmt {
                result.visit_impl(impl_block);
            } else {
                return Err(syn::Error::new_spanned(
                    impl_stmt,
                    "A #[derive_node] block must have a second item that is an impl"
                ))
            }
        }
        Ok(result)
    }
}

struct NodeDeriveDecl {
    pub(crate) deriving_classes: Vec<DerivableClasses>,
}

impl NodeDeriveDecl {
    fn from(attr: Attribute) -> syn::Result<Self> {
        let mut deriving_classes = Vec::new();
        match attr.parse_meta()? {
            Meta::List(list) => {
                for derived in list.nested.iter() {
                    if let NestedMeta::Meta(meta) = derived {                                        
                        match meta {
                            Meta::Path(path) => {
                                match path.get_ident() {
                                    None => {
                                        return Err(syn::Error::new_spanned(
                                            path,
                                            "#[derive(...)] must contain a list of DOM classes (e.g. Node, Element)"
                                        ))
                                    },
                                    Some(ident_token) => {
                                        let ident = ident_token.to_string();
                                        let class: Option<DerivableClasses> = ident.clone().try_into().ok();
                                        if let Some(class) = class {
                                            deriving_classes.push(class);
                                        } else {
                                            return Err(syn::Error::new_spanned(
                                                ident_token,
                                                format!("unknown class name in #[derive(...)]: {}", ident)
                                            ))
                                        }
                                    }
                                }
                            },
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    meta,
                                    "#[derive(...)] must contain a list of DOM classes (e.g. Node, Element)"
                                ))
                            }
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                            derived,
                            "#[derive(...)] must contain a list of DOM classes, not literals"
                        ))
                    }
                }
                Ok(NodeDeriveDecl { deriving_classes })
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    attr,
                    "#[derive(...)]: invalid contents"
                ))
            }
        }
    }
}

#[proc_macro_attribute]
/// DOes a thing
pub fn declare_node(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as NodeDecl);

    let postlude = Some("");
    let mut blurb = "The [{}](https://developer.mozilla.org/en-US/docs/Web/API/{}) node type".to_string();
    if let Some(postlude) = postlude {
        blurb.push_str(" ");
        blurb.push_str(postlude);
    }

    let struct_name = "Foobar";

    let decl = format!(r#"
        #[doc = {blurb}]
        pub struct {struct_name} {{
            // /// Reference to the sandbox to which this node belongs
            // pub context: Weak<Sandbox>,

            // /// Node behavior (fields/methods associated with the DOM class called Node)
            // pub(crate) node_behavior: Arc<NodeBehavior>,

            // pub(crate) storage: $storage,
        }}
    "#, blurb=quote!(#blurb).to_string(), struct_name=struct_name).parse().unwrap();

    decl

    // impl $ty {
    //     pub(crate) fn new(context: Weak<Sandbox>, storage: $storage) -> Arc<$ty> {
    //         let construction: Arc<$ty> = Arc::new_cyclic(|construction_weak| -> $ty {
    //             $ty {
    //                 context,
    //                 node_behavior: Arc::new(NodeBehavior::new(construction_weak.clone())),
    //                 storage,
    //             }
    //         });

    //         construction
    //     }

    //     $($rest)*
    // }

    // impl AnyNode for $ty {
    //     fn get_context(&self) -> Weak<Sandbox> {
    //         self.context.clone()
    //     }

    //     fn clone_node(&self) -> Arc<dyn AnyNode> {
    //         let mut construction = $ty::new(self.get_context(), Default::default());

    //         let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
    //         (*cons).storage = self.storage.clone();

    //         construction
    //     }

    //     fn first_child(&self) -> Option<Arc<dyn AnyNode>> {
    //         self.node_behavior.first_child()
    //     }

    //     fn last_child(&self) -> Option<Arc<dyn AnyNode>> {
    //         self.node_behavior.last_child()
    //     }

    //     fn append_child(&self, other: Arc<dyn AnyNode>) {
    //         self.node_behavior.append_child(other)
    //     }

    //     fn child_nodes(&self) -> Arc<NodeList> {
    //         self.node_behavior.child_nodes()
    //     }
    // }

    // impl PrivateAnyNode for $ty {
    //     fn get_node_behavior(&self) -> Arc<NodeBehavior> {
    //         self.node_behavior.clone()
    //     }
    // }
}

#[proc_macro_attribute]
/// Foo
pub fn impl_node(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}