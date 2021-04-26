//! Behavior implemented by all the structs that have
//! references to sandbox

use crate::{internal_prelude::*, node::template::Template};

/// Behavior implemented by all the structs that have
/// references to sandbox
pub trait SandboxMemberBehavior {
    /// Gets weak refernce to the sandbox
    fn get_context(&self) -> Weak<Sandbox>;

    /// Builds the template
    fn build<T>(&self, template: impl Template<T>) -> Result<T, DomError> {
        match self.get_context().upgrade() {
            Some(sbox) => Ok(template.build(sbox)),
            None => Err(DomError::SandboxDropped),
        }
    }
}

// PROPOSITION: remove this
/// Storage for sandbox member
pub struct SandboxMemberBehaviorStorage {
    context: Weak<Sandbox>,
}

impl SandboxMemberBehaviorStorage {
    /// Constructor
    pub fn new(context: Weak<Sandbox>) -> SandboxMemberBehaviorStorage {
        SandboxMemberBehaviorStorage { context }
    }
}

impl SandboxMemberBehavior for SandboxMemberBehaviorStorage {
    fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
}

// PROPOSITION: remove this
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
