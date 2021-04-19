//! A virtual browser window.

use crate::{
    internal_prelude::*,
    node::{concrete::DocumentNode, contents::DocumentNodeStorage},
};

crate::use_behaviors!(sandbox_member);

/// A simulated window for static rendering
pub struct Window {
    context: SandboxMemberBehaviorStorage,
    document: DocumentNode,
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<Window> {
        Arc::new_cyclic(|win_weak| {
            let document = DocumentNode::new(
                context.clone(),
                Arc::new(DocumentNodeStorage {
                    default_view: win_weak.clone(),
                }),
            );
            Window {
                context: SandboxMemberBehaviorStorage::new(context),
                document,
            }
        })
    }

    /// Gets the window's document
    // would be nice to have DocumentNode
    pub(crate) fn document(&self) -> DocumentNode {
        self.document.clone()
    }
}

impl_sandbox_member!(Window, context);
