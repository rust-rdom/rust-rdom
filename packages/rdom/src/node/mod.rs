//! Core representation of a DOM node. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use paste::paste;

use crate::{internal_prelude::*, node_list::Query};
use crate::node_list::{NodeList, NodeListStorage};
use std::sync::{Arc, RwLock, Weak};

crate::use_behaviors!(sandbox_member);
use crate::window::Window;

use element::ConcreteElement;

pub mod element;

/// An input event
pub struct InputEvent {}

/// Node type, as defined in https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
#[derive(Clone)]
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

    pub node_storage: NodeBehavior,

    /// implementation for SandboxMemberBehavior
    pub member_storage: SandboxMemberBehaviorStorage,
}

impl Node {
    pub(crate) fn new(context: Weak<Sandbox>, contents: NodeContents) -> Arc<Node> {
        let construction: Arc<Node> = Arc::new_cyclic(|construction_weak| -> Node {
            Node {
                contents,
                node_storage: NodeBehavior::new(construction_weak.clone()),
                member_storage: SandboxMemberBehaviorStorage::new(context),
            }
        });

        construction
    }

    fn first_child(&self) -> Option<Arc<Node>> {
        self.node_storage.first_child()
    }

    fn last_child(&self) -> Option<Arc<Node>> {
        self.node_storage.last_child()
    }

    fn append_child(&self, other: Arc<Node>) {
        self.node_storage.append_child(other)
    }

    pub(crate) fn static_child_nodes(&self) -> Vec<Arc<Node>> {
        self.node_storage.static_child_nodes()
    }

    fn child_nodes(&self) -> Arc<NodeList> {
        self.node_storage.child_nodes()
    }

    fn clone_node(&self) -> Arc<Node> {
        let contents = self.contents.clone();
        let mut construction = Node::new(self.get_context(), contents);

        construction
    }

    fn get_node_type(&self) -> isize {
        match self.contents {
            NodeContents::Element(_) => 1,
            NodeContents::Attribute => 2,
            NodeContents::Text => 3,
            NodeContents::CDataSection => 4,
            NodeContents::ProcessingInstruction => 7,
            NodeContents::Comment => 8,
            NodeContents::Document => 9,
            NodeContents::DocumentType => 10,
            NodeContents::DocumentFragment => 11,
        }
    }
}

impl_sandbox_member!(Node, member_storage);

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

pub struct NodeBehavior {
    /// Reference back up to the common Node
    node: Weak<Node>,

    parent_node: Option<Weak<Node>>,
    left_sibling: Option<Weak<Node>>,
    right_sibling: Option<Weak<Node>>,
    child_nodes: RwLock<Vec<Arc<Node>>>,
}

impl NodeBehavior {
    pub fn new(node: Weak<Node>) -> NodeBehavior {
        NodeBehavior {
            node,
            parent_node: None,
            left_sibling: None,
            right_sibling: None,
            child_nodes: RwLock::new(Vec::new()),
        }
    }

    fn first_child(&self) -> Option<Arc<Node>> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).first().cloned()
    }

    fn last_child(&self) -> Option<Arc<Node>> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).last().cloned()
    }

    fn append_child(&self, other: Arc<Node>) {
        let mut lock = self.child_nodes.write().unwrap();
        (*lock).push(other);
    }

    fn static_child_nodes(&self) -> Vec<Arc<Node>> {
        self.child_nodes.read().unwrap().clone()
    }

    fn child_nodes(&self) -> Arc<NodeList> {
        let strong_ref = self.node.upgrade().expect("Sandbox dropped");

        NodeList::new(
            (*strong_ref).get_context(),
            NodeListStorage::Live(Query::ChildNodes {
                children_of: strong_ref,
            }),
        )
    }
}