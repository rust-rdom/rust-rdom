// use variadic_closure::{Closure as VariadicClosure};

//! Analogous to wasm_bindgen::closure::Closure

/// Analogous to wasm_bindgen::closure::Closure
pub(crate) struct Closure/*<T: ?Sized>*/ {

}

trait WasmClosure {}

impl Closure<T>
// where
//     T: ?Sized + WasmClosure
{
    fn wrap(mut data: Box<Function<T>>) {

    }
}
