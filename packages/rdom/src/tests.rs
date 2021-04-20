#![cfg(test)]

use std::convert::TryInto;
use std::sync::Arc;

use crate::config::ScreenMetrics;
use crate::node::concrete::*;
use crate::node::contents::{CommentNodeStorage, NodeType, TextNodeStorage};
use crate::node::element::{ElementNodeStorage, HtmlButtonElementStorage, HtmlHtmlElementStorage};
use crate::node::{AnyNodeArc, NodeBehavior};
use crate::sandbox::Sandbox;

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.clone().window().document();
    let document_element = ElementNodeArc::new(
        Arc::downgrade(&sbox),
        Arc::new(ElementNodeStorage::HtmlHtml(HtmlHtmlElementStorage {
            default_view: Arc::downgrade(&sbox.window()),
        })),
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
        ElementNodeArc,
        NodeType::Element,
        Arc::new(ElementNodeStorage::HtmlButton(HtmlButtonElementStorage))
    );
}

#[test]
fn test_attr_node() {
    let _doc = test_node_creation!(AttributeNodeArc, NodeType::Attribute, Default::default());
}

#[test]
fn test_text_node() {
    let text = test_node_creation!(
        TextNodeArc,
        NodeType::Text,
        Arc::new(TextNodeStorage {
            data: "test".to_owned()
        })
    );

    let node = text.first_child().unwrap();
    let node: Result<TextNodeArc, AnyNodeArc> = node.try_into();
    match node {
        Ok(node) => {
            assert_eq!(node.contents.data().unwrap(), "test".to_owned());
        }
        _ => {
            panic!("Could not cast node");
        }
    }
}

#[test]
fn test_c_data_section_node_node() {
    let _cds = test_node_creation!(
        CDataSectionNodeArc,
        NodeType::CDataSection,
        Default::default()
    );
}

#[test]
fn test_processing_instruction_node() {
    let _pi = test_node_creation!(
        ProcessingInstructionNodeArc,
        NodeType::ProcessingInstruction,
        Default::default()
    );
}

#[test]
fn test_comment_node() {
    let _com = test_node_creation!(
        CommentNodeArc,
        NodeType::Comment,
        Arc::new(CommentNodeStorage {
            data: "test".to_owned()
        })
    );
}

#[test]
fn test_document_type_node() {
    let _dt = test_node_creation!(
        DocumentTypeNodeArc,
        NodeType::DocumentType,
        Default::default()
    );
}

#[test]
fn test_document_fragment_node() {
    let _frag = test_node_creation!(
        DocumentFragmentNodeArc,
        NodeType::DocumentFragment,
        Default::default()
    );
}

#[test]
fn can_build_node() {
    use crate::behavior::sandbox_member::SandboxMemberBehavior;
    use std::sync::Weak;

    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let node = sbox.builder::<AttributeNodeArc>().build(Default::default());
    let _: AttributeNodeArc = node; // assert that we got an AttributeNode

    assert!(Weak::ptr_eq(&node.get_context(), &Arc::downgrade(&sbox)));
}
