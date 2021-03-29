//! Representation of a [NodeList](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)
//! and associated metadata.

use std::sync::{Arc, Weak};

use crate::node::raw::{AnyRawNode, private::PrivateAnyRawNode};
use crate::sandbox::Sandbox;

/// Represents a [NodeList](https://developer.mozilla.org/en-US/docs/Web/API/NodeList) structure,
/// which may be either "live" or "static". Note that these are not strongly retained by the
/// Sandbox, and there is no guarantee they will work after the Sandbox has been dropped. So, to
/// use a NodeList, make sure you have retained both the Sandbox and an Rc to the NodeList before
/// performing any operations.
///
/// Also note that retaining a NodeList may cause other Nodes to be retained. For example,
/// `some_node.child_nodes()` and retaining the return value will cause `some_node` to be
/// retained.
pub struct NodeList {
    /// Reference to the sandbox to which this NodeList belongs
    pub context: Weak<Sandbox>,

    /// The underlying storage
    pub(crate) nodelist_storage: NodeListStorage,
}

impl NodeList {
    fn new(context: Weak<Sandbox>, nodelist_storage: NodeListStorage) -> Arc<NodeList> {
        Arc::new(NodeList {
            context,
            nodelist_storage
        })
    }

    pub(crate) fn new_static(context: Weak<Sandbox>, elements: Vec<Arc<dyn AnyRawNode>>) -> Arc<NodeList> {
        let nodelist_storage = NodeListStorage::Static(elements);
        return NodeList::new(context, nodelist_storage);
    }

    fn length(&self) -> usize {
        match &self.nodelist_storage {
            NodeListStorage::Static(list) => list.len(),
            NodeListStorage::Live(query) => {
                match query {
                    Query::ChildNodes { children_of } => {
                        children_of.get_node_behavior().static_child_nodes().len()
                    }
                }
            }
        }
    }

    fn item(&self, index: usize) -> Option<Arc<dyn AnyRawNode>> {
        match &self.nodelist_storage {
            NodeListStorage::Static(list) => list.get(index).map(|r| r.clone()),
            NodeListStorage::Live(query) => {
                match query {
                    Query::ChildNodes { children_of } => {
                        children_of.get_node_behavior().static_child_nodes().get(index).map(|r| r.clone())
                    }
                }
            }
        }
    }

    fn get(&self, index: usize) -> Option<Arc<dyn AnyRawNode>> {
        self.item(index)
    }
}

/// An encapsulation of how the NodeList will respond to operations.
pub(crate) enum NodeListStorage {
    /// A static list of nodes (e.g. result of Document.query_selector_all(...))
    Static(Vec<Arc<dyn AnyRawNode>>),

    /// Some dynamic query (e.g. result of Node.child_nodes())
    Live(Query),
}

pub(crate) enum Query {
    ChildNodes {
        children_of: Arc<dyn AnyRawNode>
    },
}