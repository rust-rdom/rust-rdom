//! Core representation of a DOM node. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use paste::paste;

use crate::internal_prelude::*;

crate::use_behaviors!(node, sandbox_member);
use crate::sandbox::Builder;
use crate::window::Window;

use std::sync::{Arc, Weak};

pub mod element;

/// An input event
pub struct InputEvent {}

#[derive(Copy, Clone)]
/// Node type, as defined in https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
pub(crate) enum NodeContents {
    Element(ConcreteElement),
    Attribute,
    Text,
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
}

pub struct Node {
    /// specifics on what type of node this is
    pub contents: NodeContents,

    /// implementation for SandboxMemberBehavior
    pub member_storage: SandboxMemberBehaviorStorage,

    /// implementation for NodeBehavior
    pub(crate) node_storage: NodeBehaviorStorage,
}

impl Node {
    pub(crate) fn new(context: Weak<Sandbox>, contents: NodeContents) -> Arc<Node> {
        let construction: Arc<Node> = Arc::new_cyclic(|construction_weak| -> Node {
            Node {
                contents,
                node_storage: NodeBehaviorStorage::new(construction_weak.clone()),
                member_storage: SandboxMemberBehaviorStorage::new(context),
            }
        });

        construction
    }

    fn first_child(&self) -> Option<Arc<dyn AnyNode>> {
        self.node_behavior.first_child()
    }

    fn last_child(&self) -> Option<Arc<dyn AnyNode>> {
        self.node_behavior.last_child()
    }

    fn append_child(&self, other: Arc<dyn AnyNode>) {
        self.node_behavior.append_child(other)
    }

    fn static_child_nodes(&self) -> Vec<Arc<dyn AnyNode>> {
        self.node_behavior.static_child_nodes()
    }

    fn child_nodes(&self) -> Arc<crate::node_list::NodeList> {
        self.node_behavior.child_nodes()
    }

    fn clone_node(&self) -> Arc<dyn AnyNode> {
        let contents = self.contents.clone();
        let mut construction = Node::new(self.get_context(), contents);

        construction
    }

    fn get_node_type(&self) -> isize {
        match self.contents {
            Element(_) => 1,
            Attribute => 2,
            Text => 3,
            CDataSection => 4,
            ProcessingInstruction => 7,
            Comment => 8,
            Document => 9,
            DocumentType => 10,
            DocumentFragment => 11,
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct DocumentNodeStorage {
    /// Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

#[derive(Default, Clone)]
pub(crate) struct TextNodeStorage {
    /// Text in the text node
    pub(crate) text: String,
}

/*

// TODO for TextNode. this will require a "nice" version
/// Creates a text node.
pub fn get_text(&self) -> Option<String> {
    Some(self.storage.text.clone())
}            

// TODO for DocumentNode; this will require a "nice" instantiation
/// Creates a text node.
pub fn create_text_node(&self, text: String) -> Arc<TextNode> {
    TextNode::new(self.get_context(), TextNodeStorage { text })
}

*/