use once_cell::sync::OnceCell;
use std::marker::Sync;
use std::sync::{Arc, Mutex};

use crate::config::ScreenMetrics;
use crate::document::Document;
use crate::window::Window;

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

        let mut win = Arc::new(Window::new(sandbox_weak.clone()));
        let mut doc = Arc::new(Document::new(sandbox_weak.clone()));

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
