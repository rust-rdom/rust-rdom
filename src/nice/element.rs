//! Nice representation of a DOM Element. See `nice` module for distinction from
//! core representation.

use paste::paste;

use std::convert::TryFrom;
use std::result::Result;

use super::AnyNiceNode;
use crate::internal_prelude::*;
use crate::node_base;

/// A base trait for all nice element types
pub trait AnyNiceElement: AnyNiceNode {}

/// Provides the trait implementations for all nice element types
macro_rules! element_base {
    ($ty: ty, impl { $($rest:tt)* }) => {
        impl AnyNiceElement for $ty {}

        node_base!($ty, impl { $($rest)* });
    }
}

/// A nice Element which could be any concrete type.
pub struct Element(pub Arc<dyn element::AnyElement>);
element_base!(Element, impl {});

macro_rules! impl_nice_elements {
    ($((
        $ty: ty,
        $core_ty: ty,
        $blurb: literal,
        $link: literal,
        impl { $( $rest:tt )* }
        $(, $postlude: literal)?
    ))*) => {
        $(
            paste! {
                #[doc =
                    "A nice ["
                    $blurb
                    "](https://developer.mozilla.org/en-US/docs/Web/API/"
                    $link
                    ") element"
                    $(" " $postlude)?
                ]
                pub struct $ty(pub Arc<$core_ty>);

                element_base!($ty, impl {
                    pub(crate) fn new(context: Weak<$crate::sandbox::Sandbox>) -> Self {
                        // TODO maybe just don't provide constructors in nice elements/nodes?
                        // calling default for someone seems a bit disingenuous, and who says
                        // we can just instantiate any type of node?
                        Self(<$core_ty>::new(context, Default::default()))
                    }
                    $($rest)*
                });

                impl From<$ty> for Element {
                    fn from(source: $ty) -> Element {
                        Element(source.0)
                    }
                }

                impl TryFrom<Element> for $ty {
                    type Error = Element;

                    fn try_from(elem: Element) -> Result<$ty, Element> {
                        elem.0
                            .downcast_arc::<$core_ty>()
                            .map($ty)
                            .map_err(Element)
                    }
                }
            }
        )*
    }
}

impl_nice_elements! {
    (
        HtmlHtmlElement,
        element::HtmlHtmlElement,
        "html",
        "HTMLHtmlElement",
        impl {}
    )
    (
        HtmlBodyElement,
        element::HtmlBodyElement,
        "body",
        "HTMLBodyElement",
        impl {}
    )
    (
        HtmlButtonElement,
        element::HtmlButtonElement,
        "button",
        "HTMLButtonElement",
        impl {}
    )
}
