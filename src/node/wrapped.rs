use downcast_rs::DowncastSync;

use std::convert::TryFrom;
use std::result::Result;
use std::sync::{Arc, Weak};

use crate::node::raw;
use crate::sandbox::Sandbox;

pub struct Element(Arc<dyn raw::AnyRawElement>);
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
        pub struct $ty(Arc<$raw_ty>);

        impl_node_base!($ty, $raw_ty);
    };
}

macro_rules! impl_element {
    ($ty:ident, $raw_ty:ty) => {
        impl_node_base!($ty, $raw_ty);

        pub struct $ty(Arc<$raw_ty>);

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

impl_element!(BodyElement, raw::BodyElement);

impl_element!(DocumentElement, raw::DocumentElement);

impl_node!(Document, raw::Document);
impl Document {
    pub(crate) fn new(context: Weak<Sandbox>) -> Self {
        Document(Arc::new(raw::Document::new(context)))
    }
}