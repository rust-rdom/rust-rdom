#![cfg(test)]

use std::sync::Arc;

use crate::behavior::node::NodeBehavior;
use crate::config::ScreenMetrics;
use crate::node::element::HtmlHtmlElement;
use crate::node::{ AnyNode,
    AttrNode, CDataSectionNode, CommentNode, Document, DocumentFragmentNode, DocumentTypeNode,
    ElementNode, NodeType, ProcessingInstructionNode, TextNode,
};
use crate::node::TextNodeStorage;

use crate::sandbox::Sandbox;
use paste::paste;

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.clone().window().document();
    let document_element = HtmlHtmlElement::new(Arc::downgrade(&sbox), ());
    let _text = doc.create_text_node("Hello, world!".to_string());
    doc.append_child(document_element);
    assert_eq!(doc.child_nodes().length(), 1);
}

macro_rules! test_node_creation {
    ($ty: ty, $node_type: expr, $storage: expr) => {{
        let metrics: ScreenMetrics = Default::default();
        let sbox = Sandbox::new(metrics);
        let doc = sbox.clone().window().document();
        let weak_sbox = Arc::downgrade(&sbox);

        let node = <$ty>::new(weak_sbox, $storage);
        doc.append_child(node);
        assert_eq!(doc.child_nodes().length(), 1);
        assert_eq!(
            doc.first_child().unwrap().get_node_type(),
            $node_type as isize
        );

        doc
    }};
}

#[test]
fn test_element_node_m() {
    let _doc = test_node_creation!(ElementNode, NodeType::Element, ());
}

#[test]
fn test_attr_node() {
    let _doc = test_node_creation!(AttrNode, NodeType::Attribute, ());
}

#[test]
fn test_text_node() {
    let _doc = test_node_creation!(TextNode, NodeType::Text, TextNodeStorage {text: "test".to_owned()});

    let node = _doc.first_child().unwrap();
    let node = AnyNode::as_any(node.as_ref()).downcast_ref::<TextNode>().unwrap();

    assert_eq!(node.get_text().unwrap(), "test".to_owned());
}

#[test]
fn test_c_data_section_node_node() {
    let _doc = test_node_creation!(CDataSectionNode, NodeType::CDataSection, ());
}

#[test]
fn test_processing_instruction_node() {
    let _doc = test_node_creation!(ProcessingInstructionNode, NodeType::ProcessingInstruction, ());
}

#[test]
fn test_comment_node() {
    let _doc = test_node_creation!(CommentNode, NodeType::Comment, TextNodeStorage {text: "test".to_owned()});
}

#[test]
fn test_document_type_node() {
    let _doc = test_node_creation!(DocumentTypeNode, NodeType::DocumentType, ());
}

#[test]
fn test_document_fragment_node() {
    let _doc = test_node_creation!(DocumentFragmentNode, NodeType::DocumentFragment, ());
}

