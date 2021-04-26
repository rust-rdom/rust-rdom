//! Data and functionality specific to each node type live here.

use crate::internal_prelude::*;
use crate::window::Window;

pub use super::element::ElementStore;

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
                    $name(Arc<[<$name Store>]>),
                )*
            }

            #[derive(Clone)]
            pub(crate) enum NodeContentsWeak {
                $(
                    $name(Weak<[<$name Store>]>),
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
                impl From<&Arc<[<$name Store>]>> for NodeContentsWeak {
                    fn from(source: &Arc<[<$name Store>]>) -> NodeContentsWeak {
                        NodeContentsWeak::$name(Arc::downgrade(source))
                    }
                }
            )*
        }
    };
}

/// Storage type for DocumentNode
#[derive(Default, Clone)]
pub struct DocumentStore {
    /// Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

/// Storage type for TextNode
#[derive(Default, Clone)]
pub struct TextStore {
    /// Text in the text node
    pub(crate) data: String,
}

impl TextStore {
    // TODO data should come from CharacterData

    /// Gives the text contents of the text node
    pub fn data(&self) -> Option<String> {
        Some(self.data.clone())
    }
}

/// Storage type for CommentNode
#[derive(Default, Clone)]
pub struct CommentStore {
    /// Text in the comment node
    pub(crate) data: String,
}

impl CommentStore {
    // TODO data should come from CharacterData

    /// Gives the text contents of the text node
    pub fn data(&self) -> Option<String> {
        Some(self.data.clone())
    }
}

/// Storage type for AttributeNode
#[derive(Default, Clone)]
pub struct AttributeStore;

/// Storage type for CDataSectionNode
#[derive(Default, Clone)]
pub struct CDataSectionStore;

/// Storage type for ProcessingInstructionNode
#[derive(Default, Clone)]
pub struct ProcessingInstructionStore;

/// Storage type for DocumentTypeNode
#[derive(Default, Clone)]
pub struct DocumentTypeStore;

/// Storage type for DocumentFragmentNode
#[derive(Default, Clone)]
pub struct DocumentFragmentStore;

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
