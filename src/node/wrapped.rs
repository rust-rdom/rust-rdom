//! Wrapped representation of a DOM Element. See [node](../index.html) module for distinction from
//! raw representation.

use downcast_rs::DowncastSync;

use std::convert::TryFrom;
use std::result::Result;
use std::sync::{Arc, Weak};

use crate::node::raw;
use crate::sandbox::Sandbox;

/// Any wrapped Element
pub struct Element(Arc<dyn raw::AnyRawElement>);

/// Any wrapped Node
pub struct Node(Arc<dyn raw::AnyRawNode>);

macro_rules! impl_node_base {
    ($ty:ident, $raw_ty:ty) => {
        impl From<$ty> for Node {
            fn from(source: $ty) -> Node {
                Node(source.0)
            }
        }

        impl TryFrom<Node> for $ty {
            type Error = Node;

            fn try_from(elem: Node) -> Result<$ty, Node> {
                elem.0
                    .downcast_arc::<$raw_ty>()
                    .map($ty)
                    .map_err(Node)
            }
        }
    };
}

macro_rules! impl_node {
    ($ty:ident, $raw_ty:ty) => {

        impl_node_base!($ty, $raw_ty);
    };
}

macro_rules! impl_element {
    ($ty:ident, $raw_ty:ty) => {
        impl_node_base!($ty, $raw_ty);

        impl From<$ty> for Element {
            fn from(source: $ty) -> Element {
                Element(source.0)
            }
        }

        impl TryFrom<Element> for $ty {
            type Error = Element;

            fn try_from(elem: Element) -> Result<$ty, Element> {
                elem.0
                    .downcast_arc::<$raw_ty>()
                    .map($ty)
                    .map_err(Element)
            }
        }
    };
}

/// A wrapped Body element
pub struct BodyElement(pub Arc<raw::BodyElement>);
impl_element!(BodyElement, raw::BodyElement);

/// A wrapped Document element (this is like <HTML />)
pub struct DocumentElement(pub Arc<raw::DocumentElement>);
impl_element!(DocumentElement, raw::DocumentElement);

/// A wrapped Document node
pub struct Document(pub Arc<raw::Document>);
impl_node!(Document, raw::Document);
impl Document {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        Document(Arc::new(raw::Document::new(context)))
    }
}