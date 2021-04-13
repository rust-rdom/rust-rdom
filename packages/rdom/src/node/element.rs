//! Core representation of a DOM element. See `nice` module for distinction from
//! nice representation.

use crate::internal_prelude::*;
use crate::sandbox::{Builder, Sandbox};

enum ConcreteElement {
    HtmlHtmlElement,
    HtmlBodyElement,
    HtmlButtonElement
}