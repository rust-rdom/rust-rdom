use std::sync::Weak;

use rdom::config::ScreenMetrics;
use rdom::node::concrete::TextNodeArc;
use rdom::node::contents::TextStore;
use rdom::node::template::HtmlHtmlTemplate;
use rdom::node::NodeBehavior;
use rdom::sandbox::Sandbox;
use rdom::{behavior::sandbox_member::SandboxMemberBehavior, node::concrete::ElementNodeArc};

// Sorry, I had too much haskell today
// This function returns a template for html with a text node
fn html_text(text: String) -> impl FnOnce(Weak<Sandbox>) -> ElementNodeArc {
    |sbox| {
        let html = sbox.buildw(HtmlHtmlTemplate);

        let text = sbox.buildw(TextStore::new(text));

        html.append_child(text.into());

        html
    }
}

fn main() {
    let metrics: ScreenMetrics = Default::default();
    let sbox = Sandbox::new(metrics);
    let doc = sbox.clone().window().document();

    let document_element = doc.buildw(html_text("Hello, world!".to_string()));

    doc.append_child(document_element.clone().into());
    assert_eq!(doc.child_nodes().length(), 1);
    assert_eq!(document_element.child_nodes().length(), 1);
}
