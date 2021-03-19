//! Wrapped representation of a DOM Element. See [node](../index.html) module for distinction from
//! raw representation.

use downcast_rs::DowncastSync;
use paste::paste;

use std::convert::TryFrom;
use std::result::Result;
use std::sync::{Arc, Weak};

use crate::error::DomError;
use crate::node::raw::{self as raw_node, element as raw_element};
use crate::sandbox::Sandbox;

/// Any wrapped Element
pub struct Element(Arc<dyn raw_element::AnyRawElement>);

/// Any wrapped Node
pub struct Node(Arc<dyn raw_node::AnyRawNode>);

macro_rules! impl_wrapped_nodes {
    ($((
        $ty: ty,
        $raw_ty: ty,
        $blurb: literal,
        $link: literal,
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
                pub struct $ty(Arc<$raw_ty>);
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
                        $ty(Arc::new($raw_ty::new(context)))
                    }

                    $($rest)*
                }
                impl raw_node::AnyRawNode for $ty {}

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
        raw_node::TextNode,
        "text",
        "Text",
        impl {}
    )
    (
        Document,
        raw_node::Document,
        "document",
        "Document",
        impl {
            fn query_selector(&self, selectors: &str) -> Result<Option<Element>, DomError> {
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