#![cfg(test)]

use std::sync::Arc;

use crate::config::ScreenMetrics;
use crate::sandbox::Sandbox;
use crate::node::raw::element::{HtmlHtmlElement, HtmlBodyElement};
use crate::node::raw::AnyRawNode;

#[test]
fn it_works() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let mut doc = sbox.clone().window().document();
    let document_element = HtmlHtmlElement::new(Arc::downgrade(&sbox), ());
    let text = doc.create_text_node("Hello, world!".to_string());
    doc.append_child(document_element);
    assert_eq!(doc.child_nodes().length(), 1);
}
