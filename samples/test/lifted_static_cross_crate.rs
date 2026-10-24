// aux-build: lifted_static_crate.rs --crate-type rlib
// compile-flags: --edition 2021
// A static lifted from a constant in another crate, used through a generic function instantiated here.
extern crate lifted_static_crate;
struct A(u32);
impl lifted_static_crate::Rel for A { fn r_type(&self) -> u32 { self.0 } }
fn main() {
    assert!(lifted_static_crate::is_call(&A(9)));
    assert!(!lifted_static_crate::is_call(&A(3)));
    assert!(lifted_static_crate::is_func_call::<lifted_static_crate::X86, _>(&A(4)));
    assert!(!lifted_static_crate::is_func_call::<lifted_static_crate::X86, _>(&A(5)));
}
