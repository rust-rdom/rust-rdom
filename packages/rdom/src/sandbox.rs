//! A sandbox represents a virtual browser tab. It contains a document and a window,
//! as well as some configuration information for screen dimensions.

use std::marker::PhantomData;

use crate::internal_prelude::*;

use crate::config::ScreenMetrics;
use crate::node::Buildable;
use crate::window::Window;

/// A Builder<R> is a machine which can be used to build nodes of reference type R.
pub struct Builder<R: Buildable> {
    pub(crate) sandbox: Weak<Sandbox>,
    _phantom: PhantomData<R>,
}

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
        Arc::new_cyclic(|sandbox_weak| -> Sandbox {
            let win = Window::new(sandbox_weak.clone());
            Sandbox {
                screen_metrics,
                window: win,
            }
        })
    }

    /// Gets the root window object
    pub fn window(&self) -> Arc<Window> {
        // Window is safe to unwrap, as it's only None during initialization.
        // This will be fixable when arc_new_cyclic is stable.
        self.window.clone()
    }

    /// Creates a builder for a specific type of node reference
    pub fn builder<T: Buildable>(self: &Arc<Self>) -> Builder<T> {
        Builder {
            sandbox: Arc::downgrade(self),
            _phantom: PhantomData,
        }
    }
}
