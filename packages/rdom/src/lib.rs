//! A Rust-based simulated DOM (browser-independent replacement for web_sys)

#![feature(arc_new_cyclic)]
#![deny(
    missing_docs,
    // missing_debug_implementations,
    // missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    // unstable_features, someday :)
    unused_qualifications
)]

pub(crate) mod behavior;
pub mod config;
pub mod error;
// pub mod closure;
pub mod function;
pub(crate) mod internal_prelude;
pub mod named_node_map;

pub mod node;
pub mod node_list;
pub mod sandbox;
mod selector;
pub mod tests;
pub mod window;
