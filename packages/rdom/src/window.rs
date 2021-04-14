//! A virtual browser window.

use crate::internal_prelude::*;
use crate::node::NodeContents;

crate::use_behaviors!(sandbox_member);

/// A simulated window for static rendering
pub struct Window {
    context: SandboxMemberBehaviorStorage,
    // would be nice to have DocumentNode
    document: Arc<Node>,
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<Window> {
        Arc::new_cyclic(|win_weak| -> Window {
            let document: Arc<Node> = Node::new(
                context.clone(),
                NodeContents::Element(
                    ConcreteElement::HtmlHtmlElement {
                        default_view: win_weak.clone(),
                    }
                ),
            );
            Window {
                context: SandboxMemberBehaviorStorage::new(context),
                document,
            }
        })
    }

    /// Gets the window's document
    // would be nice to have DocumentNode
    pub fn document(&self) -> Arc<Node> {
        self.document.clone()
    }
}

impl_sandbox_member!(Window, context);
