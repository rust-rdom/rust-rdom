use crate::internal_prelude::*;

use super::contents::{
    AttributeNodeStorage, CDataSectionNodeStorage, CommentNodeStorage, DocumentFragmentNodeStorage,
    DocumentNodeStorage, DocumentTypeNodeStorage, ProcessingInstructionNodeStorage,
    TextNodeStorage,
};
use super::{
    Buildable,
    AnyNodeStorage,
    NodeCommon,
    NodeContentsArc,
    NodeContentsWeak,
    NodeGraphStorage
};
use crate::node_list::NodeList;
use std::convert::TryFrom;
crate::use_behaviors!(sandbox_member);

#[derive(Clone)]
/// A strongly-typed handle to a node with a strong reference.
/// `T` may be the underlying storage
/// type of any node.
pub struct ConcreteNodeArc<T> {
    pub(crate) contents: Arc<T>,
    pub(crate) common: Arc<NodeCommon>,
}

#[derive(Clone)]
/// A strongly-typed handle to a node with a weak reference.
/// `T` may be the underlying storage
/// type of any node.
pub struct ConcreteNodeWeak<T> {
    pub(crate) contents: Weak<T>,
    pub(crate) common: Weak<NodeCommon>,
}

macro_rules! impl_concrete {
    ($($ti:expr => $name:ident),*) => {
        paste::paste! {
            $(
                impl AnyNodeStorage for [<$name NodeStorage>] {}

                pub(crate) type [<$name NodeArc>] = ConcreteNodeArc<[<$name NodeStorage>]>;
                pub(crate) type [<$name NodeWeak>] = ConcreteNodeWeak<[<$name NodeStorage>]>;

                impl ConcreteNodeArc<[<$name NodeStorage>]> {
                    pub(crate) fn new(context: Weak<Sandbox>, contents: Arc<[<$name NodeStorage>]>) ->
                    ConcreteNodeArc<[<$name NodeStorage>]> {
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

                impl Buildable for ConcreteNodeArc<[<$name NodeStorage>]> {
                    type Storage = [<$name NodeStorage>];
                }

                impl SandboxMemberBehavior for ConcreteNodeArc<[<$name NodeStorage>]> {
                    fn get_context(&self) -> Weak<Sandbox> {
                        self.common.context.clone()
                    }
                }

                impl TryFrom<AnyNodeArc> for ConcreteNodeArc<[<$name NodeStorage>]> {
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

                impl TryFrom<AnyNodeWeak> for ConcreteNodeWeak<[<$name NodeStorage>]> {
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

                impl From<ConcreteNodeArc<[<$name NodeStorage>]>> for AnyNodeArc {
                    fn from(concrete: ConcreteNodeArc<[<$name NodeStorage>]>) -> Self {
                        AnyNodeArc {
                            common: concrete.common,
                            contents: NodeContentsArc::$name(concrete.contents),
                        }
                    }
                }

                impl From<ConcreteNodeWeak<[<$name NodeStorage>]>> for AnyNodeWeak {
                    fn from(concrete: ConcreteNodeWeak<[<$name NodeStorage>]>) -> Self {
                        AnyNodeWeak {
                            common: concrete.common,
                            contents: NodeContentsWeak::$name(concrete.contents),
                        }
                    }
                }

                impl NodeBehavior for ConcreteNodeArc<[<$name NodeStorage>]> {
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
    pub fn create_text_node(&self, text: String) -> TextNodeArc {
        TextNodeArc::new(self.get_context(), Arc::new(TextNodeStorage { data: text }))
    }
}
