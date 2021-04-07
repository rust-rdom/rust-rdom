#![macro_use]

pub trait ParentBehavior {}
pub struct ParentBehaviorStorage;

#[macro_export]
/// Implements ParentBehavior
macro_rules! impl_parent {
    ($structname: ident, $fieldname: ident) => {
        paste::paste! {
            impl ParentBehavior for $structname {}
        }
    };
}
