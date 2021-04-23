//! Behavior according to the DOM class called Node
//!
//! Clone node is too different for different nodes,
//! so it is not defined in NodeBehavior

use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::node::graph_storage::Selector;
use crate::node::AnyNodeArc;
use crate::node_list::NodeList;

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
    /// [.querySelector](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)
    fn query_selector(&self, selector: &Selector) -> Option<ElementNodeArc>;
}
