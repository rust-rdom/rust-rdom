use std::cell::RefCell;
use std::sync::Weak;

use crate::sandbox::Sandbox;

pub struct DocumentElement {
    context: Weak<Sandbox>,
}

impl DocumentElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        DocumentElement {
            context,
        }
    }
}

pub struct BodyElement {
    context: Weak<Sandbox>,
}

impl BodyElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        BodyElement {
            context,
        }
    }
}

pub struct Element {

}

impl From<&DocumentElement> for Element {
    fn from(_: &DocumentElement) -> Element {
        // TODO!
        Element {}
    }
}

impl From<&BodyElement> for Element {
    fn from(_: &BodyElement) -> Element {
        // TODO!
        Element {}
    }
}

pub struct Node {
    
}

pub struct TextNode {
    
}

pub struct InputEvent {
    
}

pub struct InputElement {
    
}

pub struct ButtonElement {
    
}

pub struct TextAreaElement {
    
}