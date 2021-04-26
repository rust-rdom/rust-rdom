//! Concrete (as opposed to abstract) types of nodes. Each node type is represented in this module.

use crate::internal_prelude::*;

use super::contents::{
    AttributeStore, CDataSectionStore, CommentStore, DocumentFragmentStore, DocumentStore,
    DocumentTypeStore, ProcessingInstructionStore, TextStore,
};
use super::graph_storage::Selector;
use super::{
    template::TemplateWeak, AnyNodeStore, NodeBehavior, NodeCommon, NodeContentsArc,
    NodeContentsWeak, NodeGraphStorage,
};
use crate::node_list::NodeList;
use std::convert::TryFrom;
crate::use_behaviors!(sandbox_member);

#[derive(Clone)]
/// A strongly-typed handle to a node with a strong reference.
/// `S` may be the underlying storage
/// type of any node.
pub struct ConcreteNodeArc<S: AnyNodeStore> {
    pub(crate) contents: Arc<S>,
    pub(crate) common: Arc<NodeCommon>,
}

#[derive(Clone)]
/// A strongly-typed handle to a node with a weak reference.
/// `S` may be the underlying storage
/// type of any node.
pub struct ConcreteNodeWeak<S: AnyNodeStore> {
    pub(crate) contents: Weak<S>,
    pub(crate) common: Weak<NodeCommon>,
}

macro_rules! impl_concrete {
    ($($ti:expr => $name:ident),*) => {
        paste::paste! {
            $(
                impl AnyNodeStore for [<$name Store>] {}

                #[doc = "Convenience alias for a strong reference to a(n) " $name " node"]
                pub type [<$name NodeArc>] = ConcreteNodeArc<[<$name Store>]>;

                #[doc = "Convenience alias for a weak reference to a(n) " $name " node"]
                pub type [<$name NodeWeak>] = ConcreteNodeWeak<[<$name Store>]>;

                impl ConcreteNodeArc<[<$name Store>]> {
                    pub(crate) fn new(context: Weak<Sandbox>, contents: Arc<[<$name Store>]>) ->
                    ConcreteNodeArc<[<$name Store>]> {
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

                impl SandboxMemberBehavior for ConcreteNodeArc<[<$name Store>]> {
                    fn get_context(&self) -> Weak<Sandbox> {
                        self.common.context.clone()
                    }
                }

                impl TryFrom<AnyNodeArc> for ConcreteNodeArc<[<$name Store>]> {
                    type Error = AnyNodeArc;

                    fn try_from(value: AnyNodeArc) -> Result<Self, Self::Error> {
                        match value.contents {
                            NodeContentsArc::$name(element) => {
                                return Ok(ConcreteNodeArc {
                                    contents: element,
                                    common: value.common,
                                })
                            },
                            _ => Err(value),
                        }
                    }
                }

                impl TryFrom<AnyNodeWeak> for ConcreteNodeWeak<[<$name Store>]> {
                    type Error = AnyNodeWeak;

                    fn try_from(value: AnyNodeWeak) -> Result<Self, Self::Error> {
                        match value.contents {
                            NodeContentsWeak::$name(element) => {
                                return Ok(ConcreteNodeWeak {
                                    contents: element,
                                    common: value.common,
                                })
                            },
                            _ => Err(value),
                        }
                    }
                }

                impl From<ConcreteNodeArc<[<$name Store>]>> for AnyNodeArc {
                    fn from(concrete: ConcreteNodeArc<[<$name Store>]>) -> Self {
                        AnyNodeArc {
                            common: concrete.common,
                            contents: NodeContentsArc::$name(concrete.contents),
                        }
                    }
                }

                impl From<ConcreteNodeWeak<[<$name Store>]>> for AnyNodeWeak {
                    fn from(concrete: ConcreteNodeWeak<[<$name Store>]>) -> Self {
                        AnyNodeWeak {
                            common: concrete.common,
                            contents: NodeContentsWeak::$name(concrete.contents),
                        }
                    }
                }

                impl NodeBehavior for ConcreteNodeArc<[<$name Store>]> {
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
                        $ti
                    }

                    fn query_selector(&self, selector: &Selector) -> Option<ElementNodeArc> {
                        self.common.node_graph.query_selector_rec(selector)
                    }
                }

                impl TemplateWeak<[<$name NodeArc>]> for [<$name Store>] {
                    fn build(self, context: Weak<Sandbox>) -> [<$name NodeArc>] {
                        [<$name NodeArc>]::new(context, Arc::new(self))
                    }
                }
            )*
        }
    }
}

impl_concrete! {
    1 => Element,
    2 => Attribute,
    3 => Text,
    4 => CDataSection,
    5 => ProcessingInstruction,
    6 => Comment,
    7 => Document,
    8 => DocumentType,
    9 => DocumentFragment
}

impl DocumentNodeArc {
    /// Creates a new text node with the given text contents
    pub fn create_text_node(&self, text: String) -> TextNodeArc {
        TextNodeArc::new(self.get_context(), Arc::new(TextStore { data: text }))
    }
}
