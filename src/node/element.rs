//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use paste::paste;

use crate::behavior::node::{NodeBehaviour, NodeBehaviourStorage};
use crate::behavior::sandbox_member::{SandboxMemberBehaviour, SandboxMemberBehaviourStorage};
use crate::internal_prelude::*;
use crate::sandbox::Sandbox;
use crate::{impl_node, impl_sandbox_member};

/// A base trait for all core element types
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

                    /// implementation for NodeBehaviour
                    pub (crate) node_storage: NodeBehaviourStorage,

                    pub(crate) storage: $storage,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>, storage: $storage) -> Arc<$ty> {
                        let construction: Arc<$ty> = Arc::new_cyclic(|construction_weak| -> $ty {
                            $ty {
                                member_storage: SandboxMemberBehaviourStorage::new(context),
                                node_storage: NodeBehaviourStorage::new(construction_weak.clone()),
                                storage
                            }
                        });

                        construction
                    }
                }

                impl_sandbox_member!($ty, member_storage);
                impl_node!($ty, node_storage);

                impl AnyNode for $ty {
                    fn clone_node(&self) -> Arc<dyn AnyNode> {
                        let mut construction = $ty::new(self.get_context(), Default::default());

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
                        (*cons).storage = self.storage.clone();

                        construction
                    }
                }

                impl AnyElement for $ty {}
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
