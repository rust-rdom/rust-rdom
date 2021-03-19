//! A sandbox represents a virtual browser tab. It contains a document and a window,
//! as well as some configuration information for screen dimensions.

use once_cell::sync::OnceCell;
use std::marker::Sync;
use std::sync::Arc;

use crate::config::ScreenMetrics;
use crate::node::Document;
use crate::window::Window;

/// A sandbox represents a virtual browser tab. It contains a document and a window,
/// as well as some configuration information for screen dimensions.
#[derive(Clone)]
pub struct Sandbox {
    screen_metrics: ScreenMetrics,
    document: Option<Arc<Document>>,
    window: Option<Arc<Window>>,
}

impl Sandbox {
    fn new(screen_metrics: ScreenMetrics) -> Arc<Sandbox> {
        let mut sandbox = Arc::new(Sandbox {
            screen_metrics,
            document: None,
            window: None,
        });
        let sandbox_weak = Arc::downgrade(&sandbox);

        let win = Arc::new(Window::new(sandbox_weak.clone()));
        let doc = Arc::new(Document::new(sandbox_weak.clone()));

        let mut sbox = Arc::get_mut(&mut sandbox).expect("Could not construct sandbox");
        (*sbox).window = Some(win);
        (*sbox).document = Some(doc);

        sandbox
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn does_compile() {
        struct Foo {}
    }
}
