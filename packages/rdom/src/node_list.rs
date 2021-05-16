//! Representation of a [NodeList](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)
//! and associated metadata.

use crate::internal_prelude::*;

#[sourcegen::sourcegen(generator = "inject", template = "food")]
// Generated. All manual edits to the block annotated with #[sourcegen...] will be discarded.

struct Fibb {
    parent_node: Option<AnyNodeWeak>,
    left_sibling: Option<AnyNodeWeak>,
    right_sibling: Option<AnyNodeWeak>,
    child_nodes: RwLock<Vec<AnyNodeArc>>,
}

#[sourcegen::generated]
impl Fibb {
    /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
    pub fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
}

#[sourcegen::generated]
impl SandboxMemberBehavior for Fibb {
    /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
    fn get_context(&self) -> Weak<Sandbox> {
        self.get_context()
    }
}

#[sourcegen::generated]
impl Fibb {
    fn foo(&self) {}
}

/// Represents a [NodeList](https://developer.mozilla.org/en-US/docs/Web/API/NodeList) structure,
/// which may be either "live" or "static". Note that these are not strongly retained by the
/// Sandbox, and there is no guarantee they will work after the Sandbox has been dropped. So, to
/// use a NodeList, make sure you have retained both the Sandbox and an Rc to the NodeList before
/// performing any operations.
///
/// Also note that retaining a NodeList may cause other Nodes to be retained. For example,
/// `some_node.child_nodes()` and retaining the return value will cause `some_node` to be
/// retained.
#[sourcegen::sourcegen(generator = "behave", script = "SandboxMember context")]
// Generated. All manual edits to the block annotated with #[sourcegen...] will be discarded.
pub struct NodeList {
    /// Reference to the sandbox to which this NodeList belongs
    context: Weak<Sandbox>,
    /// The underlying storage
    pub(crate) nodelist_storage: NodeListStorage,
}

#[sourcegen::generated]
impl NodeList {
    /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
    pub fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
}

#[sourcegen::generated]
impl SandboxMemberBehavior for NodeList {
    fn get_context(&self) -> Weak<Sandbox> {
        self.get_context()
    }
}

impl NodeList {
    pub(crate) fn new(context: Weak<Sandbox>, nodelist_storage: NodeListStorage) -> Arc<NodeList> {
        Arc::new(NodeList {
            context,
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
