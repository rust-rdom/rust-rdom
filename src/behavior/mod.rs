//! Provides support for behaviors as defined in the DOM standards. Because the standards
//! refer extensively to classes and mixins, and because Rust does not support either one,
//! this module provides several structures that provide the same behavior but in a Rust-
//! friendly way (using composition instead of inheritance).
//!
//! Every behaviour is implemented using traits, which can be dependent on one another.
//! Every behaviour has a **BehaviourName**Behaviour and **BehaviourName**BehaviourStorage and
//! implementing macro.

use std::sync::Weak;

use crate::internal_prelude::*;

pub mod node;
pub mod sandbox_member;

/// Behavior according to the DOM class called Node

/// Behavior according to the DOM class called Element
pub struct ElementBehavior {
    /// Reference back up to the common Element
    element: Weak<dyn element::AnyElement>,
}

impl ElementBehavior {
    pub(crate) fn new(element: Weak<dyn element::AnyElement>) -> ElementBehavior {
        ElementBehavior { element }
    }
}

/// Behavior according to the DOM class called ParentNode
pub struct ParentNodeBehavior {
    /// Reference back up to the common Node
    node: Weak<dyn AnyNode>,
}

impl ParentNodeBehavior {
    pub(crate) fn new(node: Weak<dyn AnyNode>) -> ParentNodeBehavior {
        ParentNodeBehavior { node }
    }
}
