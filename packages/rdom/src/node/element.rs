//! Data and functionality to each element type live here.

use super::template::TemplateWeak;
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

        $(
            impl TemplateWeak<ElementNodeArc> for [<$name Store>] {
                fn build(self, context: Weak<Sandbox>) -> ElementNodeArc {
                    ElementNodeArc::new(
                        context,
                        Arc::new(ElementStore::$name(self)))
                }
            }
        )*
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
