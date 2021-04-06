//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use paste::paste;
use std::sync::{Arc, Weak};

use crate::behavior::NodeBehavior;
use crate::internal_prelude::*;
use crate::node_list::NodeList;
use crate::sandbox::Sandbox;

use super::query_selector::query_selector;

/// A base trait for all core element types
pub trait AnyElement: DowncastSync + AnyNode {}
impl_downcast!(sync AnyElement);

macro_rules! impl_elements {
    ($((
        $ty: ty,
        storage: $storage: ty,
        blurb: $blurb: literal,
        link: $link: literal,
        tag: $tag: literal,
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
                    ") element type"
                    $(" " $postlude)?
                ]
                #[derive(Debug)]
                pub struct $ty {
                    /// Reference to the sandbox to which this element belongs
                    pub context: Weak<Sandbox>,

                    /// Node behavior (fields/methods associated with the DOM class called Node)
                    pub(crate) node_behavior: Arc<NodeBehavior>,

                    pub(crate) storage: $storage,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>, storage: $storage) -> Arc<$ty> {
                        let construction: Arc<$ty> = Arc::new_cyclic(|construction_weak| -> $ty {
                            $ty {
                                context,
                                node_behavior: Arc::new(NodeBehavior::new(construction_weak.clone())),
                                storage
                            }
                        });

                        construction
                    }
                }

                impl AnyElement for $ty {}

                impl AnyNode for $ty {
                    fn get_context(&self) -> Weak<Sandbox> {
                        self.context.clone()
                    }

                    fn clone_node(&self) -> Arc<dyn AnyNode> {
                        // TODO this call to clone should really be something
                        // other than the standard clone trait. It is (will be/should be)
                        // our own logic specific to rdom and NOT just a verbatim clone.
                        $ty::new(self.get_context(), self.storage.clone())
                    }

                    fn first_child(&self) -> Option<Arc<dyn AnyNode>> {
                        self.node_behavior.first_child()
                    }

                    fn last_child(&self) -> Option<Arc<dyn AnyNode>> {
                        self.node_behavior.last_child()
                    }

                    fn append_child(&self, other: Arc<dyn AnyNode>) {
                        self.node_behavior.append_child(other)
                    }

                    fn child_nodes(&self) -> Arc<NodeList> {
                        self.node_behavior.child_nodes()
                    }

                    fn tag_name(&self) -> String {
                        String::from($tag)
                    }

                    fn query_selector(&self, selector: &str) -> Result<Option<Arc<dyn AnyNode>>, DomError> {
                        query_selector(self, selector)
                    }
                 }

                impl PrivateAnyNode for $ty {
                    fn get_node_behavior(&self) -> Arc<NodeBehavior> {
                        self.node_behavior.clone()
                    }
                }
            }
        )*
    }
}

impl_elements! {
    (
        HtmlHtmlElement,
        storage: (),
        blurb: "root document element",
        link: "Document/documentElement",
        tag: "HTML",
        impl {},
        "(&lt;HTML /&gt;)"
    )
    (
        HtmlBodyElement,
        storage: (),
        blurb: "body",
        link: "Document/body",
        tag: "BODY",
        impl {},
        "(&lt;BODY /&gt;)"
    )
    (
        HtmlButtonElement,
        storage: (),
        blurb: "button",
        link: "HTMLButtonElement",
        tag: "BUTTON",
        impl {},
        "(&lt;BUTTON /&gt;)"
    )
}
