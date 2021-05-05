//! A virtual browser window.

use crate::{
    internal_prelude::*,
    node::{concrete::DocumentNodeArc, contents::DocumentStore},
};

crate::use_behaviors!(sandbox_member);

#[sourcegen(generator = "window")]
// Generated. All manual edits to the block annotated with #[sourcegen...] will be discarded.
///A simulated window for static rendering
///A simulated window for static rendering
#[derive(Clone)]
pub struct Window {
    document: DocumentNodeArc,
    context: Weak<Sandbox>,
}

#[sourcegen::generated]
impl Window {
    ///gets `Weak<Sandbox>` to the `Sandbox` that it is in
    pub fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
}

#[sourcegen::generated]
impl SandboxMemberBehavior for Window {
    fn get_context(&self) -> Weak<Sandbox> {
        self.get_context()
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
