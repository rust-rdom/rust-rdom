#![macro_use]

use std::sync::Weak;

use crate::internal_prelude::*;

pub trait SandboxMemberBehaviour {
    fn get_context(&self) -> Weak<Sandbox>;
}

pub struct SandboxMemberBehaviourStorage {
    context: Weak<Sandbox>,
}

impl SandboxMemberBehaviourStorage {
    pub fn new(context: Weak<Sandbox>) -> SandboxMemberBehaviourStorage {
        SandboxMemberBehaviourStorage { context }
    }
}

impl SandboxMemberBehaviour for SandboxMemberBehaviourStorage {
    fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
}

#[macro_export]
/// Implements SandBoxMemberBehaviour
macro_rules! impl_sandbox_member {
    ($structname: ident, $fieldname: ident) => {
        paste! {
            impl SandboxMemberBehaviour for $structname {
                fn get_context(&self) -> Weak<Sandbox> {
                    self.$fieldname.get_context()
                }
            }
        }
    };
}
