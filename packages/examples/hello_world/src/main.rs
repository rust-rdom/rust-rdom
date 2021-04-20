use std::sync::Arc;

use rdom::config::ScreenMetrics;
use rdom::node::{AnyNodeArc, NodeBehavior};
use rdom::node::concrete::*;
use rdom::node::contents::{
    CommentNodeStorage, NodeType, TextNodeStorage,
};
use rdom::node::element::{ElementNodeStorage, HtmlButtonElementStorage, HtmlHtmlElementStorage};
use rdom::sandbox::Sandbox;

fn main() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.clone().window().document();
    // let document_element = ElementNodeArc::new(
    //     Arc::downgrade(&sbox),
    //     Arc::new(ElementNodeStorage::HtmlHtml(HtmlHtmlElementStorage {
    //         default_view: Arc::downgrade(&sbox.window()),
    //     })),
    // )
    // .into();
    // let _text = doc.create_text_node("Hello, world!".to_string());
    // doc.append_child(document_element);
    // assert_eq!(doc.child_nodes().length(), 1);
}
