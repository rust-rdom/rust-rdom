use std::sync::{Weak};

use crate::sandbox::Sandbox;
use crate::node::raw::{AnyNode, AnyElement};

pub struct DocumentElement {
    context: Weak<Sandbox>,
}

impl DocumentElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        DocumentElement { context }
    }
}
impl AnyElement for DocumentElement {}
impl AnyNode for DocumentElement {}