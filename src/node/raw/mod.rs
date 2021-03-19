//! Raw representation of a DOM Element. See [node](../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;

use std::rc::Rc;
use std::sync::{Arc, Weak};

use crate::sandbox::Sandbox;

mod document;
mod element;

pub use document::Document;
pub use element::document::DocumentElement;
pub use element::body::BodyElement;
pub use element::AnyRawElement;

/// The common structure of all DOM nodes
pub struct Node {
    children: Vec<Arc<Node>>,
    parent: Option<Weak<Node>>,
    right_sibling: Option<Weak<Node>>,
    left_sibling: Option<Weak<Node>>,
}

/// A text node
pub struct TextNode {}

/// An input event
pub struct InputEvent {}

/// A button element
pub struct ButtonElement {}

/// A textarea element
pub struct TextAreaElement {}

/// A base trait for all raw node types
pub trait AnyRawNode: DowncastSync {}
impl_downcast!(sync AnyRawNode);