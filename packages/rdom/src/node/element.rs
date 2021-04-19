//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use crate::internal_prelude::*;
use crate::window::Window;

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
