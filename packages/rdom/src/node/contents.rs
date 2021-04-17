use super::{DocumentNodeStorage, TextNodeStorage};
use crate::internal_prelude::*;

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
    pub(crate) fn to_node_type(&self) -> isize {
        match self {
            NodeContentsArc::Element(_) => 1,
            NodeContentsArc::Attribute => 2,
            NodeContentsArc::Text(_) => 3,
            NodeContentsArc::CDataSection => 4,
            NodeContentsArc::ProcessingInstruction => 5,
            NodeContentsArc::Comment => 6,
            NodeContentsArc::Document(_) => 7,
            NodeContentsArc::DocumentType => 8,
            NodeContentsArc::DocumentFragment => 9,
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
    pub(crate) fn to_node_type(&self) -> isize {
        match self {
            NodeContentsWeak::Element(_) => 1,
            NodeContentsWeak::Attribute => 2,
            NodeContentsWeak::Text(_) => 3,
            NodeContentsWeak::CDataSection => 4,
            NodeContentsWeak::ProcessingInstruction => 5,
            NodeContentsWeak::Comment => 6,
            NodeContentsWeak::Document(_) => 7,
            NodeContentsWeak::DocumentType => 8,
            NodeContentsWeak::DocumentFragment => 9,
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
