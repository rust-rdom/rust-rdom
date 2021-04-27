//! Types representing references to DOM nodes.

use crate::internal_prelude::*;
use crate::node_list::NodeList;

crate::use_behaviors!(sandbox_member, node);

use concrete::ElementNodeArc;
use contents::{NodeContentsArc, NodeContentsWeak};
use graph_storage::{NodeGraphStorage, Selector};

pub mod concrete;
pub mod contents;
pub mod element;
pub(crate) mod graph_storage;
pub mod template;

pub use crate::behavior::node::NodeBehavior;

/// Marker trait implemented by all node storage classes.
pub trait AnyNodeStore {}

/// An input event
pub struct InputEvent {}

/// The DOM [node](https://developer.mozilla.org/en-US/docs/Web/API/Node)
pub(crate) struct NodeCommon {
    pub(crate) node_graph: NodeGraphStorage,

    // just a context without behavior wrapper for now
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
/// A strong reference to any node (abstract, nonspecific type).
pub struct AnyNodeArc {
    pub(crate) contents: NodeContentsArc,
    pub(crate) common: Arc<NodeCommon>,
}

#[derive(Clone)]
/// A weak reference to any node (abstract, nonspecific type).
pub struct AnyNodeWeak {
    pub(crate) contents: NodeContentsWeak,
    pub(crate) common: Weak<NodeCommon>,
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

    fn query_selector(&self, selector: &Selector) -> Option<ElementNodeArc> {
        self.common.node_graph.query_selector_rec(selector)
    }
}
