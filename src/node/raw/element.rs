//! Raw representation of a DOM element. See [node](../../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;
use paste::paste;
use std::sync::{Arc, Weak};

use crate::behavior::NodeBehavior;
use crate::node_list::NodeList;
use crate::node::raw::private::PrivateAnyRawNode;
use crate::node::raw::AnyRawNode;
use crate::sandbox::Sandbox;

/// A base trait for all raw element types
pub trait AnyRawElement: DowncastSync + AnyRawNode {}
impl_downcast!(sync AnyRawElement);

macro_rules! impl_raw_elements {
    ($((
        $ty: ty,
        storage: $storage: ty,
        blurb: $blurb: literal,
        link: $link: literal,
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
                impl AnyRawElement for $ty {}
                impl AnyRawNode for $ty {
                    fn get_context(&self) -> Weak<Sandbox> {
                        self.context.clone()
                    }

                    fn clone_node(&self) -> Arc<dyn AnyRawNode> {
                        // TODO this call to clone should really be something
                        // other than the standard clone trait. It is (will be/should be)
                        // our own logic specific to rdom and NOT just a verbatim clone.
                        $ty::new(self.get_context(), self.storage.clone())
                    }

                    fn first_child(&self) -> Option<Arc<dyn AnyRawNode>> {
                        self.node_behavior.first_child()
                    }

                    fn last_child(&self) -> Option<Arc<dyn AnyRawNode>> {
                        self.node_behavior.last_child()
                    }

                    fn append_child(&self, other: Arc<dyn AnyRawNode>) {
                        self.node_behavior.append_child(other)
                    }

                    fn child_nodes(&self) -> Arc<NodeList> {
                        self.node_behavior.child_nodes()
                    }
                }

                impl PrivateAnyRawNode for $ty {
                    fn get_node_behavior(&self) -> Arc<NodeBehavior> {
                        self.node_behavior.clone()
                    }
                }
            }
        )*
    }
}

impl_raw_elements! {
    (
        HtmlHtmlElement,
        storage: (),
        blurb: "root document element",
        link: "Document/documentElement",
        impl {},
        "(&lt;HTML /&gt;)"
    )
    (
        HtmlBodyElement,
        storage: (),
        blurb: "body",
        link: "Document/body",
        impl {},
        "(&lt;BODY /&gt;)"
    )
    (
        HtmlButtonElement,
        storage: (),
        blurb: "button",
        link: "HTMLButtonElement",
        impl {},
        "(&lt;BUTTON /&gt;)"
    )
}
