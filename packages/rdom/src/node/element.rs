//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use crate::internal_prelude::*;
use crate::window::Window;
use crate::sandbox::Builder;
use super::concrete::ConcreteNodeArc;

macro_rules! declare_elements {
    ($($tag:literal => $name:ident),*) => {
        paste::paste! {
            /// Enum of all concrete elements
            #[derive(Clone)]
            pub enum ElementNodeStorage {
                $(
                    #[doc = "[" $tag "](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag ")"]
                    $name([<$name ElementStorage>]),
                )*
            }
        }
    };
}

declare_elements! {
    "HTML" => HtmlHtml,
    "BODY" => HtmlBody,
    "BUTTON" => HtmlButton
}

/// html element storage
#[derive(Clone)]
pub struct HtmlHtmlElementStorage {
    /// pointer up to the window
    pub default_view: Weak<Window>,
}
/// body element storage
#[derive(Clone)]
pub struct HtmlBodyElementStorage;
/// button element storage
#[derive(Clone)]
pub struct HtmlButtonElementStorage;

impl Builder<ConcreteNodeArc<ElementNodeStorage>> {
    // TODO it would be nice if these didn't all return generic Elements but instead we had some kind of
    // concrete types representing each element type.

    pub fn build_html(&self, default_view: Weak<Window>) -> ConcreteNodeArc<ElementNodeStorage> {
        ConcreteNodeArc::<ElementNodeStorage>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStorage::HtmlHtml(
                HtmlHtmlElementStorage {
                    default_view
                }
            ))
        )
    }

    pub fn build_body(&self, default_view: Weak<Window>) -> ConcreteNodeArc<ElementNodeStorage> {
        ConcreteNodeArc::<ElementNodeStorage>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStorage::HtmlBody(
                HtmlBodyElementStorage
            ))
        )
    }

    pub fn build_button(&self, default_view: Weak<Window>) -> ConcreteNodeArc<ElementNodeStorage> {
        ConcreteNodeArc::<ElementNodeStorage>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStorage::HtmlButton(
                HtmlButtonElementStorage
            ))
        )
    }
}