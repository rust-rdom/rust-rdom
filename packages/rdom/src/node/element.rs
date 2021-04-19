//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use crate::internal_prelude::*;
use crate::window::Window;

#[derive(Clone)]
/// Enum of all concrete elements
pub enum ElementNodeStorage {
    /// [html](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html)
    HtmlHtmlElement {
        /// reference to window
        default_view: Weak<Window>,
    },
    /// [body](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body)
    HtmlBodyElement,
    /// [button](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
    HtmlButtonElement,
}
