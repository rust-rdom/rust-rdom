//! Raw representation of a DOM element. See [node](../../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;
use paste::paste;
use std::sync::Weak;

use crate::sandbox::Sandbox;

use super::AnyRawNode;

/// A base trait for all raw element types
pub trait AnyRawElement: DowncastSync + AnyRawNode {}
impl_downcast!(sync AnyRawElement);

macro_rules! impl_raw_elements {
    ($(($ty: ty, $blurb: literal, $link: literal $(, $postlude: literal)?))*) => {
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
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
                        $ty { context }
                    }
                }
                impl AnyRawElement for $ty {}
                impl AnyRawNode for $ty {}
            }
        )*
    }
}

impl_raw_elements! {
    (
        HtmlHtmlElement,
        "root document element",
        "Document/documentElement",
        "(<HTML />)"
    )
    (
        HtmlBodyElement,
        "body",
        "Document/body",
        "(<BODY />)"
    )
    (
        HtmlButtonElement,
        "button",
        "HTMLButtonElement",
        "(<BUTTON />)"
    )
}
