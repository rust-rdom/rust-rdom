//! A virtual browser window.

use crate::internal_prelude::*;

crate::use_behaviors!(sandbox_member);
use crate::node::{DocumentNode, DocumentNodeStorage};

/// A simulated window for static rendering
#[derive(Debug)]
pub struct Window {
    context: SandboxMemberBehaviorStorage,
    document: Arc<DocumentNode>,
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<Window> {
        Arc::new_cyclic(|win_weak| -> Window {
            let document: Arc<DocumentNode> = DocumentNode::new(
                context.clone(),
                DocumentNodeStorage {
                    default_view: win_weak.clone(),
                },
            );
            Window {
                context: SandboxMemberBehaviorStorage::new(context),
                document,
            }
        })
    }

    /// Gets the window's document
    pub fn document(&self) -> Arc<DocumentNode> {
        self.document.clone()
    }
}

impl_sandbox_member!(Window, context);
