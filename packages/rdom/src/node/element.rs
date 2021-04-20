//! Data and functionality to each element type live here.

use super::concrete::ConcreteNodeArc;
use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::sandbox::Builder;
use crate::window::Window;

macro_rules! declare_elements {
    ($($tag:literal => $name:ident),*) => {
        paste::paste! {
        /// Enum of all concrete elements
        #[derive(Clone)]
        pub enum ElementStore {
            $(
                #[doc = "[" $tag "](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag ")"]
                $name([<$name ES>]),
            )*
        }

        impl ElementStore {
            /// [Element.tagName](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
            pub fn tag_name(&self) -> String {
                match self {
                    $(
                        ElementStore::$name(_) => $tag.to_string(),
                    )*
                }
            }
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
pub struct HtmlHtmlES {
    /// pointer up to the window
    pub default_view: Weak<Window>,
}
/// body element storage
#[derive(Clone)]
pub struct HtmlBodyES;
/// button element storage
#[derive(Clone)]
pub struct HtmlButtonES;

impl Builder<ElementNodeArc> {
    // TODO it would be nice if these didn't all return generic Elements but instead we had some kind of
    // concrete types representing each element type.

    /// Builds a new HtmlHtmlElement node with a weak reference to its corresponding window
    pub fn build_html(&self, default_view: Weak<Window>) -> ConcreteNodeArc<ElementStore> {
        ConcreteNodeArc::<ElementStore>::new(
            self.sandbox.clone(),
            Arc::new(ElementStore::HtmlHtml(HtmlHtmlES { default_view })),
        )
    }

    /// Builds a new HtmlBodyElement node
    pub fn build_body(&self) -> ConcreteNodeArc<ElementStore> {
        ConcreteNodeArc::<ElementStore>::new(
            self.sandbox.clone(),
            Arc::new(ElementStore::HtmlBody(HtmlBodyES)),
        )
    }

    /// Builds a new HtmlButtonElement node
    pub fn build_button(&self) -> ConcreteNodeArc<ElementStore> {
        ConcreteNodeArc::<ElementStore>::new(
            self.sandbox.clone(),
            Arc::new(ElementStore::HtmlButton(HtmlButtonES)),
        )
    }
}
