//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use paste::paste;

crate::use_behaviors!(node, sandbox_member);
use crate::internal_prelude::*;
use crate::sandbox::Sandbox;

use super::query_selector::query_selector;

/// A base trait for all core element types
pub trait AnyElement: DowncastSync + AnyNode {
    /// Gets html tag (DIV for <div> or BUTTON for <button>)
    /// [mdn docs](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
    fn tag_name(&self) -> String;
}
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

                impl_sandbox_member!($ty, member_storage);
                impl_node!($ty, node_storage);

                impl AnyNode for $ty {
                    fn clone_node(&self) -> Arc<dyn AnyNode> {
                        let mut construction = $ty::new(self.get_context(), Default::default());

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
                        (*cons).storage = self.storage.clone();

                        construction
                    }

                    fn as_element(&self) -> Option<&dyn AnyElement> {
                        Some(self)
                    }

                    fn query_selector(&self, selector: &str) -> Result<Option<Arc<dyn AnyNode>>, DomError> {
                        query_selector(self, selector)
                    }
                }

                impl AnyElement for $ty {
                    fn tag_name(&self) -> String {
                        $tag.to_string()
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
