//! Types representing references to DOM nodes.

use crate::behavior::sandbox_member::SandboxMemberBehavior;
use crate::node_list::NodeList;
use crate::proxy_node_behavior;
use crate::selector::Selector;
use crate::{behavior::parent_node_prelude::ParentNodeBehaviorStorage, internal_prelude::*};

use concrete::ElementNodeArc;
use contents::{NodeContentsArc, NodeContentsWeak};
use graph_storage::NodeGraphStorage;

pub mod concrete;
pub mod contents;
pub mod element;
pub(crate) mod graph_storage;

pub(crate) use crate::behavior::node::NodeBehavior;

use std::fmt;

/// Marker trait implemented by all node storage classes.
pub trait AnyNodeStore {}

/// Marker trait implemented by any node reference type which can be built.
pub trait Buildable {
    /// Underlying storage struct for the node type.
    type Storage: AnyNodeStore;
}

/// Contains links to mixins/behaviors used by the
/// [Node](https://developer.mozilla.org/en-US/docs/Web/API/Node)
/// class
pub struct NodeCommon {
    pub(crate) node_graph: NodeGraphStorage,

    pub(crate) parent_node_behavior: ParentNodeBehaviorStorage,

    // just a context without behavior wrapper for now
    /// Context, pointing to the Sandbox
    pub context: Weak<Sandbox>,
}

// The tree structure is that you have common
// and concrete storage for each node
// AnyNode and ConcreteNode are nodes for acessing
// this storage.
// Common and Concrete are unique for each node, hence they
// are in Arcs, AnyNodeRef and ConcreteNodeRef are just wrappers
// With this we would actually probably not even need nice

/// a strong reference to any node (abstract nonspecific type)
#[sourcegen::sourcegen(generator = "behave", script = "SandboxMember common.context")]
// Generated. All manual edits to the block annotated with #[sourcegen...] will be discarded.
#[derive(Clone)]
pub struct AnyNodeArc {
    pub(crate) contents: NodeContentsArc,
    pub(crate) common: Arc<NodeCommon>,
}

#[sourcegen::generated]
impl AnyNodeArc {
    /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
    pub fn get_context(&self) -> Weak<Sandbox> {
        self.common.context.clone()
    }
}

#[sourcegen::generated]
impl SandboxMemberBehavior for AnyNodeArc {
    fn get_context(&self) -> Weak<Sandbox> {
        self.get_context()
    }
}

impl fmt::Debug for AnyNodeArc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyNodeArc")
            .field("contents", &self.contents)
            .field("common_addr", &format!("{:p}", Arc::as_ptr(&self.common)))
            .finish()
    }
}

/// a weak reference to any node (abstract nonspecific type)
#[derive(Clone)]
pub struct AnyNodeWeak {
    pub(crate) contents: NodeContentsWeak,
    pub(crate) common: Weak<NodeCommon>,
}

impl fmt::Debug for AnyNodeWeak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyNodeWeak")
            .field("contents", &self.contents)
            .field("common_addr", &format!("{:p}", Weak::as_ptr(&self.common)))
            .finish()
    }
}

impl AnyNodeWeak {
    fn upgrade(&self) -> Option<AnyNodeArc> {
        Some(AnyNodeArc {
            common: self.common.upgrade()?,
            contents: self.contents.upgrade()?,
        })
    }
}

impl AnyNodeArc {
    fn downgrade(&self) -> AnyNodeWeak {
        AnyNodeWeak {
            common: Arc::downgrade(&self.common),
            contents: self.contents.downgrade(),
        }
    }
}

impl AnyNodeArc {
    pub(crate) fn new(context: Weak<Sandbox>, contents: NodeContentsArc) -> AnyNodeArc {
        let common = Arc::new_cyclic(|construction_weak| NodeCommon {
            node_graph: NodeGraphStorage::new(AnyNodeWeak {
                common: construction_weak.clone(),
                contents: contents.downgrade(),
            }),
            parent_node_behavior: ParentNodeBehaviorStorage::new(construction_weak.clone()),
            context,
        });

        AnyNodeArc { contents, common }
    }

    proxy_node_behavior!();
}

impl NodeBehavior for AnyNodeArc {
    fn first_child(&self) -> Option<AnyNodeArc> {
        self.common.node_graph.first_child()
    }

    fn last_child(&self) -> Option<AnyNodeArc> {
        self.common.node_graph.last_child()
    }

    fn append_child(&self, other: AnyNodeArc) {
        self.common.node_graph.append_child(other)
    }

    fn child_nodes(&self) -> Arc<NodeList> {
        self.common.node_graph.child_nodes()
    }

    fn clone_node(&self) -> AnyNodeArc {
        let contents = self.contents.clone();
        AnyNodeArc::new(self.get_context(), contents)
    }

    fn node_type(&self) -> isize {
        self.contents.to_node_type().get_node_number()
    }

    fn query_selector(&self, selector: &Selector) -> Result<Option<ElementNodeArc>, DomError> {
        self.common.parent_node_behavior.query_selector(selector)
    }
}
