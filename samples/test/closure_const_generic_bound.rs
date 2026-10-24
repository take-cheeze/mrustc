// compile-flags: --edition 2021
// A closure in an impl over a const parameter keeps the impl's bounds on types that only use that parameter
// (`Class<IS_64>: ElfClass`), so a trait method called in the closure finds its impl (mold 3, the LoongArch target).
pub struct Class<const IS_64: bool>;
pub trait ElfClass { const SIZE: usize; }
impl ElfClass for Class<false> { const SIZE: usize = 4; }
impl ElfClass for Class<true> { const SIZE: usize = 8; }
pub trait Target: Sized + 'static { type Rel: RelRecord; }
pub trait RelRecord { fn r_type(&self) -> u32; }
pub struct ElfRela<E: Target>(u32, std::marker::PhantomData<E>);
impl<E: Target> RelRecord for ElfRela<E> { fn r_type(&self) -> u32 { self.0 } }
pub struct Arch<const IS_64: bool>;
impl<const IS_64: bool> Target for Arch<IS_64> where Class<IS_64>: ElfClass { type Rel = ElfRela<Self>; }
impl<const IS_64: bool> Arch<IS_64> where Class<IS_64>: ElfClass {
    fn next_is(rels: &[<Self as Target>::Rel], i: usize, ty: u32) -> bool {
        rels.get(i + 1).is_some_and(|r| r.r_type() == ty)
    }
    fn word() -> usize { <Class<IS_64> as ElfClass>::SIZE }
}
fn main() {
    let rels = [ElfRela::<Arch<true>>(1, std::marker::PhantomData), ElfRela(7, std::marker::PhantomData)];
    assert!(Arch::<true>::next_is(&rels, 0, 7));
    assert!(!Arch::<true>::next_is(&rels, 1, 7));
    assert_eq!(Arch::<true>::word(), 8);
    assert_eq!(Arch::<false>::word(), 4);
}
