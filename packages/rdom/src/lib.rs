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

#[macro_use]
extern crate downcast_rs;

pub(crate) mod behavior;
pub mod config;
pub mod error;
pub(crate) mod internal_prelude;
pub mod named_node_map;
pub mod nice;
pub mod node;
pub mod node_list;
pub mod sandbox;
pub mod tests;
pub mod window;

#[rdom_macro::declare_node]
// this const is just required to group the syntaxes of struct and impl together
const _: () = {
    #[derive(Node)]
    #[core = "Button"]
    pub struct HtmlButtonElement {
        foo: i32
    }

    // Change this to Foobar to make it compile; it should compile without doing that,
    // once everything about the declare_node macro works
    impl HtmlButtonElement {
        fn do_something() {
        }
    }
};

// struct Blah(HtmlButtonElement);
