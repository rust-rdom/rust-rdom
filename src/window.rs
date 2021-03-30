//! A virtual browser window.

use std::sync::Arc;
use std::sync::Weak;

use crate::node::raw::{Document, DocumentStorage};
use crate::sandbox::Sandbox;

/// A simulated window for static rendering
pub struct Window {
    context: Weak<Sandbox>,
    document: Arc<Document>,
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<Window> {
        let window = Arc::new_cyclic(|win_weak| -> Window {
            let document: Arc<Document> = Document::new(
                context.clone(),
                DocumentStorage {
                    default_view: win_weak.clone(),
                },
            );
            Window { context, document }
        });

        window
    }

    /// Gets the window's document
    pub fn document(&self) -> Arc<Document> {
        self.document.clone()
    }
}
