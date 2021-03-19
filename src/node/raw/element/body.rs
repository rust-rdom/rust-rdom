use std::sync::{Weak};

use crate::sandbox::Sandbox;
use crate::node::raw::{AnyRawNode, AnyRawElement};

/// The [body](https://developer.mozilla.org/en-US/docs/Web/API/Document/body) element type
pub struct BodyElement {
    /// Reference to the sandbox to which this node belongs
    context: Weak<Sandbox>,
}

impl BodyElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        BodyElement { context }
    }
}
impl AnyRawElement for BodyElement {}
impl AnyRawNode for BodyElement {}