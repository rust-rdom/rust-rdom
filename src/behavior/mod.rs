//! Provides support for behaviors as defined in the DOM standards. Because the standards
//! refer extensively to classes and mixins, and because Rust does not support either one,
//! this module provides several structures that provide the same behavior but in a Rust-
//! friendly way (using composition instead of inheritance).

use std::sync::{Arc, Weak};

use crate::error::DomError;
use crate::node::raw::element as raw_element;
use crate::node::raw::AnyRawNode;

/// Behavior according to the DOM class called Node
pub struct NodeBehavior {
    /// Reference back up to the raw Node
    node: Weak<dyn AnyRawNode>,

    parent_node: Option<Weak<dyn AnyRawNode>>,
    left_sibling: Option<Weak<dyn AnyRawNode>>,
    right_sibling: Option<Weak<dyn AnyRawNode>>,
    child_nodes: Vec<Arc<dyn AnyRawNode>>,
}

impl NodeBehavior {
    pub(crate) fn new(node: Weak<dyn AnyRawNode>) -> NodeBehavior {
        NodeBehavior {
            node,
            parent_node: None,
            left_sibling: None,
            right_sibling: None,
            child_nodes: Vec::new(),
        }
    }

    pub(crate) fn first_child(&self) -> Option<&Arc<dyn AnyRawNode>> {
        self.child_nodes.first()
    }

    pub(crate) fn last_child(&self) -> Option<&Arc<dyn AnyRawNode>> {
        self.child_nodes.last()
    }

    pub(crate) fn append_child(&mut self, other: Arc<dyn AnyRawNode>) {
        self.child_nodes.push(other)
    }

    pub(crate) fn static_child_nodes(&self) -> Vec<Arc<dyn AnyRawNode>> {
        self.child_nodes.clone()
    }

    pub(crate) fn clone_node(&self) -> Result<Arc<dyn AnyRawNode>, DomError> {
        let raw_node = self
            .node
            .upgrade()
            .ok_or_else(|| DomError::SandboxDropped)?;
        Ok((*raw_node).clone_node())
    }
}

/// Behavior according to the DOM class called Element
pub struct ElementBehavior {
    /// Reference back up to the raw Element
    element: Weak<dyn raw_element::AnyRawElement>,
}

impl ElementBehavior {
    pub(crate) fn new(element: Weak<dyn raw_element::AnyRawElement>) -> ElementBehavior {
        ElementBehavior { element }
    }
}

/// Behavior according to the DOM class called ParentNode
pub struct ParentNodeBehavior {
    /// Reference back up to the raw Node
    node: Weak<dyn AnyRawNode>,
}

impl ParentNodeBehavior {
    pub(crate) fn new(node: Weak<dyn AnyRawNode>) -> ParentNodeBehavior {
        ParentNodeBehavior {
            node,
        }
    }
}