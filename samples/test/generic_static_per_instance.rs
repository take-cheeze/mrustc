// compile-flags: --edition 2021
// A trait's provided constant that borrows another associated constant (`Self::R.is_some()`) promotes the borrow to
// a generic static: its value is per implementation (mold 3, `Target::SUPPORTS_TLSDESC`).
trait Target {
    const R: Option<u32> = None;
    const HAS_R: bool = Self::R.is_some();
    const R_OR_ZERO: u32 = match &Self::R { Some(v) => *v, None => 0 };
}
struct X;
struct Y;
impl Target for X { const R: Option<u32> = Some(36); }
impl Target for Y {}
fn report<E: Target>() -> (bool, u32) { (E::HAS_R, E::R_OR_ZERO) }
fn main() {
    assert_eq!(report::<X>(), (true, 36));
    assert_eq!(report::<Y>(), (false, 0));
}
