//! The public interface to static rendering functionality.

#[macro_use]
extern crate downcast_rs;

use crate::config::ScreenMetrics;
use crate::window::Window;

mod config;
mod error;
mod node;
mod sandbox;
mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
