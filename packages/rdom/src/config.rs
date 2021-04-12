//! Configuration for a sandbox. A sandbox represents a virtual browser tab, so
//! in order to allow some values (e.g. window.innerWidth), we need a configuration
//! for the tab, which is what the structures in this module represent.

/// Screen metrics configuration (width and height)
#[derive(Clone, Debug)]
pub struct ScreenMetrics {
    inner_width: u16,
    inner_height: u16,
}

impl Default for ScreenMetrics {
    fn default() -> ScreenMetrics {
        ScreenMetrics {
            inner_width: 1500,
            inner_height: 1000,
        }
    }
}
