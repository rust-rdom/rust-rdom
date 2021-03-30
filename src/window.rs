//! A virtual browser window.

use std::sync::Weak;
use std::sync::Arc;

use crate::sandbox::Sandbox;
use crate::node::raw::{Document, DocumentStorage};

/// A simulated window for static rendering
pub struct Window {
    context: Weak<Sandbox>,
    document: Arc<Document>
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<Window> {
        let window = Arc::new_cyclic(|win_weak| -> Window {
            let mut document: Arc<Document> = Document::new(context.clone(), DocumentStorage {
                default_view: win_weak.clone()
            });
            Window { context, document }
        });

        window
    }

    /// Gets the window's document
    pub fn document(&self) -> Arc<Document> {
        self.document.clone()
    }
}
