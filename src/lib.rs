//! The public interface to static rendering functionality.

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

use crate::document::Document;
use crate::window::Window;
use crate::config::ScreenMetrics;

mod config;
mod document;
mod element;
mod error;
mod window;
mod sandbox;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
