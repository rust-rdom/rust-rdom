use super::concrete::ConcreteNodeArc;
use super::{NodeCommon, NodeGraphStorage};
use crate::internal_prelude::*;
use crate::sandbox::Builder;
use crate::window::Window;

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
                    $name(Arc<[<$name NS>]>),
                )*
            }

            #[derive(Clone)]
            pub(crate) enum NodeContentsWeak {
                $(
                    $name(Weak<[<$name NS>]>),
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
                impl Builder<[<$name NS>]> {
                    pub fn build(&self, storage: [<$name NS>]) -> ConcreteNodeArc<[<$name NS>]> {
                        ConcreteNodeArc::<[<$name NS>]>::new(self.sandbox.clone(), Arc::new(storage))
                    }
                }

                impl From<&Arc<[<$name NS>]>> for NodeContentsWeak {
                    fn from(source: &Arc<[<$name NS>]>) -> NodeContentsWeak {
                        NodeContentsWeak::$name(Arc::downgrade(source))
                    }
                }
            )*
        }
    };
}

#[derive(Default, Clone)]
pub(crate) struct DocumentNS {
    /// Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

#[derive(Default, Clone)]
pub(crate) struct TextNS {
    /// Text in the text node
    pub(crate) data: String,
}

impl TextNS {
    // TODO data should come from CharacterData

    /// Gives the text contents of the text node
    pub fn data(&self) -> Option<String> {
        Some(self.data.clone())
    }
}

#[derive(Default, Clone)]
pub(crate) struct CommentNS {
    /// Text in the comment node
    pub(crate) data: String,
}

impl CommentNS {
    // TODO data should come from CharacterData

    /// Gives the text contents of the text node
    pub fn data(&self) -> Option<String> {
        Some(self.data.clone())
    }
}

#[derive(Default, Clone)]
pub(crate) struct AttributeNS;
#[derive(Default, Clone)]
pub(crate) struct CDataSectionNS;
#[derive(Default, Clone)]
pub(crate) struct ProcessingInstructionNS;
#[derive(Default, Clone)]
pub(crate) struct DocumentTypeNS;
#[derive(Default, Clone)]
pub(crate) struct DocumentFragmentNS;

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
