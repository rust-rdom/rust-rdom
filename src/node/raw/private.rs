use std::sync::Arc;

use crate::behavior::NodeBehavior;

/// Internal-only functionality of AnyRawNode
pub trait PrivateAnyRawNode {
    fn get_node_behavior(&self) -> Arc<NodeBehavior>;
}