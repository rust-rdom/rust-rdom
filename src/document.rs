use std::boxed::Box;
use std::cell::RefCell;
use std::sync::Weak;

use crate::element::{Element, BodyElement, DocumentElement};
use crate::error::DomError;
use crate::sandbox::Sandbox;

pub struct Document {
    sandbox: Weak<Sandbox>,
    document_element: Box<DocumentElement>,
    body: Box<BodyElement>
}

impl Document {
    pub(crate) fn new(sandbox: Weak<Sandbox>) -> Self {
        Document {
            sandbox: sandbox.clone(),
            document_element: Box::new(DocumentElement::new(sandbox.clone())),
            body: Box::new(BodyElement::new(sandbox))
        }
    }

    fn query_selector(&self, selectors: &str) -> Result<Option<Element>, DomError> {
        match selectors {
            "html" => Ok(Some((&*self.document_element).into())),
            "body" => Ok(Some((&*self.body).into())),
            _ => {
                Err(DomError::InvalidQuerySelector)
            }
        }
    }
}