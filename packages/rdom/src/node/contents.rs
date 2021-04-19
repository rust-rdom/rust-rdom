use super::{DocumentNodeStorage, TextNodeStorage, NodeCommon, NodeGraphStorage};
use super::concrete::ConcreteNodeArc;
use crate::sandbox::Builder;
use crate::internal_prelude::*;

macro_rules! declare_contents {
    ($($ti:expr => $name:ident),*) => {
        paste::paste! {
            pub(crate) enum NodeType {
                $(
                    $name,
                )*
            }

            impl NodeType {
                pub(crate) fn get_node_number(&self) -> isize {
                    match self {
                        $(
                            NodeType::$name => $ti,
                        )*
                    }
                }
            }

            /// Node type, as defined in https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
            #[derive(Clone)]
            pub(crate) enum NodeContentsArc {
                $(
                    $name(Arc<[<$name NodeStorage>]>),
                )*
            }

            #[derive(Clone)]
            pub(crate) enum NodeContentsWeak {
                $(
                    $name(Weak<[<$name NodeStorage>]>),
                )*
            }

            impl NodeContentsArc {
                pub(crate) fn to_node_type(&self) -> NodeType {
                    match self {
                        $(
                            NodeContentsArc::$name(_) => NodeType::$name,
                        )*
                    }
                }

                pub(crate) fn downgrade(&self) -> NodeContentsWeak {
                    match self {
                        $(
                            NodeContentsArc::$name(strong) => NodeContentsWeak::$name(Arc::downgrade(&strong)),
                        )*
                    }
                }
            }

            impl NodeContentsWeak {
                pub(crate) fn to_node_type(&self) -> NodeType {
                    match self {
                        $(
                            NodeContentsWeak::$name(_) => NodeType::$name,
                        )*
                    }
                }

                pub(crate) fn upgrade(&self) -> Option<NodeContentsArc> {
                    match self {
                        $(
                            NodeContentsWeak::$name(weak) => Some(NodeContentsArc::$name(weak.upgrade()?)),
                        )*
                    }
                }
            }

            $(
                impl Builder<[<$name NodeStorage>]> {
                    pub fn build(&self, storage: [<$name NodeStorage>]) -> ConcreteNodeArc<[<$name NodeStorage>]> {
                        ConcreteNodeArc::<[<$name NodeStorage>]>::new(self.sandbox.clone(), Arc::new(storage))
                    }
                }

                impl From<&Arc<[<$name NodeStorage>]>> for NodeContentsWeak {
                    fn from(source: &Arc<[<$name NodeStorage>]>) -> NodeContentsWeak {
                        NodeContentsWeak::$name(Arc::downgrade(source))
                    }
                }
            )*
        }
    };
}

#[derive(Default, Clone)]
pub(crate) struct AttributeNodeStorage;
#[derive(Default, Clone)]
pub(crate) struct CDataSectionNodeStorage;
#[derive(Default, Clone)]
pub(crate) struct ProcessingInstructionNodeStorage;
#[derive(Default, Clone)]
pub(crate) struct CommentNodeStorage;
#[derive(Default, Clone)]
pub(crate) struct DocumentTypeNodeStorage;
#[derive(Default, Clone)]
pub(crate) struct DocumentFragmentNodeStorage;

declare_contents! {
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
