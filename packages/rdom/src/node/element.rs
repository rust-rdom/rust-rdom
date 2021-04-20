//! Data and functionality to each element type live here.

use super::concrete::ConcreteNodeArc;
use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::sandbox::Builder;
use crate::window::Window;

macro_rules! declare_elements {
    ($($tag:literal => $name:ident),*) => {
        paste::paste! {
            /// Enum of all concrete element storages
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

impl Builder<ElementNodeArc> {
    // TODO it would be nice if these didn't all return generic Elements but instead we had some kind of
    // concrete types representing each element type.

    /// Builds a new HtmlHtmlElement node with a weak reference to its corresponding window
    pub fn build_html(&self, default_view: Weak<Window>) -> ConcreteNodeArc<ElementNodeStorage> {
        ConcreteNodeArc::<ElementNodeStorage>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStorage::HtmlHtml(HtmlHtmlElementStorage {
                default_view,
            })),
        )
    }

    /// Builds a new HtmlBodyElement node
    pub fn build_body(&self) -> ConcreteNodeArc<ElementNodeStorage> {
        ConcreteNodeArc::<ElementNodeStorage>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStorage::HtmlBody(HtmlBodyElementStorage)),
        )
    }

    /// Builds a new HtmlButtonElement node
    pub fn build_button(&self) -> ConcreteNodeArc<ElementNodeStorage> {
        ConcreteNodeArc::<ElementNodeStorage>::new(
            self.sandbox.clone(),
            Arc::new(ElementNodeStorage::HtmlButton(HtmlButtonElementStorage)),
        )
    }
}
