//! Core representation of a DOM node. See `nice` module for distinction from
//! nice representation.

use crate::node_list::{NodeList, NodeListStorage};
use crate::{internal_prelude::*, node_list::Query};
use std::{convert::TryFrom, sync::RwLock};

crate::use_behaviors!(sandbox_member);
use crate::window::Window;

use contents::{NodeContentsArc, NodeContentsWeak, NodeType};
use element::ElementNodeStorage;
use graph_storage::NodeGraphStorage;

pub(crate) mod contents;
pub(crate) mod typed_arc;
pub mod element;
pub(crate) mod graph_storage;

/// An input event
pub struct InputEvent {}

/// The DOM [node](https://developer.mozilla.org/en-US/docs/Web/API/Node)
pub(crate) struct NodeCommon {
    pub(crate) node_graph: NodeGraphStorage,

    // just a context without behaviour wrapper for now
    /// Context, pointing to the Sandbox
    pub context: Weak<Sandbox>,
}

pub(crate) trait AnyStorage {}

// The tree structure is that you have common
// and concrete storage for each node
// AnyNode and ConcreteNode are nodes for acessing
// this storage.
// Common and Concrete are unique for each node, hence they
// are in Arcs, AnyNodeRef and ConcreteNodeRef are just wrappers
// With this we would actually probably not even need nice
#[derive(Clone)]
/// Strong pair of refernces to common and concrete node storages
pub struct AnyNodeArc {
    pub(crate) contents: NodeContentsArc,
    pub(crate) common: Arc<NodeCommon>,
}

#[derive(Clone)]
/// Weak version of AnyNodeArc
pub struct AnyNodeWeak {
    pub(crate) contents: NodeContentsWeak,
    pub(crate) common: Weak<NodeCommon>,
}

#[derive(Clone)]
/// Concrete variant of AnyNodeArc
pub struct ConcreteNodeArc<T> {
    pub(crate) contents: Arc<T>,
    pub(crate) common: Arc<NodeCommon>,
}

#[derive(Clone)]
/// Concrete variant of AnyNodeWeak
pub struct ConcreteNodeWeak<T> {
    pub(crate) contents: Weak<T>,
    pub(crate) common: Weak<NodeCommon>,
}

macro_rules! impl_concrete {
    ($var:ident($ty:ident)) => {
        impl AnyStorage for $ty {}

        impl ConcreteNodeArc<$ty> {
            pub(crate) fn new(context: Weak<Sandbox>, contents: Arc<$ty>) -> ConcreteNodeArc<$ty> {
                let common = Arc::new_cyclic(|construction_weak| NodeCommon {
                    node_graph: NodeGraphStorage::new(AnyNodeWeak {
                        contents: (&contents).into(),
                        common: construction_weak.clone(),
                    }),
                    context,
                });

                ConcreteNodeArc { contents, common }
            }
        }

        impl SandboxMemberBehavior for ConcreteNodeArc<$ty> {
            fn get_context(&self) -> Weak<Sandbox> {
                self.common.context.clone()
            }
        }

        impl TryFrom<AnyNodeArc> for ConcreteNodeArc<$ty> {
            type Error = DomError;

            fn try_from(value: AnyNodeArc) -> Result<Self, Self::Error> {
                let contents = match value.contents {
                    NodeContentsArc::$var(element) => Ok(element),
                    _ => Err(DomError::NodeCastFail),
                }?;

                Ok(ConcreteNodeArc {
                    contents,
                    common: value.common,
                })
            }
        }

        impl TryFrom<AnyNodeWeak> for ConcreteNodeWeak<$ty> {
            type Error = DomError;

            fn try_from(value: AnyNodeWeak) -> Result<Self, Self::Error> {
                let contents = match value.contents {
                    NodeContentsWeak::$var(element) => Ok(element),
                    _ => Err(DomError::NodeCastFail),
                }?;

                Ok(ConcreteNodeWeak {
                    contents,
                    common: value.common
                })
            }
        }

        impl From<ConcreteNodeArc<$ty>> for AnyNodeArc {
            fn from(concrete: ConcreteNodeArc<$ty>) -> Self {
                AnyNodeArc {
                    common: concrete.common,
                    contents: NodeContentsArc::$var(concrete.contents),
                }
            }
        }

        impl From<ConcreteNodeWeak<$ty>> for AnyNodeWeak {
            fn from(concrete: ConcreteNodeWeak<$ty>) -> Self {
                AnyNodeWeak {
                    common: concrete.common,
                    contents: NodeContentsWeak::$var(concrete.contents),
                }
            }
        }

        impl NodeBehaviour for ConcreteNodeArc<$ty> {
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
                AnyNodeArc::from(self.clone()).clone_node()
            }

            fn get_node_type(&self) -> isize {
                (NodeType::$var).get_node_number()
            }
        }
    };

    ($($var:ident($ty:ident)),*) => {
        $(
            impl_concrete!($var($ty));
        )*
    }
}

impl_concrete! {
    Element(ElementNodeStorage),
    Document(DocumentNodeStorage),
    Text(TextNodeStorage)
}

// NodeBehaviour trait will be here for now
/// Trait for main functions connected to node behaviour
pub trait NodeBehaviour {
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
            context,
        });

        AnyNodeArc { contents, common }
    }
}

impl SandboxMemberBehavior for AnyNodeArc {
    fn get_context(&self) -> Weak<Sandbox> {
        self.common.context.clone()
    }
}

impl NodeBehaviour for AnyNodeArc {
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
        let construction = AnyNodeArc::new(self.get_context(), contents);

        construction
    }

    fn get_node_type(&self) -> isize {
        self.contents.to_node_type().get_node_number()
    }
}

#[derive(Default, Clone)]
pub(crate) struct DocumentNodeStorage {
    /// Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

#[derive(Default, Clone)]
pub(crate) struct TextNodeStorage {
    /// Text in the text node
    pub(crate) text: String,
}

/*

// TODO for TextNode. this will require a "nice" version
/// Creates a text node.
pub fn get_text(&self) -> Option<String> {
    Some(self.storage.text.clone())
}

// TODO for DocumentNode; this will require a "nice" instantiation
/// Creates a text node.
pub fn create_text_node(&self, text: String) -> Arc<TextNode> {
    TextNode::new(self.get_context(), TextNodeStorage { text })
}

*/
