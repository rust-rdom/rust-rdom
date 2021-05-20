#![macro_use]

pub(crate) trait ElementBehavior {}
pub struct ElementBehaviorStorage;

/// Implements ElementBehavior
#[macro_export]
macro_rules! impl_element {
    ($structname: ident, $fieldname: ident) => {
        paste::paste! {
            impl ElementBehavior for $structname {}
        }
    };
}
