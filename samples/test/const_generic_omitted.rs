// compile-flags: --edition 2021
// Omitted const generic arguments. In a type (`S<u8>`) a defaulted const argument takes its default; in a struct
// pattern or literal with no arguments (`S { .. }`) every argument is inferred (rayon 1.12, `SplitProducer`).
struct S<T, const INCL: bool = false> { x: T, n: usize }
impl<T: Copy + Into<u64>, const INCL: bool> S<T, INCL> {
    fn f(self) -> u64 {
        let S { x, n } = self;
        let y = S { x, n: n + 1 };
        let s2: S<T, INCL> = y;
        x.into() + s2.n as u64 + if INCL { 100 } else { 0 }
    }
}
enum K<const B: bool> { One(u8) }
impl<const B: bool> K<B> {
    fn get(self) -> u8 { let K::One(v) = self; if B { v * 2 } else { v } }
}
fn main() {
    let a: S<u8> = S { x: 3, n: 4 };
    let b: S<u8, true> = S { x: 3, n: 4 };
    assert_eq!(a.f(), 8);
    assert_eq!(b.f(), 108);
    assert_eq!(K::<true>::One(4).get(), 8);
    let k: K<false> = K::One(4);
    assert_eq!(k.get(), 4);
}
