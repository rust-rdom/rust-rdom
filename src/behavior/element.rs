#![macro_use]

pub trait ElementBehavior {}
pub struct ElementBehaviorStorage;

#[macro_export]
/// Implements ElementBehavior
macro_rules! impl_element {
    ($structname: ident, $fieldname: ident) => {
        paste! {
            impl ElementBehavior for $structname {}
        }
    };
}
