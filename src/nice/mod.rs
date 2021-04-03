//! Wrapped representation of a DOM Node. See [node](../index.html) module for distinction from
//! raw representation.

use paste::paste;

use std::convert::TryFrom;
use std::result::Result;
use std::sync::{Arc, Weak};

use crate::internal_prelude::*;

pub mod element;

/// A base trait for all wrapped node types
pub trait AnyWrappedNode {
    /// Gives a weak reference to the sandbox the node was created in.
    fn get_context(&self) -> Weak<Sandbox>;
}

#[macro_export]
/// Provides the trait implementations for all wrapped node types
macro_rules! node_base {
    ($ty: ty, impl { $($rest:tt)* }) => {
        impl AnyWrappedNode for $ty {
            fn get_context(&self) -> Weak<$crate::sandbox::Sandbox> {
                self.0.clone().get_context()
            }
        }

        impl $ty {
            $($rest)*
        }
    }
}

macro_rules! impl_wrapped_nodes {
    ($((
        $ty: ty,
        raw: $raw_ty: ty,
        blurb: $blurb: literal,
        link: $link: literal,
        impl { $( $rest:tt )* }
        $(, $postlude: literal)?
    ))*) => {
        $(
            paste! {
                #[doc =
                    "A wrapped ["
                    $blurb
                    "](https://developer.mozilla.org/en-US/docs/Web/API/"
                    $link
                    ") node"
                    $(" " $postlude)?
                ]
                pub struct $ty(pub Arc<$raw_ty>);

                node_base!($ty, impl {
                    pub(crate) fn new(context: Weak<$crate::sandbox::Sandbox>) -> Self {
                        Self(<$raw_ty>::new(context, Default::default()))
                    }
                    $($rest)*
                });

                impl From<$ty> for Node {
                    fn from(source: $ty) -> Node {
                        Node(source.0)
                    }
                }

                impl TryFrom<Node> for $ty {
                    type Error = Node;

                    fn try_from(elem: Node) -> Result<$ty, Node> {
                        elem.0
                            .downcast_arc::<$raw_ty>()
                            .map($ty)
                            .map_err(Node)
                    }
                }
            }
        )*
    }
}

impl_wrapped_nodes! {
    (
        TextNode,
        raw: node::TextNode,
        blurb: "text",
        link: "Text",
        impl {}
    )
    (
        Document,
        raw: node::Document,
        blurb: "document",
        link: "Document",
        impl {
            fn query_selector(&self, selectors: &str) -> Result<Option<wrapped_element::Element>, DomError> {
                if self.get_context().upgrade().is_none() {
                    return Err(DomError::SandboxDropped)
                }
                match selectors {
                    //"html" => {
                    //    Ok(Some(self.document_element.into()))
                    //},
                    //"body" => Ok(Some((&*self.body).into())),
                    _ => Err(DomError::InvalidQuerySelector),
                }
            }
        }
    )
}

/// Any wrapped Node
pub struct Node(pub Arc<dyn AnyRawNode>);
node_base!(Node, impl {});
