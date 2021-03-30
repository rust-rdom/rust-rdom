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
    window: Arc<Window>,
}

impl Sandbox {
    /// Creates a new sandbox (enclosure of a single DOM context)
    pub fn new(screen_metrics: ScreenMetrics) -> Arc<Sandbox> {
        let sandbox = Arc::new_cyclic(|sandbox_weak| -> Sandbox {
            let win = Window::new(sandbox_weak.clone());
            Sandbox {
                screen_metrics,
                window: win
            }
        });

        sandbox
    }

    /// Gets the root window object
    pub fn window(&self) -> Arc<Window> {
        // Window is safe to unwrap, as it's only None during initialization.
        // This will be fixable when arc_new_cyclic is stable.
        self.window.clone()
    }
}