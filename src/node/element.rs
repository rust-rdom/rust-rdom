//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use paste::paste;
use std::any::Any;

crate::use_behaviors!(node, sandbox_member);
use crate::impl_builder;
use crate::internal_prelude::*;
use crate::sandbox::{Builder, Sandbox};

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
                    /// implementation for SandboxMemberBehavior
                    pub member_storage: SandboxMemberBehaviorStorage,

                    /// implementation for NodeBehavior
                    pub (crate) node_storage: NodeBehaviorStorage,

                    pub(crate) storage: $storage,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>, storage: $storage) -> Arc<$ty> {
                        let construction: Arc<$ty> = Arc::new_cyclic(|construction_weak| -> $ty {
                            $ty {
                                member_storage: SandboxMemberBehaviorStorage::new(context),
                                node_storage: NodeBehaviorStorage::new(construction_weak.clone()),
                                storage
                            }
                        });

                        construction
                    }
                }

                impl_builder!($ty);

                impl_sandbox_member!($ty, member_storage);
                impl_node!($ty, node_storage);

                impl AnyNode for $ty {
                    fn clone_node(&self) -> Arc<dyn AnyNode> {
                        let mut construction = $ty::new(self.get_context(), Default::default());

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
                        (*cons).storage = self.storage.clone();

                        construction
                    }

                    fn get_node_type(&self) -> isize {
                        1 // Element node type is 1
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
