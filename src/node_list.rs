//! Representation of a [NodeList](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)
//! and associated metadata.

use std::sync::{Arc, Weak};

use crate::node::raw::{AnyRawNode, private::PrivateAnyRawNode};
use crate::sandbox::Sandbox;

/// Represents a [NodeList](https://developer.mozilla.org/en-US/docs/Web/API/NodeList), which
/// may be either "live" or "static". Note that these are not strongly retained by the Sandbox,
/// and there is no guarantee they will work after the Sandbox has been dropped. So, to use
/// a NodeList, make sure you have retained both the Sandbox and an Rc to the NodeList before
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

    pub(crate) fn new_static(context: Weak<Sandbox>, elements: Vec<Arc<dyn AnyRawNode>>) -> Option<Arc<NodeList>> {
        if elements.len() >= u32::MAX as usize {
            return None;
        }
        let nodelist_storage = NodeListStorage::Static(elements);
        return Some(NodeList::new(context, nodelist_storage));
    }

    fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }

    fn length(&self) -> u32 {
        match &self.nodelist_storage {
            NodeListStorage::Static(list) => list.len() as u32,
            NodeListStorage::Live(query) => {
                match query {
                    Query::ChildNodes { children_of } => {
                        let (size, _) = (*children_of).get_node_behavior().static_child_nodes();
                        size
                    }
                }
            }
        }
    }

    // fn item(&self, index: u32) -> u32 {

    // }

    // fn get(&self, index: u32) -> u32 {
    //     self.item(index)
    // }
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