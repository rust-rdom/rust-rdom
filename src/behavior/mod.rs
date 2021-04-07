//! Provides support for behaviors as defined in the DOM standards. Because the standards
//! refer extensively to classes and mixins, and because Rust does not support either one,
//! this module provides several structures that provide the same behavior but in a Rust-
//! friendly way (using composition instead of inheritance).
//!
//! Every behavior is implemented using traits, which can be dependent on one another.
//! Every behavior has a **BehaviorName**Behavior and **BehaviorName**BehaviorStorage and
//! implementing macro.

// vscode snippet for behavior files
// "add_behavior": {
//     "prefix": "add_behavior",
//     "description": "adds template code to behavior file",
//     "body": [
//         "#![macro_use]",
//         "",
//         "pub trait $2Behavior {}",
//         "pub struct $2BehaviorStorage;",
//         "",
//         "#[macro_export]",
//         "/// Implements $2Behavior",
//         "macro_rules! impl_$1 {",
//         "    (\\$structname: ident, \\$fieldname: ident) => {",
//         "        paste::paste! {",
//         "            impl $2Behavior for \\$structname {}",
//         "        }",
//         "    };",
//         "}"
//     ]
// }

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

#[macro_export]
/// Provides use statements for behaviors
macro_rules! use_behaviors {
    ($($name:ident),*) => {
        paste::paste!{
            $(
                use crate::behavior::[<$name _prelude>]::*;
            )*
        }
    }
}

// need to write mod name twice because of
// macro importing
pub mod element;
pub mod node;
pub mod parent;
pub mod sandbox_member;

generate_preludes! {
    node Node,
    sandbox_member SandboxMember,
    parent Parent,
    element Element
}
