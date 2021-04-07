//! Provides support for behaviors as defined in the DOM standards. Because the standards
//! refer extensively to classes and mixins, and because Rust does not support either one,
//! this module provides several structures that provide the same behavior but in a Rust-
//! friendly way (using composition instead of inheritance).
//!
//! Every behavior is implemented using traits, which can be dependent on one another.
//! Every behavior has a **BehaviorName**Behavior and **BehaviorName**BehaviorStorage and
//! implementing macro.

/// Macro for generating preludes and enforcing mod names
macro_rules! generate_preludes {
    ($($modname: ident $traitname: ident),*) => {
        paste::paste! {
            $(
                pub mod [<$modname _prelude>] {
                    pub use super::$modname::{
                        [<$traitname Behavior>],
                        [<$traitname BehaviorStorage>]
                    };

                    pub use crate::[<impl_ $modname>];
                }
            )*
        }
    };
}

pub mod node;
pub mod sandbox_member;

generate_preludes! {
    node Node,
    sandbox_member SandboxMember
}
