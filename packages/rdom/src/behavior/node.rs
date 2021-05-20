//! Behavior according to the DOM class called Node
//!
//! Clone node is too different for different nodes,
//! so it is not defined in NodeBehavior

use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::node::AnyNodeArc;
use crate::node_list::NodeList;
use crate::selector::Selector;

/// NodeBehavior trait for internal use only.
/// Trait for main functions connected to node behavior.
pub(crate) trait NodeBehavior {
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
    fn query_selector(&self, selector: &Selector) -> Result<Option<ElementNodeArc>, DomError>;
}

/// Passes methods through to NodeBehavior, for public use.
#[macro_export]
macro_rules! proxy_node_behavior {
    () => {
        paste::paste! {
            /// Returns first child
            pub fn first_child(&self) -> Option<AnyNodeArc> {
                <Self as crate::behavior::NodeBehavior>::first_child(self)
            }
            /// Returns last child
            pub fn last_child(&self) -> Option<AnyNodeArc> {
                <Self as crate::behavior::NodeBehavior>::last_child(self)
            }
            /// Adds child to child list
            pub fn append_child(&self, other: AnyNodeArc) {
                <Self as crate::behavior::NodeBehavior>::append_child(self, other)
            }
            /// Gets live list of all child nodes
            pub fn child_nodes(&self) -> Arc<NodeList> {
                <Self as crate::behavior::NodeBehavior>::child_nodes(self)
            }
            /// Clones node
            pub fn clone_node(&self) -> AnyNodeArc {
                <Self as crate::behavior::NodeBehavior>::clone_node(self)
            }
            /// [Node.getType](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)
            pub fn node_type(&self) -> isize {
                <Self as crate::behavior::NodeBehavior>::get_node_type(self)
            }
            /// [.querySelector](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)
            pub fn query_selector(&self, selector: &Selector) -> Result<Option<ElementNodeArc>, DomError> {
                <Self as crate::behavior::NodeBehavior>::query_selector(self, selector)
            }
        }
    };
}
