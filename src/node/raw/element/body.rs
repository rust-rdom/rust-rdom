use std::sync::{Weak};

use crate::sandbox::Sandbox;
use crate::node::raw::{AnyNode, AnyElement};

pub struct BodyElement {
    context: Weak<Sandbox>,
}

impl BodyElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        BodyElement { context }
    }
}
impl AnyElement for BodyElement {}
impl AnyNode for BodyElement {}