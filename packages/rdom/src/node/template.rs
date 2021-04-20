//! Mod that includes all about templates

use super::{concrete::ElementNodeArc, element::HtmlHtmlStore};
use crate::internal_prelude::*;

/// Template for building nodes from context
pub trait Template<T> {
    /// Performs the build
    fn build(self, context: Weak<Sandbox>) -> T;
}

impl<T, F: Fn(Weak<Sandbox>) -> T> Template<T> for F {
    fn build(self, context: Weak<Sandbox>) -> T {
        (self)(context)
    }
}
