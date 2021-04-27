//! Data and functionality to each element type live here.

use super::template::TemplateWeak;
use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::window::Window;

macro_rules! declare_html_elements {
    ($($tag:literal => $name:ident),*) => {
        paste::paste! {
        /// Enum of all HTMLElements
        #[derive(Clone)]
        pub enum HtmlElementStore {
            $(
                #[doc = "[" $tag "](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag ")"]
                $name([<$name Store>]),
            )*
            /// Represents an invalid HTML element
            HtmlUnknown(HtmlUnknownStore)
        }

        impl HtmlElementStore {
            /// [Element.tagName](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
            pub fn tag_name(&self) -> String {
                match self {
                    $(
                        HtmlElementStore::$name(_) => $tag.to_string(),
                    )*
                    HtmlElementStore::HtmlUnknown(store) => store.tag_name.clone(),
                }
            }
        }

        $(
            impl TemplateWeak<ElementNodeArc> for [<$name Store>] {
                fn build(self, context: Weak<Sandbox>) -> ElementNodeArc {
                    ElementNodeArc::new(
                        context,
                        Arc::new(ElementStore::HtmlElement(HtmlElementStore::$name(self))))
                }
            }
        )*
    }
    };
}

/// Enum of all SVGElements
#[derive(Clone)]
pub enum SvgElementStore {}

/// Enum of all concrete elements
#[derive(Clone)]
pub enum ElementStore {
    /// Enum variant for an HTMLElement
    HtmlElement(HtmlElementStore),

    /// Enum variant for an SVGElement
    SvgElement(SvgElementStore),
}

impl ElementStore {
    /// [Element.tagName](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
    pub fn tag_name(&self) -> String {
        match self {
            ElementStore::HtmlElement(el) => el.tag_name(),
            ElementStore::SvgElement(_) => {
                unimplemented!()
            }
        }
    }
}

declare_html_elements! {
    "HTML" => HtmlHtml,
    "BODY" => HtmlBody,
    "BUTTON" => HtmlButton
}

/// html element storage
#[derive(Clone)]
pub struct HtmlHtmlStore;
/// html unknown element storage
#[derive(Clone)]
pub struct HtmlUnknownStore {
    pub(crate) tag_name: String,
}
/// body element storage
#[derive(Clone)]
pub struct HtmlBodyStore;
/// button element storage
#[derive(Clone)]
pub struct HtmlButtonStore;
