use std::sync::{Weak};

use crate::sandbox::Sandbox;
use crate::node::raw::{AnyRawNode, AnyRawElement};

pub struct DocumentElement {
    context: Weak<Sandbox>,
}

impl DocumentElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        DocumentElement { context }
    }
}
impl AnyRawElement for DocumentElement {}
impl AnyRawNode for DocumentElement {}