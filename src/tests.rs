#![cfg(test)]

use std::sync::Arc;

use crate::behavior::node::NodeBehavior;
use crate::config::ScreenMetrics;
use crate::node::{self, element::HtmlHtmlElement};
use crate::sandbox::Sandbox;

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

#[test]
fn can_build_node() {
    use crate::behavior::sandbox_member::SandboxMemberBehavior;
    use std::sync::Weak;

    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let node = sbox.builder::<node::AttrNode>().build();
    let _: Arc<node::AttrNode> = node; // assert that we got an AttrNode

    assert!(Weak::ptr_eq(&node.get_context(), &Arc::downgrade(&sbox)));
}
