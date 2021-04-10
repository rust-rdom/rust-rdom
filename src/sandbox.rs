//! A sandbox represents a virtual browser tab. It contains a document and a window,
//! as well as some configuration information for screen dimensions.

use crate::internal_prelude::*;

use crate::config::ScreenMetrics;
use crate::node::{self, element};
use crate::window::Window;

#[derive(PartialEq, Eq)]
pub(crate) enum BuildableNode {
    AttrNode,
    TextNode,
    DocumentNode,
    HtmlHtmlElement,
    HtmlBodyElement,
    HtmlButtonElement,
}

/// A sandbox represents a virtual browser tab. It contains a document and a window,
/// as well as some configuration information for screen dimensions.
#[derive(Clone)]
pub struct Sandbox {
    screen_metrics: ScreenMetrics,
    window: Arc<Window>,
}

impl Sandbox {
    /// Creates a new sandbox (enclosure of a single DOM context)
    pub fn new(screen_metrics: ScreenMetrics) -> Arc<Sandbox> {
        Arc::new_cyclic(|sandbox_weak| -> Sandbox {
            let win = Window::new(sandbox_weak.clone());
            Sandbox {
                screen_metrics,
                window: win,
            }
        })
    }

    /// Gets the root window object
    pub fn window(&self) -> Arc<Window> {
        // Window is safe to unwrap, as it's only None during initialization.
        // This will be fixable when arc_new_cyclic is stable.
        self.window.clone()
    }

    pub(crate) fn build_node<const N: BuildableNode>(self: Arc<Self>) -> Arc<dyn AnyNode> {
        let sbox = Arc::downgrade(&self);

        match N {
            BuildableNode::AttrNode => node::AttrNode::new(sbox, Default::default()),
            BuildableNode::TextNode => node::TextNode::new(sbox, Default::default()),
            BuildableNode::DocumentNode => node::DocumentNode::new(sbox, Default::default()),
            BuildableNode::HtmlHtmlElement => {
                element::HtmlHtmlElement::new(sbox, Default::default())
            }
            BuildableNode::HtmlBodyElement => {
                element::HtmlBodyElement::new(sbox, Default::default())
            }
            BuildableNode::HtmlButtonElement => {
                element::HtmlButtonElement::new(sbox, Default::default())
            }
        }
    }
}
