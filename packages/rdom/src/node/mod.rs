//! Core representation of a DOM node. See `nice` module for distinction from
//! nice representation.

use crate::internal_prelude::*;
use crate::node_list::NodeList;

crate::use_behaviors!(sandbox_member);
use crate::window::Window;

use contents::{NodeContentsArc, NodeContentsWeak};
use graph_storage::NodeGraphStorage;

pub(crate) mod concrete;
pub(crate) mod contents;
pub mod element;
pub(crate) mod graph_storage;

pub(crate) trait AnyNodeStorage {}

/// An input event
pub struct InputEvent {}

/// The DOM [node](https://developer.mozilla.org/en-US/docs/Web/API/Node)
pub(crate) struct NodeCommon {
    pub(crate) node_graph: NodeGraphStorage,

    // just a context without behaviour wrapper for now
    /// Context, pointing to the Sandbox
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
/// A strong reference to any node (nonspecific type).
pub struct AnyNodeArc {
    pub(crate) contents: NodeContentsArc,
    pub(crate) common: Arc<NodeCommon>,
}

#[derive(Clone)]
/// A weak reference to any node (nonspecific type).
pub struct AnyNodeWeak {
    pub(crate) contents: NodeContentsWeak,
    pub(crate) common: Weak<NodeCommon>,
}

// NodeBehavior trait will be here for now
/// Trait for main functions connected to node behaviour
pub trait NodeBehavior {
    /// Returns first child
    fn first_child(&self) -> Option<AnyNodeArc>;
    /// Returns last child
    fn last_child(&self) -> Option<AnyNodeArc>;
    /// Adds child to child list
    fn append_child(&self, other: AnyNodeArc);
    /// Gets live list of all child nodes
    fn child_nodes(&self) -> Arc<NodeList>;
    /// Clones node
    fn clone_node(&self) -> AnyNodeArc;
    /// [Node.getType](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)
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

impl NodeBehavior for AnyNodeArc {
    fn first_child(&self) -> Option<AnyNodeArc> {
        self.common.node_graph.first_child()
    }

    fn last_child(&self) -> Option<AnyNodeArc> {
        self.common.node_graph.last_child()
    }

    fn append_child(&self, other: AnyNodeArc) {
        self.common.node_graph.append_child(other)
    }

    fn child_nodes(&self) -> Arc<NodeList> {
        self.common.node_graph.child_nodes()
    }

    fn clone_node(&self) -> AnyNodeArc {
        let contents = self.contents.clone();
        AnyNodeArc::new(self.get_context(), contents)
    }

    fn get_node_type(&self) -> isize {
        self.contents.to_node_type().get_node_number()
    }
}

/*

// TODO for DocumentNode; this will require a "nice" instantiation
/// Creates a text node.
pub fn create_text_node(&self, text: String) -> Arc<TextNode> {
    TextNode::new(self.get_context(), TextNodeStorage { text })
}

*/
