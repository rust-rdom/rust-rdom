//! A virtual browser window.

use crate::{
    internal_prelude::*,
    node::{concrete::DocumentNodeArc, contents::DocumentStore},
};

crate::use_behaviors!(sandbox_member);

/// A simulated window for static rendering
pub struct Window {
    context: SandboxMemberBehaviorStorage,
    document: DocumentNodeArc,
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<Window> {
        Arc::new_cyclic(|win_weak| {
            let document = DocumentNodeArc::new(
                context.clone(),
                Arc::new(DocumentStore {
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
    pub fn document(&self) -> DocumentNodeArc {
        self.document.clone()
    }
}

impl_sandbox_member!(Window, context);
