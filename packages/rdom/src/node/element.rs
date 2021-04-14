//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use crate::internal_prelude::*;
use crate::sandbox::{Sandbox};
use crate::window::Window;

#[derive(Clone)]
pub enum ConcreteElement {
    HtmlHtmlElement {
        default_view: Weak<Window>
    },
    HtmlBodyElement,
    HtmlButtonElement
}