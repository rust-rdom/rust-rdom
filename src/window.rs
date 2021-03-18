use std::cell::RefCell;
use std::sync::Weak;

use crate::sandbox::Sandbox;

/// A simulated window for static rendering
pub struct Window {
    context: Weak<Sandbox>,
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        Window { context }
    }
}
