//! Data and functionality specific to each node type live here.

use super::concrete::*;
use crate::internal_prelude::*;
use crate::sandbox::Builder;
use crate::window::Window;

pub use super::element::ElementNS;
/// Marker trait implemented by all node storage classes
pub trait AnyNS {}

macro_rules! declare_contents {
    ($($ti:expr => $name:ident),*) => {
        paste::paste! {
            /// Specifies the type of the node.
            /// See https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
            pub enum NodeType {
                $(
                    #[doc = "A node type corresponding to " $name " nodes"]
                    $name,
                )*
            }

            impl NodeType {
                /// Returns the number corresponding to the node type per
                /// https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType#node_type_constants
                pub fn get_node_number(&self) -> isize {
                    match self {
                        $(
                            NodeType::$name => $ti,
                        )*
                    }
                }
            }

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
                impl From<&Arc<[<$name NS>]>> for NodeContentsWeak {
                    fn from(source: &Arc<[<$name NS>]>) -> NodeContentsWeak {
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
                impl Builder<[<$name NodeArc>]> {
                    #[doc = "Builds a new " $name " node with the given storage value"]
                    pub fn build(&self, storage: [<$name NS>]) -> [<$name NodeArc>] {
                        ConcreteNodeArc::<[<$name NS>]>::new(self.sandbox.clone(), Arc::new(storage))
                    }
                }
            )*
        }
    };
}

/// Storage type for DocumentNode
#[derive(Default, Clone)]
pub struct DocumentNS {
    /// Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

/// Storage type for TextNode
#[derive(Default, Clone)]
pub struct TextNS {
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

/// Storage type for CommentNode
#[derive(Default, Clone)]
pub struct CommentNS {
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

/// Storage type for AttributeNode
#[derive(Default, Clone)]
pub struct AttributeNS;

/// Storage type for CDataSectionNode
#[derive(Default, Clone)]
pub struct CDataSectionNS;

/// Storage type for ProcessingInstructionNode
#[derive(Default, Clone)]
pub struct ProcessingInstructionNS;

/// Storage type for DocumentTypeNode
#[derive(Default, Clone)]
pub struct DocumentTypeNS;

/// Storage type for DocumentFragmentNode
#[derive(Default, Clone)]
pub struct DocumentFragmentNS;

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
