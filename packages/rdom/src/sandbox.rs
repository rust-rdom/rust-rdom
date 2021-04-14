//! A sandbox represents a virtual browser tab. It contains a document and a window,
//! as well as some configuration information for screen dimensions.

use std::marker::PhantomData;

use crate::internal_prelude::*;

use crate::config::ScreenMetrics;
use crate::node::{self, element};
use crate::window::Window;

// pub(crate) struct Builder<T: AnyNode> {
//     pub(crate) sandbox: Weak<Sandbox>,
//     _phantom: PhantomData<T>,
// }
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

    // pub(crate) fn builder<T: AnyNode>(self: &Arc<Self>) -> Builder<T> {
    //     Builder {
    //         sandbox: Arc::downgrade(self),
    //         _phantom: PhantomData,
    //     }
    // }
}
