use downcast_rs::DowncastSync;

use std::rc::Rc;
use std::sync::{Arc, Weak};

use crate::sandbox::Sandbox;

mod document;
mod element;

pub use document::Document;
pub use element::document::DocumentElement;
pub use element::body::BodyElement;
pub use element::AnyElement;

pub struct Node {
    children: Vec<Arc<Node>>,
    parent: Option<Weak<Node>>,
    right_sibling: Option<Weak<Node>>,
    left_sibling: Option<Weak<Node>>,
}

pub struct TextNode {}

pub struct InputEvent {}

pub struct ButtonElement {}

pub struct TextAreaElement {}

// #[derive(Debug)]
// pub struct InputElement {}
// impl AnyElement for InputElement {}
// impl AnyNode for InputElement {}


pub trait AnyNode: DowncastSync {}
impl_downcast!(sync AnyNode);