//! Raw representation of a DOM element. See [node](../../index.html) module for distinction from
//! wrapped representation.

use downcast_rs::DowncastSync;
use paste::paste;
use std::sync::{Arc, Weak};

use crate::behavior::NodeBehavior;
use crate::node::raw::AnyRawNode;
use crate::sandbox::Sandbox;

/// A base trait for all raw element types
pub trait AnyRawElement: DowncastSync + AnyRawNode {}
impl_downcast!(sync AnyRawElement);

/// Represents a [NodeList](https://developer.mozilla.org/en-US/docs/Web/API/NodeList), which
/// may be either live or static. Note that these are not strongly retained by the Sandbox,
/// and there is no guarantee they will work after the Sandbox has been dropped. So, to use
/// a NodeList, make sure you have retained both the Sandbox and an Rc to the NodeList before
/// performing any operations.
pub struct NodeList {
    /// Reference to the sandbox to which this NodeList belongs
    pub context: Weak<Sandbox>,

    /// The underlying storage
    pub(crate) nodelist_storage: Arc<NodeListStorage>,
}

impl NodeList {
    pub(crate) fn new(context: Weak<Sandbox>, nodelist_storage: Arc<NodeListStorage>) -> Arc<NodeList> {
        Arc::new(NodeList {
            context,
            nodelist_storage
        })
    }

    fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
}

/// An encapsulation of how the NodeList will respond to operations.
pub(crate) enum NodeListStorage {
    /// A static list of nodes (e.g. result of Document.query_selector_all(...))
    Static(Vec<Weak<dyn AnyRawNode>>),

    /// Some dynamic query (e.g. result of Node.child_nodes())
    Live(Query),
}

pub(crate) enum Query {
    ChildNodes {
        children_of: Weak<dyn AnyRawNode>
    },
}