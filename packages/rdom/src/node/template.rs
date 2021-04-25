//! Mod that includes all about templates

use crate::internal_prelude::*;

/// Template for building nodes from context
pub trait Template<T> {
    /// Performs the build
    fn build(self, context: Arc<Sandbox>) -> T;
}

impl<T, F: Fn(Arc<Sandbox>) -> T> Template<T> for F {
    fn build(self, context: Arc<Sandbox>) -> T {
        (self)(context)
    }
}
