//! Raw representation of a DOM node. See [node](../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;
use paste::paste;

use std::rc::Rc;
use std::sync::{Arc, Weak};

use crate::error::DomError;
use crate::sandbox::Sandbox;

pub mod element;

/// The common structure of all DOM nodes
pub struct Node {
    context: Option<Weak<Sandbox>>,
    children: Vec<Arc<Node>>,
    parent: Option<Weak<Node>>,
    right_sibling: Option<Weak<Node>>,
    left_sibling: Option<Weak<Node>>,
}

/// An input event
pub struct InputEvent {}

/// A base trait for all raw node types
pub trait AnyRawNode: DowncastSync {
    /// Gives a weak reference to the sandbox the node was created in.
    fn get_context(&self) -> Weak<Sandbox>;
}
impl_downcast!(sync AnyRawNode);

macro_rules! impl_raw_nodes {
    ($((
        $ty: ty,
        $blurb: literal,
        $link: literal,
        impl { $( $rest:tt )* }
        $(, $postlude: literal)?
    ))*) => {
        $(
            paste! {
                #[doc =
                    "The ["
                    $blurb
                    "](https://developer.mozilla.org/en-US/docs/Web/API/"
                    $link
                    ") node type"
                    $(" " $postlude)?
                ]
                pub struct $ty {
                    /// Reference to the sandbox to which this node belongs
                    pub context: Weak<Sandbox>,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
                        $ty { context }
                    }

                    $($rest)*
                }
                impl AnyRawNode for $ty {
                    fn get_context(&self) -> Weak<Sandbox> {
                        self.context.clone()
                    }
                }
            }
        )*
    }
}

impl_raw_nodes! {
    (
        TextNode,
        "text",
        "Text",
        impl {}
    )
    (
        Document,
        "document",
        "Document",
        impl {}
    )
}
