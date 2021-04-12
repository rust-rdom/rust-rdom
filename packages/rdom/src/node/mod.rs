//! Core representation of a DOM node. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use node::element::AnyElement;
use paste::paste;

use crate::internal_prelude::*;

crate::use_behaviors!(node, sandbox_member);
use crate::window::Window;

use std::{
    fmt,
    sync::{Arc, Weak},
};

mod query_selector;

use crate::sandbox::Builder;
use query_selector::query_selector;

pub mod element;

// I have to abandon this private interface for now - maksimil
// pub(crate) mod private;

/// An input event
pub struct InputEvent {}

#[derive(Copy, Clone, Debug)]
/// Node type, as defined in https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
pub(crate) enum NodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CDataSection = 4,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
}

impl ToString for NodeType {
    fn to_string(&self) -> String {
        match self {
            NodeType::Element => "element",
            NodeType::Attribute => "attribute",
            NodeType::Text => "text",
            NodeType::CDataSection => "CData section",
            NodeType::ProcessingInstruction => "processing instruction",
            NodeType::Comment => "comment",
            NodeType::Document => "document",
            NodeType::DocumentType => "document type",
            NodeType::DocumentFragment => "document fragment",
        }
        .to_string()
    }
}

/// A base trait for all common node types
pub trait AnyNode: DowncastSync + SandboxMemberBehavior + NodeBehavior {
    /// Clones node according to Node.cloneNode()
    fn clone_node(&self) -> Arc<dyn AnyNode>;

    /// Tries to cast node into element
    fn as_element(&self) -> Option<&dyn AnyElement>;

    /// [Document.querySelector](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)
    fn query_selector(&self, selector: &str) -> Result<Option<Arc<dyn AnyNode>>, DomError>;
    /// Returns the node type, as defined in https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
    fn get_node_type(&self) -> isize;
}

impl_downcast!(sync AnyNode);

#[macro_export]
/// implements builder for type
macro_rules! impl_builder {
    ($ty: ident) => {
        impl Builder<$ty> {
            pub fn build(&self) -> Arc<$ty> {
                #[allow(clippy::unit_arg)]
                $ty::new(self.sandbox.clone(), Default::default())
            }
        }
    };
}

impl fmt::Debug for dyn AnyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_element() {
            Some(element) => write!(f, "<{} />", element.tag_name()),
            None => write!(f, "Node of type {}", self.get_node_type()),
        }
    }
}

macro_rules! impl_nodes {
    ($((
        $ty: ty,
        storage: $storage: ty,
        blurb: $blurb: literal,
        link: $link: literal,
        node_type: $node_type: expr,
        impl { $( $rest:tt )* }
        $(, $postlude: literal)?
    ))*) => {
        $(
            paste! {
                #[doc =
                    "The ["
                    $blurb
                    "](https://developer.mozilla.org/en-US/docs/Web/API/"
                    $link
                    ") node type"
                    $(" " $postlude)?
                ]
                #[derive(Debug)]
                pub struct $ty {
                    /// implementation for SandboxMemberBehavior
                    pub member_storage: SandboxMemberBehaviorStorage,

                    /// implementation for NodeBehavior
                    pub(crate) node_storage: NodeBehaviorStorage,

                    pub(crate) storage: $storage,

                    node_type: NodeType,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>, storage: $storage) -> Arc<$ty> {
                        let construction: Arc<$ty> = Arc::new_cyclic(|construction_weak| -> $ty {
                            $ty {
                                storage,
                                node_storage: NodeBehaviorStorage::new(construction_weak.clone()),
                                member_storage: SandboxMemberBehaviorStorage::new(context),
                                node_type: $node_type,
                            }
                        });

                        construction
                    }

                    $($rest)*
                }

                impl_builder!($ty);

                impl_sandbox_member!($ty, member_storage);
                impl_node!($ty, node_storage);

                impl AnyNode for $ty {
                    fn clone_node(&self) -> Arc<dyn AnyNode> {
                        let mut construction = $ty::new(self.get_context(), Default::default());

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
                        (*cons).storage = self.storage.clone();

                        construction
                    }

                    fn as_element(&self) -> Option<&dyn AnyElement> {
                        None
                    }

                    fn query_selector(&self, selector: &str) -> Result<Option<Arc<dyn AnyNode>>, DomError> {
                        query_selector(self, selector)
                    }

                    fn get_node_type(&self) -> isize {
                        self.node_type as isize
                    }
                }
            }
        )*
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct DocumentNodeStorage {
    /// Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

#[derive(Default, Clone, Debug)]
pub(crate) struct TextNodeStorage {
    /// Text in the text node
    pub(crate) text: String,
}

impl_nodes! {
    (
        ElementNode,
        storage: (),
        blurb: "Element",
        link: "Element",
        node_type: NodeType::Element,
        impl {}
    )
    (
        AttrNode,
        storage: (),
        blurb: "attr (attribute)",
        link: "Attr",
        node_type: NodeType::Attribute,
        impl {}
    )
    (
        TextNode,
        storage: TextNodeStorage,
        blurb: "text",
        link: "Text",
        node_type: NodeType::Text,
        impl {
            /// Creates a text node.
            pub fn get_text(&self) -> Option<String> {
                Some(self.storage.text.clone())
            }
        }
    )
    (
        CDataSectionNode,
        storage: (),
        blurb: "CDATASection",
        link: "CDATASection",
        node_type: NodeType::CDataSection,
        impl {}
    )
    (
        ProcessingInstructionNode,
        storage: () /* or ProcessingInstructiNodeStorage */,
        blurb: "ProcessingInstruction",
        link: "ProcessingInstruction",
        node_type: NodeType::ProcessingInstruction,
        impl {}
    )
    (
        CommentNode,
        storage: TextNodeStorage,
        blurb: "Comment",
        link: "Comment",
        node_type: NodeType::Comment,
        impl {}
    )
    (
        DocumentNode,
        storage: DocumentNodeStorage,
        blurb: "document",
        link: "Document",
        node_type: NodeType::Document,
        impl {
            /// Creates a text node.
            pub fn create_text_node(&self, text: String) -> Arc<TextNode> {
                TextNode::new(self.get_context(), TextNodeStorage { text })
            }
        }
    )
    (
        DocumentTypeNode,
        storage: () /* or DocumentTypeNodeStorage */,
        blurb: "DocumentType",
        link: "DocumentType",
        node_type: NodeType::DocumentType,
        impl {}
    )
    (
        DocumentFragmentNode,
        storage: () /* or DocumentFragmentNodeStorage */,
        blurb: "DocumentFragment",
        link: "DocumentFragment",
        node_type: NodeType::DocumentFragment,
        impl {}
    )
}
