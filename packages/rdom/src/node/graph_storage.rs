use crate::internal_prelude::*;
use std::{convert::TryFrom, sync::RwLock};
use crate::node_list::{Query, NodeList, NodeListStorage};
use crate::behavior::sandbox_member::SandboxMemberBehavior;

/// NodeGraphStorage contains all the data connected
/// to graph of the nodes
pub struct NodeGraphStorage {
    /// Reference back up to the common Node
    node: AnyNodeWeak,

    parent_node: Option<AnyNodeWeak>,
    left_sibling: Option<AnyNodeWeak>,
    right_sibling: Option<AnyNodeWeak>,
    child_nodes: RwLock<Vec<AnyNodeArc>>,
}

impl NodeGraphStorage {
    /// Constructs a new NodeGraphStorage
    pub fn new(node: AnyNodeWeak) -> NodeGraphStorage {
        NodeGraphStorage {
            node,
            parent_node: None,
            left_sibling: None,
            right_sibling: None,
            child_nodes: RwLock::new(Vec::new()),
        }
    }

    pub(crate) fn first_child(&self) -> Option<AnyNodeArc> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).first().cloned()
    }

    pub(crate) fn last_child(&self) -> Option<AnyNodeArc> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).last().cloned()
    }

    pub(crate) fn append_child(&self, other: AnyNodeArc) {
        let mut lock = self.child_nodes.write().unwrap();
        (*lock).push(other);
    }

    pub(crate) fn static_child_nodes(&self) -> Vec<AnyNodeArc> {
        self.child_nodes.read().unwrap().clone()
    }

    pub(crate) fn child_nodes(&self) -> Arc<NodeList> {
        let strong_ref = self.node.upgrade().expect("Sandbox dropped");

        NodeList::new(
            strong_ref.get_context(),
            NodeListStorage::Live(Query::ChildNodes {
                children_of: strong_ref,
            }),
        )
    }
}