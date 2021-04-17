//! A virtual browser window.

use crate::{internal_prelude::*, node::contents::NodeContentsArc};

crate::use_behaviors!(sandbox_member);

/// A simulated window for static rendering
pub struct Window {
    context: SandboxMemberBehaviorStorage,
    // would be nice to have DocumentNode
    document: AnyNodeArc,
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<Window> {
        Arc::new_cyclic(|win_weak| -> Window {
            let document = AnyNodeArc::new(
                context.clone(),
                NodeContentsArc::Element(Arc::new(ElementNodeStorage::HtmlHtmlElement {
                    default_view: win_weak.clone(),
                })),
            );
            Window {
                context: SandboxMemberBehaviorStorage::new(context),
                document,
            }
        })
    }

    /// Gets the window's document
    // would be nice to have DocumentNode
    pub fn document(&self) -> AnyNodeArc {
        self.document.clone()
    }
}

impl_sandbox_member!(Window, context);
