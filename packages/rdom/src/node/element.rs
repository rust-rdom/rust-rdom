//! Data and functionality to each element type live here.

use super::{concrete::ConcreteNodeArc, template::Template};
use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::window::Window;

macro_rules! declare_elements {
    ($($tag:literal => $name:ident),*) => {
        paste::paste! {
        /// Enum of all concrete elements
        #[derive(Clone)]
        pub enum ElementStore {
            $(
                #[doc = "[" $tag "](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag ")"]
                $name([<$name Store>]),
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
pub struct HtmlHtmlStore {
    /// pointer up to the window
    pub default_view: Weak<Window>,
}
/// body element storage
#[derive(Clone)]
pub struct HtmlBodyStore;
/// button element storage
#[derive(Clone)]
pub struct HtmlButtonStore;

/// Template for html
pub struct HtmlHtmlTemplate;

impl Template<ElementNodeArc> for HtmlHtmlTemplate {
    fn build(self, context: Arc<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            Arc::downgrade(&context),
            Arc::new(ElementStore::HtmlHtml(HtmlHtmlStore {
                default_view: Arc::downgrade(&context.window()),
            })),
        )
    }
}

/// Template for body
pub struct HtmlBodyTemplate;

impl Template<ElementNodeArc> for HtmlBodyTemplate {
    fn build(self, context: Arc<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            Arc::downgrade(&context),
            Arc::new(ElementStore::HtmlBody(HtmlBodyStore)),
        )
    }
}

/// Template for button
pub struct HtmlButtonTemplate;

impl Template<ElementNodeArc> for HtmlButtonTemplate {
    fn build(self, context: Arc<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            Arc::downgrade(&context),
            Arc::new(ElementStore::HtmlButton(HtmlButtonStore)),
        )
    }
}
