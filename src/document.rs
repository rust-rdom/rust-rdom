use std::boxed::Box;
use std::cell::RefCell;
use std::sync::{Arc, Weak};

use crate::element::{BodyElement, DocumentElement, Element};
use crate::error::DomError;
use crate::sandbox::Sandbox;

pub struct Document {
    sandbox: Weak<Sandbox>,
    //document_element: DocumentElement,
}

impl Document {
    pub(crate) fn new(sandbox: Weak<Sandbox>) -> Self {
        Document {
            sandbox: sandbox.clone(),
            //document_element: DocumentElement::new(sandbox.clone()),
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
