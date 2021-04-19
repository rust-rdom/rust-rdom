#![cfg(test)]

use std::convert::TryInto;
use std::sync::Arc;

use crate::config::ScreenMetrics;
use crate::node::concrete::*;
use crate::node::contents::{AttributeNS, CommentNS, DocumentNS, NodeType, TextNS};
use crate::node::element::{ElementNS, HtmlButtonES, HtmlHtmlES};
use crate::node::NodeBehaviour;
use crate::sandbox::Sandbox;

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc: DocumentNode = sbox.clone().window().document().try_into().unwrap();
    let document_element = ElementNode::new(
        Arc::downgrade(&sbox),
        Arc::new(ElementNS::HtmlHtml(HtmlHtmlES {
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
        ElementNode,
        NodeType::Element,
        Arc::new(ElementNS::HtmlButton(HtmlButtonES))
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
        Arc::new(TextNS {
            data: "test".to_owned()
        })
    );

    let node = text.first_child().unwrap();
    let node: ConcreteNodeArc<TextNS> = node.try_into().unwrap();

    assert_eq!(node.contents.data().unwrap(), "test".to_owned());
}

#[test]
fn test_c_data_section_node_node() {
    let _cds = test_node_creation!(CDataSectionNode, NodeType::CDataSection, Default::default());
}

#[test]
fn test_processing_instruction_node() {
    let _pi = test_node_creation!(
        ProcessingInstructionNode,
        NodeType::ProcessingInstruction,
        Default::default()
    );
}

#[test]
fn test_comment_node() {
    let _com = test_node_creation!(
        CommentNode,
        NodeType::Comment,
        Arc::new(CommentNS {
            data: "test".to_owned()
        })
    );
}

#[test]
fn test_document_type_node() {
    let _dt = test_node_creation!(DocumentTypeNode, NodeType::DocumentType, Default::default());
}

#[test]
fn test_document_fragment_node() {
    let _frag = test_node_creation!(
        DocumentFragmentNode,
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
    let node = sbox.builder::<AttributeNS>().build(Default::default());
    let _: ConcreteNodeArc<AttributeNS> = node; // assert that we got an AttributeNode

    assert!(Weak::ptr_eq(&node.get_context(), &Arc::downgrade(&sbox)));
}
