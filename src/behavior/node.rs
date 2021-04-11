//! Behavior according to the DOM class called Node
//!
//! Clone node is too different for different nodes,
//! so it is not defined in NodeBehavior

use crate::node_list::{NodeList, NodeListStorage};
use crate::{internal_prelude::*, node_list::Query};
use std::sync::{Arc, RwLock, Weak};

pub trait NodeBehavior {
    fn first_child(&self) -> Option<Arc<dyn AnyNode>>;
    fn last_child(&self) -> Option<Arc<dyn AnyNode>>;
    fn append_child(&self, other: Arc<dyn AnyNode>);

    #[doc(hidden)]
    /// RDOM-private: gives a clone of the backing children Vec
    fn static_child_nodes(&self) -> Vec<Arc<dyn AnyNode>>;

    fn child_nodes(&self) -> Arc<NodeList>;
}

pub struct NodeBehaviorStorage {
    /// Reference back up to the common Node
    node: Weak<dyn AnyNode>,

    parent_node: Option<Weak<dyn AnyNode>>,
    left_sibling: Option<Weak<dyn AnyNode>>,
    right_sibling: Option<Weak<dyn AnyNode>>,
    child_nodes: RwLock<Vec<Arc<dyn AnyNode>>>,
}

impl NodeBehaviorStorage {
    pub fn new(node: Weak<dyn AnyNode>) -> NodeBehaviorStorage {
        NodeBehaviorStorage {
            node,
            parent_node: None,
            left_sibling: None,
            right_sibling: None,
            child_nodes: RwLock::new(Vec::new()),
        }
    }
}

impl NodeBehavior for NodeBehaviorStorage {
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
            impl NodeBehavior for $structname {
                fn first_child(&self) -> Option<Arc<dyn AnyNode>> {
                    self.$fieldname.first_child()
                }

                fn last_child(&self) -> Option<Arc<dyn AnyNode>> {
                    self.$fieldname.last_child()
                }

                fn append_child(&self, other: Arc<dyn AnyNode>) {
                    self.$fieldname.append_child(other)
                }

                fn static_child_nodes(&self) -> Vec<Arc<dyn AnyNode>> {
                    self.$fieldname.static_child_nodes()
                }

                fn child_nodes(&self) -> Arc<$crate::node_list::NodeList> {
                    self.$fieldname.child_nodes()
                }
            }
        }
    };
}
