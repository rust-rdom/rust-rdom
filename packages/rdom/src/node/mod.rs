//! Core representation of a DOM node. See `nice` module for distinction from
//! nice representation.

use crate::node_list::{NodeList, NodeListStorage};
use crate::{internal_prelude::*, node_list::Query};
use std::sync::RwLock;

crate::use_behaviors!(sandbox_member);
use crate::window::Window;

use element::ConcreteElement;

pub mod element;

/// An input event
pub struct InputEvent {}

/// Node type, as defined in https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
#[derive(Clone)]
pub(crate) enum NodeContentsArc {
    Element(Arc<ConcreteElement>),
    Attribute,
    Text,
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
}

#[derive(Clone)]
pub(crate) enum NodeContentsWeak {
    Element(Weak<ConcreteElement>),
    Attribute,
    Text,
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
}

impl NodeContentsArc {
    fn to_isize(&self) -> isize {
        match self {
            NodeContentsArc::Element(_) => 1,
            NodeContentsArc::Attribute => 2,
            NodeContentsArc::Text => 3,
            NodeContentsArc::CDataSection => 4,
            NodeContentsArc::ProcessingInstruction => 5,
            NodeContentsArc::Comment => 6,
            NodeContentsArc::Document => 7,
            NodeContentsArc::DocumentType => 8,
            NodeContentsArc::DocumentFragment => 9,
        }
    }

    fn downgrade(&self) -> NodeContentsWeak {
        match self {
            NodeContentsArc::Element(strong) => NodeContentsWeak::Element(Arc::downgrade(&strong)),
            NodeContentsArc::Attribute => NodeContentsWeak::Attribute,
            NodeContentsArc::Text => NodeContentsWeak::Text,
            NodeContentsArc::CDataSection => NodeContentsWeak::CDataSection,
            NodeContentsArc::ProcessingInstruction => NodeContentsWeak::ProcessingInstruction,
            NodeContentsArc::Comment => NodeContentsWeak::Comment,
            NodeContentsArc::Document => NodeContentsWeak::Document,
            NodeContentsArc::DocumentType => NodeContentsWeak::DocumentType,
            NodeContentsArc::DocumentFragment => NodeContentsWeak::DocumentFragment,
        }
    }
}

impl NodeContentsWeak {
    fn to_isize(&self) -> isize {
        match self {
            NodeContentsWeak::Element(_) => 1,
            NodeContentsWeak::Attribute => 2,
            NodeContentsWeak::Text => 3,
            NodeContentsWeak::CDataSection => 4,
            NodeContentsWeak::ProcessingInstruction => 5,
            NodeContentsWeak::Comment => 6,
            NodeContentsWeak::Document => 7,
            NodeContentsWeak::DocumentType => 8,
            NodeContentsWeak::DocumentFragment => 9,
        }
    }

    fn upgrade(&self) -> Option<NodeContentsArc> {
        Some(match self {
            NodeContentsWeak::Element(weak) => NodeContentsArc::Element(weak.upgrade()?),
            NodeContentsWeak::Attribute => NodeContentsArc::Attribute,
            NodeContentsWeak::Text => NodeContentsArc::Text,
            NodeContentsWeak::CDataSection => NodeContentsArc::CDataSection,
            NodeContentsWeak::ProcessingInstruction => NodeContentsArc::ProcessingInstruction,
            NodeContentsWeak::Comment => NodeContentsArc::Comment,
            NodeContentsWeak::Document => NodeContentsArc::Document,
            NodeContentsWeak::DocumentType => NodeContentsArc::DocumentType,
            NodeContentsWeak::DocumentFragment => NodeContentsArc::DocumentFragment,
        })
    }
}

/// The DOM [node](https://developer.mozilla.org/en-US/docs/Web/API/Node)
pub struct NodeCommon {
    pub(crate) node_graph: NodeGraphStorage,

    // just a context without behaviour wrapper for now
    pub context: Weak<Sandbox>,
}

// The tree structure is that you have common
// and concrete storage for each node
// AnyNode and ConcreteNode are nodes for acessing
// this storage.
// Common and Concrete are unique for each node, hence they
// are in Arcs, AnyNodeRef and ConcreteNodeRef are just wrappers
// With this we would actually probably not even need nice
#[derive(Clone)]
pub struct AnyNodeArc {
    pub(crate) contents: NodeContentsArc,
    pub(crate) common: Arc<NodeCommon>,
}

#[derive(Clone)]
pub struct AnyNodeWeak {
    pub(crate) contents: NodeContentsWeak,
    pub(crate) common: Weak<NodeCommon>,
}
// #[derive(Clone)]
// pub struct ConcreteNodeRef<T> {
//     pub(crate) contents: Arc<T>,
//     pub(crate) common: Arc<NodeCommon>,
// }

// NodeBehaviour trait will be here for now
pub trait NodeBehaviour {
    fn first_child(&self) -> Option<AnyNodeArc>;
    fn last_child(&self) -> Option<AnyNodeArc>;
    fn append_child(&self, other: AnyNodeArc);
    fn static_child_nodes(&self) -> Vec<AnyNodeArc>;
    fn child_nodes(&self) -> Arc<NodeList>;
    fn clone_node(&self) -> AnyNodeArc;
    fn get_node_type(&self) -> isize;
}

impl AnyNodeWeak {
    fn upgrade(&self) -> Option<AnyNodeArc> {
        Some(AnyNodeArc {
            common: self.common.upgrade()?,
            contents: self.contents.upgrade()?,
        })
    }
}

impl AnyNodeArc {
    fn downgrade(&self) -> AnyNodeWeak {
        AnyNodeWeak {
            common: Arc::downgrade(&self.common),
            contents: self.contents.downgrade(),
        }
    }
}

impl AnyNodeArc {
    pub(crate) fn new(context: Weak<Sandbox>, contents: NodeContentsArc) -> AnyNodeArc {
        let common = Arc::new_cyclic(|construction_weak| NodeCommon {
            node_graph: NodeGraphStorage::new(AnyNodeWeak {
                common: construction_weak.clone(),
                contents: contents.downgrade(),
            }),
            context,
        });

        AnyNodeArc { contents, common }
    }
}

impl SandboxMemberBehavior for AnyNodeArc {
    fn get_context(&self) -> Weak<Sandbox> {
        self.common.context.clone()
    }
}

impl NodeBehaviour for AnyNodeArc {
    fn first_child(&self) -> Option<AnyNodeArc> {
        self.common.node_graph.first_child()
    }

    fn last_child(&self) -> Option<AnyNodeArc> {
        self.common.node_graph.last_child()
    }

    fn append_child(&self, other: AnyNodeArc) {
        self.common.node_graph.append_child(other)
    }

    fn static_child_nodes(&self) -> Vec<AnyNodeArc> {
        self.common.node_graph.static_child_nodes()
    }

    fn child_nodes(&self) -> Arc<NodeList> {
        self.common.node_graph.child_nodes()
    }

    fn clone_node(&self) -> AnyNodeArc {
        let contents = self.contents.clone();
        let mut construction = AnyNodeArc::new(self.get_context(), contents);

        construction
    }

    fn get_node_type(&self) -> isize {
        self.contents.to_isize()
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

// NodeGraphStorage contains all the data connected
// to graph of the nodes
pub struct NodeGraphStorage {
    /// Reference back up to the common Node
    node: AnyNodeWeak,

    parent_node: Option<AnyNodeWeak>,
    left_sibling: Option<AnyNodeWeak>,
    right_sibling: Option<AnyNodeWeak>,
    child_nodes: RwLock<Vec<AnyNodeArc>>,
}

impl NodeGraphStorage {
    pub fn new(node: AnyNodeWeak) -> NodeGraphStorage {
        NodeGraphStorage {
            node,
            parent_node: None,
            left_sibling: None,
            right_sibling: None,
            child_nodes: RwLock::new(Vec::new()),
        }
    }
}

impl NodeGraphStorage {
    fn first_child(&self) -> Option<AnyNodeArc> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).first().cloned()
    }

    fn last_child(&self) -> Option<AnyNodeArc> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).last().cloned()
    }

    fn append_child(&self, other: AnyNodeArc) {
        let mut lock = self.child_nodes.write().unwrap();
        (*lock).push(other);
    }

    fn static_child_nodes(&self) -> Vec<AnyNodeArc> {
        self.child_nodes.read().unwrap().clone()
    }

    fn child_nodes(&self) -> Arc<NodeList> {
        let strong_ref = self.node.upgrade().expect("Sandbox dropped");

        NodeList::new(
            strong_ref.get_context(),
            NodeListStorage::Live(Query::ChildNodes {
                children_of: strong_ref,
            }),
        )
    }
}
