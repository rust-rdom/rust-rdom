//! The public interface to static rendering functionality.

#[macro_use]
extern crate downcast_rs;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

use crate::config::ScreenMetrics;
use crate::document::Document;
use crate::window::Window;

mod config;
mod document;
mod element;
mod error;
mod sandbox;
mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
