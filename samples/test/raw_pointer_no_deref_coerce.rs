// compile-flags: --edition 2021
// A raw pointer never deref-coerces: in `ptr_mut(&mut wrapper)` passed as `*mut Inner`, `B` is `Inner` and the
// `&mut Wrapper` argument deref-coerces instead (zstd-safe 7.2, `ptr_mut(&mut output)`).
use std::ops::{Deref, DerefMut};
struct Inner { v: u32 }
struct Wrapper<'a, C: ?Sized> { buf: Inner, _c: &'a C }
impl<'a, C: ?Sized> Deref for Wrapper<'a, C> { type Target = Inner; fn deref(&self) -> &Inner { &self.buf } }
impl<'a, C: ?Sized> DerefMut for Wrapper<'a, C> { fn deref_mut(&mut self) -> &mut Inner { &mut self.buf } }
fn ptr_mut<B>(p: &mut B) -> *mut B { p as *mut B }
unsafe fn set(p: *mut Inner) { unsafe { (*p).v = 42; } }
fn go<C: ?Sized>(c: &C) -> u32 {
    let mut w = Wrapper { buf: Inner { v: 1 }, _c: c };
    unsafe { set(ptr_mut(&mut w)) };
    w.buf.v
}
fn main() {
    assert_eq!(go("x"), 42);
}
