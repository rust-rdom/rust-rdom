use std::sync::{Weak};

use crate::sandbox::Sandbox;
use crate::node::raw::{AnyRawNode, AnyRawElement};

pub struct BodyElement {
    context: Weak<Sandbox>,
}

impl BodyElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        BodyElement { context }
    }
}
impl AnyRawElement for BodyElement {}
impl AnyRawNode for BodyElement {}