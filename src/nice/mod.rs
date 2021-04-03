//! This module contains a nicer, public representation of Nodes and Elements. This is
//! nice in comparison to what rdom calls the "common" representation of Nodes and
//! Elements, which is a bit more cumbersome to deal with in some cases.
//!
//! For most purposes, a nice element is what you want. Nice elements store
//! an `Arc` of the common element, which ensures that the underlying common element is retained
//! as long as you maintain that reference to it. (This is how all `Arc`s work.)
//!
//! For some DOM operations, ownership of said `Arc` (or nice element) is sufficient
//! to perform the operation. However, this `Arc` does not ensure that the whole sandbox
//! is retained, due to the possibility that the sandbox is dropped at an arbitrary time
//! while you hold this reference.
//!
//! As a result, you must be careful to not drop the sandbox until you are totally done
//! performing DOM operations, else you may find that those operations fail.
//!
//! Rdom opts for weak pointers in all but one direction (down), so if the sandbox is
//! dropped, most of the elements will be dropped with it. This design is
//! chosen to help with preventing memory leaks, but it has the side effect of causing some
//! operations (such as getting the parent node of an element) to fail at runtime.

use paste::paste;

use std::convert::TryFrom;
use std::result::Result;
use std::sync::{Arc, Weak};

use crate::{behavior::sandbox_member::SandboxMemberBehaviour, internal_prelude::*};

pub mod element;

/// A base trait for all wrapped node types
pub trait AnyWrappedNode: SandboxMemberBehaviour {}

#[macro_export]
/// Provides the trait implementations for all wrapped node types
macro_rules! node_base {
    ($ty: ty, impl { $($rest:tt)* }) => {
        impl SandboxMemberBehaviour for $ty {
            fn get_context(&self) -> Weak<Sandbox> {
                self.0.clone().get_context()
            }
        }

        impl AnyWrappedNode for $ty {}

        impl $ty {
            $($rest)*
        }
    }
}

macro_rules! impl_nice_nodes {
    ($((
        $ty: ty,
        common: $common_ty: ty,
        blurb: $blurb: literal,
        link: $link: literal,
        impl { $( $rest:tt )* }
        $(, $postlude: literal)?
    ))*) => {
        $(
            paste! {
                #[doc =
                    "A nice version of ["
                    $blurb
                    "](https://developer.mozilla.org/en-US/docs/Web/API/"
                    $link
                    ") node"
                    $(" " $postlude)?
                ]
                pub struct $ty(pub Arc<$common_ty>);

                node_base!($ty, impl {
                    pub(crate) fn new(context: Weak<$crate::sandbox::Sandbox>) -> Self {
                        Self(<$common_ty>::new(context, Default::default()))
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
                            .downcast_arc::<$common_ty>()
                            .map($ty)
                            .map_err(Node)
                    }
                }
            }
        )*
    }
}

impl_nice_nodes! {
    (
        TextNode,
        common: node::TextNode,
        blurb: "text",
        link: "Text",
        impl {}
    )
    (
        Document,
        common: node::Document,
        blurb: "document",
        link: "Document",
        impl {
            fn query_selector(&self, selectors: &str) -> Result<Option<nice_element::Element>, DomError> {
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
pub struct Node(pub Arc<dyn AnyNode>);
node_base!(Node, impl {});
