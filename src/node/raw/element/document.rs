use std::sync::{Weak};

use crate::sandbox::Sandbox;
use crate::node::raw::{AnyRawNode, AnyRawElement};

/// The [root document element](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement)
/// element type
pub struct DocumentElement {
    /// Reference to the sandbox to which this node belongs
    pub context: Weak<Sandbox>,
}

impl DocumentElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        DocumentElement { context }
    }
}
impl AnyRawElement for DocumentElement {}
impl AnyRawNode for DocumentElement {}