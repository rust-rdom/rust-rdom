use downcast_rs::DowncastSync;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Weak};

use crate::sandbox::Sandbox;

pub struct BodyElement {
    context: Weak<Sandbox>,
}

impl BodyElement {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        BodyElement { context }
    }
}

//impl From<&DocumentElement> for &Element {
//    fn from(_: &DocumentElement) -> &Element {
//        // TODO!
//        &Element {}
//    }
//}
//
// impl From<&BodyElement> for Element {
//     fn from(_: &BodyElement) -> Element {
//         // TODO!
//         Element {}
//     }
// }

pub struct Node {
    children: Vec<Arc<Node>>,
    right_sibling: Option<Weak<Node>>,
    left_sibling: Option<Weak<Node>>,
}

pub struct TextNode {}

pub struct InputEvent {}

pub struct ButtonElement {}

pub struct TextAreaElement {}

// pub struct Element(Box<dyn AnyElement>);

pub trait AnyNode: DowncastSync {}
impl_downcast!(sync AnyNode);

pub trait AnyElement: DowncastSync + AnyNode {}
impl_downcast!(sync AnyElement);

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

// Concrete types implementing Base.
#[derive(Debug)]
pub struct InputElement {}
impl AnyElement for InputElement {}
impl AnyNode for InputElement {}

// Concrete types implementing Base.
#[derive(Debug)]
pub struct Element {}
impl AnyElement for Element {}
impl AnyNode for Element {}
