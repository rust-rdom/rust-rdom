//! Provides support for behaviors as defined in the DOM standards. Because the standards
//! refer extensively to classes and mixins, and because Rust does not support either one,
//! this module provides several structures that provide the same behavior but in a Rust-
//! friendly way (using composition instead of inheritance).

use std::sync::Weak;

use crate::node::raw::AnyRawNode;

/// Behavior according to the DOM class called Node
pub struct NodeBehavior {
}

impl NodeBehavior {
    pub(crate) fn new(node: Weak<impl AnyRawNode>) -> NodeBehavior {
        NodeBehavior {}
    }
}