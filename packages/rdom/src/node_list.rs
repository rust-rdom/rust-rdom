//! Representation of a [NodeList](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)
//! and associated metadata.

use crate::internal_prelude::*;

crate::use_behaviors!(sandbox_member);

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
    pub context: SandboxMemberBehaviorStorage,

    /// The underlying storage
    pub(crate) nodelist_storage: NodeListStorage,
}

impl NodeList {
    pub(crate) fn new(context: Weak<Sandbox>, nodelist_storage: NodeListStorage) -> Arc<NodeList> {
        Arc::new(NodeList {
            context: SandboxMemberBehaviorStorage::new(context),
            nodelist_storage,
        })
    }

    pub(crate) fn new_static(context: Weak<Sandbox>, elements: Vec<AnyNodeArc>) -> Arc<NodeList> {
        let nodelist_storage = NodeListStorage::Static(elements);
        NodeList::new(context, nodelist_storage)
    }

    /// NodeList#length
    pub fn length(&self) -> usize {
        match &self.nodelist_storage {
            NodeListStorage::Static(list) => list.len(),
            NodeListStorage::Live(query) => match query {
                Query::ChildNodes { children_of } => {
                    children_of.common.node_graph.static_child_nodes().len()
                }
            },
        }
    }

    /// NodeList#item
    pub fn item(&self, index: usize) -> Option<AnyNodeArc> {
        match &self.nodelist_storage {
            NodeListStorage::Static(list) => list.get(index).cloned(),
            NodeListStorage::Live(query) => match query {
                Query::ChildNodes { children_of } => children_of
                    .common
                    .node_graph
                    .static_child_nodes()
                    .get(index)
                    .cloned(),
            },
        }
    }

    /// NodeList#get
    pub fn get(&self, index: usize) -> Option<AnyNodeArc> {
        self.item(index)
    }
}

impl_sandbox_member!(NodeList, context);

/// An encapsulation of how the NodeList will respond to operations.
pub(crate) enum NodeListStorage {
    /// A static list of nodes (e.g. result of Document.query_selector_all(...))
    Static(Vec<AnyNodeArc>),

    /// Some dynamic query (e.g. result of Node.child_nodes())
    Live(Query),
}

pub(crate) enum Query {
    ChildNodes { children_of: AnyNodeArc },
}