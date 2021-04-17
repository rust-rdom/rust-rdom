use super::{DocumentNodeStorage, TextNodeStorage};
use crate::internal_prelude::*;

pub(crate) enum NodeType {
    Element,
    Attribute,
    Text,
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
}

impl NodeType {
    pub(crate) fn get_node_number(&self) -> isize {
        match self {
            NodeType::Element => 1,
            NodeType::Attribute => 2,
            NodeType::Text => 3,
            NodeType::CDataSection => 4,
            NodeType::ProcessingInstruction => 5,
            NodeType::Comment => 6,
            NodeType::Document => 7,
            NodeType::DocumentType => 8,
            NodeType::DocumentFragment => 9,
        }
    }
}

/// Node type, as defined in https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
#[derive(Clone)]
pub(crate) enum NodeContentsArc {
    Element(Arc<ConcreteElement>),
    Attribute,
    Text(Arc<TextNodeStorage>),
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document(Arc<DocumentNodeStorage>),
    DocumentType,
    DocumentFragment,
}

#[derive(Clone)]
pub(crate) enum NodeContentsWeak {
    Element(Weak<ConcreteElement>),
    Attribute,
    Text(Weak<TextNodeStorage>),
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document(Weak<DocumentNodeStorage>),
    DocumentType,
    DocumentFragment,
}

impl NodeContentsArc {
    pub(crate) fn to_node_type(&self) -> NodeType {
        match self {
            NodeContentsArc::Element(_) => NodeType::Element,
            NodeContentsArc::Attribute => NodeType::Attribute,
            NodeContentsArc::Text(_) => NodeType::Text,
            NodeContentsArc::CDataSection => NodeType::CDataSection,
            NodeContentsArc::ProcessingInstruction => NodeType::ProcessingInstruction,
            NodeContentsArc::Comment => NodeType::Comment,
            NodeContentsArc::Document(_) => NodeType::Document,
            NodeContentsArc::DocumentType => NodeType::DocumentType,
            NodeContentsArc::DocumentFragment => NodeType::DocumentFragment,
        }
    }

    pub(crate) fn downgrade(&self) -> NodeContentsWeak {
        match self {
            NodeContentsArc::Element(strong) => NodeContentsWeak::Element(Arc::downgrade(&strong)),
            NodeContentsArc::Attribute => NodeContentsWeak::Attribute,
            NodeContentsArc::Text(strong) => NodeContentsWeak::Text(Arc::downgrade(&strong)),
            NodeContentsArc::CDataSection => NodeContentsWeak::CDataSection,
            NodeContentsArc::ProcessingInstruction => NodeContentsWeak::ProcessingInstruction,
            NodeContentsArc::Comment => NodeContentsWeak::Comment,
            NodeContentsArc::Document(strong) => {
                NodeContentsWeak::Document(Arc::downgrade(&strong))
            }
            NodeContentsArc::DocumentType => NodeContentsWeak::DocumentType,
            NodeContentsArc::DocumentFragment => NodeContentsWeak::DocumentFragment,
        }
    }
}

impl NodeContentsWeak {
    pub(crate) fn to_node_type(&self) -> NodeType {
        match self {
            NodeContentsWeak::Element(_) => NodeType::Element,
            NodeContentsWeak::Attribute => NodeType::Attribute,
            NodeContentsWeak::Text(_) => NodeType::Text,
            NodeContentsWeak::CDataSection => NodeType::CDataSection,
            NodeContentsWeak::ProcessingInstruction => NodeType::ProcessingInstruction,
            NodeContentsWeak::Comment => NodeType::Comment,
            NodeContentsWeak::Document(_) => NodeType::Document,
            NodeContentsWeak::DocumentType => NodeType::DocumentType,
            NodeContentsWeak::DocumentFragment => NodeType::DocumentFragment,
        }
    }

    pub(crate) fn upgrade(&self) -> Option<NodeContentsArc> {
        Some(match self {
            NodeContentsWeak::Element(weak) => NodeContentsArc::Element(weak.upgrade()?),
            NodeContentsWeak::Attribute => NodeContentsArc::Attribute,
            NodeContentsWeak::Text(weak) => NodeContentsArc::Text(weak.upgrade()?),
            NodeContentsWeak::CDataSection => NodeContentsArc::CDataSection,
            NodeContentsWeak::ProcessingInstruction => NodeContentsArc::ProcessingInstruction,
            NodeContentsWeak::Comment => NodeContentsArc::Comment,
            NodeContentsWeak::Document(weak) => NodeContentsArc::Document(weak.upgrade()?),
            NodeContentsWeak::DocumentType => NodeContentsArc::DocumentType,
            NodeContentsWeak::DocumentFragment => NodeContentsArc::DocumentFragment,
        })
    }
}
