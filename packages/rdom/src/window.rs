//! A virtual browser window.

use crate::{
    internal_prelude::*,
    node::{concrete::DocumentNodeArc, contents::DocumentStore},
};

crate::use_behaviors!(sandbox_member);

/// A simulated window for static rendering
#[sourcegen::sourcegen(generator = "sandbox-member")]
// Generated. All manual edits to the block annotated with #[sourcegen...] will be discarded.
pub struct Window {
    // generated
    context: Weak<Sandbox>,

    document: DocumentNodeArc,
}

// generated
impl SandboxMemberBehavior for Window {
    fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
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
            Window { context, document }
        })
    }

    /// Gets the window's document
    // would be nice to have DocumentNode
    pub fn document(&self) -> DocumentNodeArc {
        self.document.clone()
    }
}
