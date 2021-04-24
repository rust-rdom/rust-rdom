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

pub mod element;
pub mod parent_node;
pub mod sandbox_member;

generate_preludes! {
    sandbox_member SandboxMember,
    parent_node ParentNode
}

pub mod node;
pub mod node_prelude {
    pub use super::node::NodeBehavior;
}
