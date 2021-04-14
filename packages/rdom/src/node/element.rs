//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use crate::internal_prelude::*;
use crate::sandbox::{Sandbox};

#[derive(Clone)]
pub enum ConcreteElement {
    HtmlHtmlElement,
    HtmlBodyElement,
    HtmlButtonElement
}