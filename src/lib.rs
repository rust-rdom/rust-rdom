//! A Rust-based simulated DOM (browser-independent replacement for web_sys)

#![deny(
    missing_docs,
    // missing_debug_implementations,
    // missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

#[macro_use]
extern crate downcast_rs;

use crate::config::ScreenMetrics;
use crate::window::Window;

pub mod sandbox;
pub mod behavior;
pub mod config;
pub mod error;
pub mod node;
pub mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
