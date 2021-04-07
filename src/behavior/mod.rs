//! Provides support for behaviors as defined in the DOM standards. Because the standards
//! refer extensively to classes and mixins, and because Rust does not support either one,
//! this module provides several structures that provide the same behavior but in a Rust-
//! friendly way (using composition instead of inheritance).
//!
//! Every behaviour is implemented using traits, which can be dependent on one another.
//! Every behaviour has a **BehaviourName**Behaviour and **BehaviourName**BehaviourStorage and
//! implementing macro.

use std::sync::RwLock;

use crate::internal_prelude::*;
use crate::node_list::{NodeList, Query, NodeListStorage};

pub mod node;
pub mod sandbox_member;

/// Behavior according to the DOM class called Node
pub struct NodeBehavior {
    /// Reference back up to the core Node
    node: Weak<dyn AnyNode>,

    parent_node: Option<Weak<dyn AnyNode>>,
    left_sibling: Option<Weak<dyn AnyNode>>,
    right_sibling: Option<Weak<dyn AnyNode>>,
    child_nodes: RwLock<Vec<Arc<dyn AnyNode>>>,
}

impl NodeBehavior {
    pub(crate) fn new(node: Weak<dyn AnyNode>) -> NodeBehavior {
        NodeBehavior {
            node,
            parent_node: None,
            left_sibling: None,
            right_sibling: None,
            child_nodes: RwLock::new(Vec::new()),
        }
    }

    pub(crate) fn first_child(&self) -> Option<Arc<dyn AnyNode>> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).first().cloned()
    }

    pub(crate) fn last_child(&self) -> Option<Arc<dyn AnyNode>> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).last().cloned()
    }

    pub(crate) fn append_child(&self, other: Arc<dyn AnyNode>) {
        let mut lock = self.child_nodes.write().unwrap();
        (*lock).push(other);
    }

    pub(crate) fn static_child_nodes(&self) -> Vec<Arc<dyn AnyNode>> {
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

    pub(crate) fn clone_node(&self) -> Result<Arc<dyn AnyNode>, DomError> {
        let node_core = self.node.upgrade().ok_or(DomError::SandboxDropped)?;
        Ok((*node_core).clone_node())
    }
}

/// Behavior according to the DOM class called Element
pub struct ElementBehavior {
    /// Reference back up to the core Element
    element: Weak<dyn element::AnyElement>,
}

impl ElementBehavior {
    pub(crate) fn new(element: Weak<dyn element::AnyElement>) -> ElementBehavior {
        ElementBehavior { element }
    }
}

/// Behavior according to the DOM class called ParentNode
pub struct ParentNodeBehavior {
    /// Reference back up to the core Node
    node: Weak<dyn AnyNode>,
}

impl ParentNodeBehavior {
    pub(crate) fn new(node: Weak<dyn AnyNode>) -> ParentNodeBehavior {
        ParentNodeBehavior { node }
    }
}
