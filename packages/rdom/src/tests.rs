#![cfg(test)]

use std::convert::{TryFrom, TryInto};
use std::sync::{Arc, Weak};

use crate::node::concrete::*;
use crate::node::contents::{AttributeStore, CommentStore, NodeContentsWeak, NodeType, TextStore};
use crate::node::element::{
    ElementKind, ElementStore, HtmlBodyStore, HtmlButtonStore, HtmlElementStore, HtmlHtmlStore,
};
use crate::node::AnyNodeArc;
use crate::sandbox::Sandbox;
use crate::selector::Selector;
use crate::{
    config::ScreenMetrics,
    node::contents::{
        CDataSectionStore, DocumentFragmentStore, DocumentTypeStore, ProcessingInstructionStore,
    },
};

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.clone().window().document();

    let document_element =
        ConcreteNodeArc::<ElementStore>::new_cyclic(Arc::downgrade(&sbox), |node_weak| {
            ElementStore::new(
                ElementKind::HtmlElement(HtmlElementStore::HtmlHtml(HtmlHtmlStore)),
                Arc::downgrade(&sbox),
                node_weak.clone().into(),
            )
        })
        .into();

    let _text = doc.create_text_node("Hello, world!".to_string());
    doc.append_child(document_element);
    assert_eq!(doc.child_nodes().length(), 1);
}

macro_rules! test_node_creation {
    ($ty: ty, $node_type: expr, $storage: expr, $sbox: expr) => {{
        let sbox = $sbox;
        let doc = sbox.clone().window().document();
        let weak_sbox = Arc::downgrade(&sbox);

        let node = <$ty>::new_cyclic(weak_sbox, $storage);
        doc.append_child(node.into());
        assert_eq!(doc.child_nodes().length(), 1);
        assert_eq!(
            doc.first_child().unwrap().node_type(),
            $node_type.get_node_number()
        );

        doc
    }};
}

#[test]
fn test_element_node_m() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let _elem = test_node_creation!(
        ConcreteNodeArc<ElementStore>,
        NodeType::Element,
        |node_weak| ElementStore::new(
            ElementKind::HtmlElement(HtmlElementStore::HtmlButton(HtmlButtonStore)),
            Arc::downgrade(&sbox),
            node_weak.clone().into()
        ),
        sbox.clone()
    );
}

#[test]
fn test_attr_node() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let _doc = test_node_creation!(
        ConcreteNodeArc<AttributeStore>,
        NodeType::Attribute,
        |_node_weak| Default::default(),
        sbox
    );
}

#[test]
fn test_text_node() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let text = test_node_creation!(
        ConcreteNodeArc<TextStore>,
        NodeType::Text,
        |_node_weak| TextStore {
            data: "test".to_owned()
        },
        sbox
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
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let _cds = test_node_creation!(
        ConcreteNodeArc<CDataSectionStore>,
        NodeType::CDataSection,
        |_node_weak| Default::default(),
        sbox
    );
}

#[test]
fn test_processing_instruction_node() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let _pi = test_node_creation!(
        ConcreteNodeArc<ProcessingInstructionStore>,
        NodeType::ProcessingInstruction,
        |_node_weak| Default::default(),
        sbox
    );
}

#[test]
fn test_comment_node() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let _com = test_node_creation!(
        ConcreteNodeArc<CommentStore>,
        NodeType::Comment,
        |_node_weak| CommentStore {
            data: "test".to_owned()
        },
        sbox
    );
}

#[test]
fn test_document_type_node() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let _dt = test_node_creation!(
        ConcreteNodeArc<DocumentTypeStore>,
        NodeType::DocumentType,
        |_node_weak| Default::default(),
        sbox
    );
}

#[test]
fn test_document_fragment_node() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let _frag = test_node_creation!(
        ConcreteNodeArc<DocumentFragmentStore>,
        NodeType::DocumentFragment,
        |_node_weak| Default::default(),
        sbox
    );
}

#[test]
fn can_build_node() {
    use std::sync::Weak;

    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let node = sbox.builder::<AttributeNodeArc>().build(Default::default());
    let _: ConcreteNodeArc<AttributeStore> = node; // assert that we got an AttributeNode

    assert!(Weak::ptr_eq(&node.get_context(), &Arc::downgrade(&sbox)));
}

#[test]
fn tag_name() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);

    let button = sbox.builder::<ElementNodeArc>().build_button();
    let body = sbox.builder::<ElementNodeArc>().build_body();

    assert_eq!(button.contents.tag_name(), "BUTTON");
    assert_eq!(body.contents.tag_name(), "BODY");
}

#[test]
fn selector() {
    let sbox = Sandbox::new(Default::default());
    let sbox = Arc::downgrade(&sbox);

    let button = ElementNodeArc::new_cyclic(sbox.clone(), |node_weak| {
        ElementStore::new(
            ElementKind::HtmlElement(HtmlElementStore::HtmlButton(HtmlButtonStore)),
            sbox.clone(),
            node_weak.clone().into(),
        )
    });
    let body = ElementNodeArc::new_cyclic(sbox.clone(), |node_weak| {
        ElementStore::new(
            ElementKind::HtmlElement(HtmlElementStore::HtmlBody(HtmlBodyStore)),
            sbox.clone(),
            node_weak.clone().into(),
        )
    });

    let button_any: AnyNodeArc = button.clone().into();

    let selector = Selector::try_from("button".to_string()).unwrap();

    assert_eq!(selector.matches_selected_node(&button_any).is_some(), true);
    assert!(selector.is_selected_element(button));
    assert!(!selector.is_selected_element(body));
}

#[test]
fn cyclic_elements() {
    let sbox = Sandbox::new(Default::default());
    let sbox = Arc::downgrade(&sbox);

    let button = ElementNodeArc::new_cyclic(sbox.clone(), |node_weak| {
        ElementStore::new(
            ElementKind::HtmlElement(HtmlElementStore::HtmlButton(HtmlButtonStore)),
            sbox.clone(),
            node_weak.clone().into(),
        )
    });

    assert!(button
        .contents
        .node
        .common
        .ptr_eq(&Arc::downgrade(&button.common)));

    match &button.common.node_graph.node.contents {
        NodeContentsWeak::Element(el) => {
            assert!(Weak::ptr_eq(&el, &Arc::downgrade(&button.contents)));
        }
        _ => {
            assert!(false);
        }
    }

    match &button.contents.node.contents {
        NodeContentsWeak::Element(el) => {
            assert!(el.ptr_eq(&Arc::downgrade(&button.contents)));
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn query_selector() {
    let sbox_strong = Sandbox::new(Default::default());
    let sbox = Arc::downgrade(&sbox_strong);

    let button = ElementNodeArc::new_cyclic(sbox.clone(), |node_weak| {
        ElementStore::new(
            ElementKind::HtmlElement(HtmlElementStore::HtmlButton(HtmlButtonStore)),
            sbox.clone(),
            node_weak.clone().into(),
        )
    });
    let body = ElementNodeArc::new_cyclic(sbox.clone(), |node_weak| {
        ElementStore::new(
            ElementKind::HtmlElement(HtmlElementStore::HtmlBody(HtmlBodyStore)),
            sbox.clone(),
            node_weak.clone().into(),
        )
    });

    let buttonselector = Selector::try_from("BUTTON").unwrap();
    let bodyselector = Selector::try_from("BODY").unwrap();

    let doc = sbox_strong.window().document();

    doc.append_child(button.clone().into());
    doc.append_child(body.clone().into());

    let qbody = doc.query_selector(&bodyselector).unwrap().unwrap();
    let qbutton = doc.query_selector(&buttonselector).unwrap().unwrap();

    assert!(Arc::ptr_eq(&qbody.common, &body.common));
    assert!(Arc::ptr_eq(&qbutton.common, &button.common));
    assert_eq!(doc.child_element_count().unwrap(), 2);
}

#[test]
fn query_selector_child() {
    let sbox_strong = Sandbox::new(Default::default());
    let sbox = Arc::downgrade(&sbox_strong);

    let button = ElementNodeArc::new_cyclic(sbox.clone(), |node_weak| {
        ElementStore::new(
            ElementKind::HtmlElement(HtmlElementStore::HtmlButton(HtmlButtonStore)),
            sbox.clone(),
            node_weak.clone().into(),
        )
    });
    let body = ElementNodeArc::new_cyclic(sbox.clone(), |node_weak| {
        ElementStore::new(
            ElementKind::HtmlElement(HtmlElementStore::HtmlBody(HtmlBodyStore)),
            sbox.clone(),
            node_weak.clone().into(),
        )
    });

    let buttonselector = Selector::try_from("BUTTON").unwrap();
    let bodyselector = Selector::try_from("BODY").unwrap();

    let doc = sbox_strong.window().document();

    doc.append_child(body.clone().into());
    body.append_child(button.clone().into());

    let qbody = doc.query_selector(&bodyselector).unwrap().unwrap();
    let qbutton = doc.query_selector(&buttonselector).unwrap().unwrap();

    assert!(Arc::ptr_eq(&qbody.common, &body.common));
    assert!(Arc::ptr_eq(&qbutton.common, &button.common));
}
