//! Behavior implemented by all the structs that have
//! references to sandbox

use crate::internal_prelude::*;
use crate::node::template::{TemplateArc, TemplateWeak};

/// Behavior implemented by all the structs that have
/// references to sandbox
pub trait SandboxMemberBehavior {
    /// Gets weak refernce to the sandbox
    fn get_context(&self) -> Weak<Sandbox>;

    /// Builds with the `TemplateWeak`
    fn buildw<T>(&self, template: impl TemplateWeak<T>) -> T {
        template.build(self.get_context())
    }

    /// Builds with the `TemplateArc`
    fn build<T>(&self, template: impl TemplateArc<T>) -> Result<T, DomError> {
        match self.get_context().upgrade() {
            Some(context) => Ok(template.build(context)),
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
