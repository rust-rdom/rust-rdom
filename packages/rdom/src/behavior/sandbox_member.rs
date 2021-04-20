use std::sync::Weak;

use crate::{internal_prelude::*, node::template::Template};

pub trait SandboxMemberBehavior {
    fn get_context(&self) -> Weak<Sandbox>;

    fn build<T>(&self, template: impl Template<T>) -> T {
        template.build(self.get_context())
    }
}

pub struct SandboxMemberBehaviorStorage {
    context: Weak<Sandbox>,
}

impl SandboxMemberBehaviorStorage {
    pub fn new(context: Weak<Sandbox>) -> SandboxMemberBehaviorStorage {
        SandboxMemberBehaviorStorage { context }
    }
}

impl SandboxMemberBehavior for SandboxMemberBehaviorStorage {
    fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
}

#[macro_export]
/// Implements SandBoxMemberBehavior
macro_rules! impl_sandbox_member {
    ($structname: ident, $fieldname: ident) => {
        paste::paste! {
            impl SandboxMemberBehavior for $structname {
                fn get_context(&self) -> Weak<Sandbox> {
                    self.$fieldname.get_context()
                }
            }
        }
    };
}
