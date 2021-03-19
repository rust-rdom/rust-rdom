use std::sync::Weak;

use crate::error::DomError;
use crate::node::Element;
use crate::node::raw::AnyRawNode;
use crate::sandbox::Sandbox;

/// The root [Document](https://developer.mozilla.org/en-US/docs/Web/API/Document) node type
pub struct Document {
    /// Reference to the sandbox to which this node belongs
    pub sandbox: Weak<Sandbox>,
}
impl AnyRawNode for Document {}

impl Document {
    pub(crate) fn new(sandbox: Weak<Sandbox>) -> Self {
        Document {
            // Since Document is the root node, it contains a link to the Sandbox
            sandbox: sandbox.clone(),
        }
    }

    fn query_selector(&self, selectors: &str) -> Result<Option<Element>, DomError> {
        match selectors {
            //"html" => {
            //    Ok(Some(self.document_element.into()))
            //},
            //"body" => Ok(Some((&*self.body).into())),
            _ => Err(DomError::InvalidQuerySelector),
        }
    }
}