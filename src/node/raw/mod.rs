//! Raw representation of a DOM node. See [node](../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;
use paste::paste;

use std::rc::Rc;
use std::sync::{Arc, Weak};

use crate::error::DomError;
use crate::sandbox::Sandbox;

pub mod element;

/// Linkages to other nodes
pub struct NodeLinkages {
    /// Descendant nodes of this node
    pub children: Vec<Weak<dyn AnyRawNode>>,
    /// Parent node of this node
    pub parent: Option<Weak<dyn AnyRawNode>>,
    /// Right sibling of this node
    pub right_sibling: Option<Weak<dyn AnyRawNode>>,
    /// Left sibling of this node
    pub left_sibling: Option<Weak<dyn AnyRawNode>>,
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

                    /// Linkages to other nodes
                    pub linkages: NodeLinkages,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
                        $ty {
                            context,
                            linkages: NodeLinkages {
                                parent: None,
                                left_sibling: None,
                                right_sibling: None,
                                children: Vec::new()
                            }
                        }
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
