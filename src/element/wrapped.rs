use downcast_rs::DowncastSync;

// use std::rc::{Rc, Weak};
use std::convert::TryFrom;
use std::result::Result;
use std::sync::{Arc, Weak};

use crate::element::raw;
use crate::sandbox::Sandbox;

pub struct Element(Arc<dyn raw::AnyElement>);
pub struct BodyElement(Arc<raw::BodyElement>);
pub struct DocumentElement(Arc<raw::DocumentElement>);

impl DocumentElement {
    // pub(crate) fn new(context: Weak<Sandbox>) -> Self {
    //     DocumentElement(Rc::new(raw::DocumentElement::new(context)))
    // }
}

impl From<DocumentElement> for Element {
    fn from(doc: DocumentElement) -> Element {
        Element(doc.0)
    }
}

impl TryFrom<Element> for DocumentElement {
    type Error = Element;

    fn try_from(elem: Element) -> Result<DocumentElement, Element> {
        elem.0
            .downcast_arc::<raw::DocumentElement>()
            .map(DocumentElement)
            .map_err(Element)
    }
}
