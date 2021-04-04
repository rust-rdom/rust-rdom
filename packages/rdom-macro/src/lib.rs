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

use quote::quote;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Token, parse_macro_input};


struct NodeDecl();

impl Parse for NodeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![struct]>();
        Ok(NodeDecl())
    }
}

#[proc_macro]
/// DOes a thing
pub fn declare_node(input: TokenStream) -> TokenStream {
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