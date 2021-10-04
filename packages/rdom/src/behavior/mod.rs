pub(crate) use self::parent_node::ParentNodeBehavior;
pub(crate) use super::node::NodeBehavior;

/// Macro for generating preludes and enforcing mod names
macro_rules! generate_preludes {
    ($($modname: ident $traitname: ident),*) => {
        paste::paste! {
            $(
                pub mod [<$modname _prelude>] {
                    pub(crate) use super::$modname::{
                        [<$traitname Behavior>],
                        [<$traitname BehaviorStorage>]
                    };

                    pub(crate) use crate::[<impl_ $modname>];
                }
            )*
        }
    };
}

/// Provides use statements for behaviors
#[macro_export]
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
mod event_target;

generate_preludes! {
    parent_node ParentNode
}

pub mod node;
pub mod node_prelude {
    pub(crate) use super::node::NodeBehavior;
}
