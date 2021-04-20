//! Data and functionality specific to each node type live here.

use super::concrete::ConcreteNodeArc;
use crate::internal_prelude::*;
use crate::sandbox::Builder;
use crate::window::Window;

pub use super::element::ElementNodeStorage;

macro_rules! declare_contents {
    ($($ti:expr => $name:ident),*) => {
        paste::paste! {
            pub enum NodeType {
                $(
                    $name,
                )*
            }

            impl NodeType {
                fn get_node_number(&self) -> isize {
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
                impl From<&Arc<[<$name NodeStorage>]>> for NodeContentsWeak {
                    fn from(source: &Arc<[<$name NodeStorage>]>) -> NodeContentsWeak {
                        NodeContentsWeak::$name(Arc::downgrade(source))
                    }
                }
            )*
        }
    };
}

macro_rules! impl_standard_builder {
    ($($name:ident),*) => {
        paste::paste! {
            $(
                impl Builder<ConcreteNodeArc<[<$name NodeStorage>]>> {
                    pub fn build(&self, storage: [<$name NodeStorage>]) -> ConcreteNodeArc<[<$name NodeStorage>]> {
                        ConcreteNodeArc::<[<$name NodeStorage>]>::new(self.sandbox.clone(), Arc::new(storage))
                    }
                }
            )*
        }
    };
}

#[derive(Default, Clone)]
pub struct DocumentNodeStorage {
    /// Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

#[derive(Default, Clone)]
pub struct TextNodeStorage {
    /// Text in the text node
    pub(crate) data: String,
}

impl TextNodeStorage {
    // TODO data should come from CharacterData

    /// Gives the text contents of the text node
    pub fn data(&self) -> Option<String> {
        Some(self.data.clone())
    }
}

#[derive(Default, Clone)]
pub struct CommentNodeStorage {
    /// Text in the comment node
    pub(crate) data: String,
}

impl CommentNodeStorage {
    // TODO data should come from CharacterData

    /// Gives the text contents of the text node
    pub fn data(&self) -> Option<String> {
        Some(self.data.clone())
    }
}

#[derive(Default, Clone)]
pub struct AttributeNodeStorage;
#[derive(Default, Clone)]
pub struct CDataSectionNodeStorage;
#[derive(Default, Clone)]
pub struct ProcessingInstructionNodeStorage;
#[derive(Default, Clone)]
pub struct DocumentTypeNodeStorage;
#[derive(Default, Clone)]
pub struct DocumentFragmentNodeStorage;

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

impl_standard_builder! {
    Attribute,
    Text,
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment
}
