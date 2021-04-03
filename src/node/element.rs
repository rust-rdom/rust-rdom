//! Raw representation of a DOM element. See [node](../../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;
use paste::paste;
use std::sync::{Arc, Weak};

use crate::behavior::sandbox_member::{SandboxMemberBehaviour, SandboxMemberBehaviourStorage};
use crate::behavior::NodeBehavior;
use crate::impl_sandbox_member;
use crate::internal_prelude::*;
use crate::node_list::NodeList;
use crate::sandbox::Sandbox;

/// A base trait for all common element types
pub trait AnyElement: DowncastSync + AnyNode {}
impl_downcast!(sync AnyElement);

macro_rules! impl_elements {
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
                    /// implementation for SandboxMemberBehaviour
                    pub member_storage: SandboxMemberBehaviourStorage,

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
                                member_storage: SandboxMemberBehaviourStorage::new(context),
                                node_behavior: Arc::new(NodeBehavior::new(construction_weak.clone())),
                                storage
                            }
                        });

                        construction
                    }
                }

                impl_sandbox_member!($ty, member_storage);

                impl AnyElement for $ty {}
                impl AnyNode for $ty {
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
