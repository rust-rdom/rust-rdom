use crate::internal_prelude::*;

use crate::behavior::NodeBehavior;

/// Internal-only functionality of AnyNode
pub trait PrivateAnyNode {
    fn get_node_behavior(&self) -> Arc<NodeBehavior>;
}
