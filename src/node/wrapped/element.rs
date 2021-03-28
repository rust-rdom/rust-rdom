use paste::paste;

use std::convert::TryFrom;
use std::result::Result;
use std::sync::{Arc, Weak};

use crate::node_base;
use crate::sandbox::Sandbox;
use crate::node::raw::{self as raw_node, element as raw_element, AnyRawNode};
use super::AnyWrappedNode;

/// A base trait for all wrapped element types
pub trait AnyWrappedElement: AnyWrappedNode {}

/// Provides the trait implementations for all wrapped element types
macro_rules! element_base {
    ($ty: ty, impl { $($rest:tt)* }) => {
        impl AnyWrappedElement for $ty {}

        node_base!($ty, impl { $($rest)* });
    }
}

/// A wrapped Element which could be any concrete type.
pub struct Element(pub Arc<dyn raw_element::AnyRawElement>);
element_base!(Element, impl {});

macro_rules! impl_wrapped_elements {
    ($((
        $ty: ty,
        $raw_ty: ty,
        $blurb: literal,
        $link: literal,
        impl { $( $rest:tt )* }
        $(, $postlude: literal)?
    ))*) => {
        $(
            paste! {
                #[doc =
                    "A wrapped ["
                    $blurb
                    "](https://developer.mozilla.org/en-US/docs/Web/API/"
                    $link
                    ") element"
                    $(" " $postlude)?
                ]
                pub struct $ty(pub Arc<$raw_ty>);

                element_base!($ty, impl {        
                    pub(crate) fn new(context: Weak<$crate::sandbox::Sandbox>) -> Self {
                        Self(Arc::new(<$raw_ty>::new(context)))
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
                            .downcast_arc::<$raw_ty>()
                            .map($ty)
                            .map_err(Element)
                    }
                }
            }
        )*
    }
}

impl_wrapped_elements! {
    (
        HtmlHtmlElement,
        raw_element::HtmlHtmlElement,
        "html",
        "HTMLHtmlElement",
        impl {}
    )
    (
        HtmlBodyElement,
        raw_element::HtmlBodyElement,
        "body",
        "HTMLBodyElement",
        impl {}
    )
    (
        HtmlButtonElement,
        raw_element::HtmlButtonElement,
        "button",
        "HTMLButtonElement",
        impl {}
    )
}