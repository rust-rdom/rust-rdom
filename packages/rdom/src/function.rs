//! Analogous to js_sys::Function

use variadic_closure::{Closure as VariadicClosure};

/// Analogous to js_sys::Function
pub(crate) struct Function<A, B, C, D, E, F, G, H, RET> {
    code: VariadicClosure<A, B, C, D, E, F, G, H, RET>
}

impl<A, B, C, D, E, F, G, H, RET> Function<A, B, C, D, E, F, G, H, RET> {
    pub fn new(code: VariadicClosure<A, B, C, D, E, F, G, H, RET>) -> Function<A, B, C, D, E, F, G, H, RET> {
        Function {
            code
        }
    }
}
