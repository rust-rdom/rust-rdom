#![cfg(test)]

use std::sync::Arc;

use crate::config::ScreenMetrics;
use crate::node::concrete::{AttributeNode, ConcreteNodeArc, DocumentNode, ElementNode, TextNode};
use crate::node::contents::NodeType;
use crate::node::element::ElementNodeStorage;
use crate::node::TextNodeStorage;

use crate::node::DocumentNodeStorage;
use crate::node::NodeBehaviour;
use crate::sandbox::Sandbox;
use quote::quote;
use std::convert::TryInto;

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc: DocumentNode = sbox.clone().window().document().try_into().unwrap();
    let document_element = ElementNode::new(
        Arc::downgrade(&sbox),
        Arc::new(ElementNodeStorage::HtmlHtmlElement {
            default_view: Arc::downgrade(&sbox.window()),
        }),
    )
    .into();
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
        doc.append_child(node.into());
        assert_eq!(doc.child_nodes().length(), 1);
        assert_eq!(
            doc.first_child().unwrap().get_node_type(),
            $node_type.get_node_number()
        );

        doc
    }};
}

#[test]
fn test_element_node_m() {
    let _elem = test_node_creation!(
        ElementNode,
        NodeType::Element,
        Arc::new(ElementNodeStorage::HtmlButtonElement)
    );
}

#[test]
fn test_attr_node() {
    let _doc = test_node_creation!(AttributeNode, NodeType::Attribute, Default::default());
}

#[test]
fn test_text_node() {
    let text = test_node_creation!(
        TextNode,
        NodeType::Text,
        Arc::new(TextNodeStorage {
            text: "test".to_owned()
        })
    );

    let node = text.first_child().unwrap();
    let node: ConcreteNodeArc<TextNodeStorage> = node.try_into().unwrap();

    assert_eq!(node.contents.get_text().unwrap(), "test".to_owned());
}

// #[test]
// fn test_c_data_section_node_node() {
//     let _doc = test_node_creation!(CDataSectionNode, NodeType::CDataSection, ());
// }

// #[test]
// fn test_processing_instruction_node() {
//     let _doc = test_node_creation!(ProcessingInstructionNode, NodeType::ProcessingInstruction, ());
// }

// #[test]
// fn test_comment_node() {
//     let _doc = test_node_creation!(CommentNode, NodeType::Comment, TextNodeStorage {text: "test".to_owned()});
// }

// #[test]
// fn test_document_type_node() {
//     let _doc = test_node_creation!(DocumentTypeNode, NodeType::DocumentType, ());
// }

// #[test]
// fn test_document_fragment_node() {
//     let _doc = test_node_creation!(DocumentFragmentNode, NodeType::DocumentFragment, ());
// }

// #[test]
// fn can_build_node() {
//     use crate::behavior::sandbox_member::SandboxMemberBehavior;
//     use std::sync::Weak;

//     let metrics: ScreenMetrics = Default::default();
//     let sbox = Sandbox::new(metrics);
//     let node = sbox.builder::<AttrNode>().build();
//     let _: Arc<AttrNode> = node; // assert that we got an AttrNode

//     assert!(Weak::ptr_eq(&node.get_context(), &Arc::downgrade(&sbox)));
// }
