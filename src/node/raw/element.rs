//! Raw representation of a DOM element. See [node](../../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;
use paste::paste;
use std::sync::{Arc, Weak};

use crate::behavior::NodeBehavior;
use crate::node::raw::AnyRawNode;
use crate::sandbox::Sandbox;

/// A base trait for all raw element types
pub trait AnyRawElement: DowncastSync + AnyRawNode {}
impl_downcast!(sync AnyRawElement);

macro_rules! impl_raw_elements {
    ($((
        $ty: ty,
        $storage: ty,
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
                    ") element type"
                    $(" " $postlude)?
                ]
                pub struct $ty {
                    /// Reference to the sandbox to which this element belongs
                    pub context: Weak<Sandbox>,

                    /// Node behavior (fields/methods associated with the DOM class called Node)
                    pub node_behavior: Option<Arc<NodeBehavior>>,

                    pub(crate) storage: $storage,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<$ty> {
                        let mut construction = Arc::new($ty {
                            context,
                            node_behavior: None,
                            storage: Default::default()
                        });

                        let construction_weak = Arc::downgrade(&construction);

                        let node_behavior = Arc::new(NodeBehavior::new(construction_weak.clone()));

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct element");
                        (*cons).node_behavior = Some(node_behavior);

                        construction
                    }
                }
                impl AnyRawElement for $ty {}
                impl AnyRawNode for $ty {
                    fn get_context(&self) -> Weak<Sandbox> {
                        self.context.clone()
                    }

                    fn clone_node(&self) -> Arc<dyn AnyRawNode> {
                        let mut construction = $ty::new(self.get_context());

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
                        (*cons).storage = self.storage.clone();

                        construction
                    }
                }
            }
        )*
    }
}

impl_raw_elements! {
    (
        HtmlHtmlElement,
        (),
        "root document element",
        "Document/documentElement",
        impl {},
        "(&lt;HTML /&gt;)"
    )
    (
        HtmlBodyElement,
        (),
        "body",
        "Document/body",
        impl {},
        "(&lt;BODY /&gt;)"
    )
    (
        HtmlButtonElement,
        (),
        "button",
        "HTMLButtonElement",
        impl {},
        "(&lt;BUTTON /&gt;)"
    )
}
