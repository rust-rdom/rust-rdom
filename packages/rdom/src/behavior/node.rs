//! Behavior according to the DOM class called Node
//!
//! Clone node is too different for different nodes,
//! so it is not defined in NodeBehavior

use crate::node_list::{NodeList, NodeListStorage};
use crate::{internal_prelude::*, node_list::Query};
use std::sync::{Arc, RwLock, Weak};


pub struct NodeBehavior {
    /// Reference back up to the common Node
    node: Weak<dyn AnyNode>,

    parent_node: Option<Weak<dyn AnyNode>>,
    left_sibling: Option<Weak<dyn AnyNode>>,
    right_sibling: Option<Weak<dyn AnyNode>>,
    child_nodes: RwLock<Vec<Arc<dyn AnyNode>>>,
}

impl NodeBehavior {
    pub fn new(node: Weak<dyn AnyNode>) -> NodeBehavior {
        NodeBehavior {
            node,
            parent_node: None,
            left_sibling: None,
            right_sibling: None,
            child_nodes: RwLock::new(Vec::new()),
        }
    }

    fn first_child(&self) -> Option<Arc<dyn AnyNode>> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).first().cloned()
    }

    fn last_child(&self) -> Option<Arc<dyn AnyNode>> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).last().cloned()
    }

    fn append_child(&self, other: Arc<dyn AnyNode>) {
        let mut lock = self.child_nodes.write().unwrap();
        (*lock).push(other);
    }

    fn static_child_nodes(&self) -> Vec<Arc<dyn AnyNode>> {
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

#[macro_export]
/// Implements NodeBehavior
macro_rules! impl_node {
    ($structname: ident, $fieldname: ident) => {
        paste::paste! {
        }
    };
}
