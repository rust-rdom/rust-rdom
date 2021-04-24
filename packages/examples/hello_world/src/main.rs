use std::sync::Arc;

use rdom::config::ScreenMetrics;
use rdom::node::concrete::*;
use rdom::node::NodeBehavior;
use rdom::sandbox::Sandbox;

fn main() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.clone().window().document();

    let document_element = sbox.builder::<ElementNodeArc>().build_html();
    let _text = doc.create_text_node("Hello, world!".to_string());
    doc.append_child(document_element.into());
    assert_eq!(doc.child_nodes().length(), 1);
}
