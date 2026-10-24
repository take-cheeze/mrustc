// A constant's borrowed value is lifted to a static that only a generic function uses: the crate that defines it
// must still emit it, as the function is instantiated in another crate (mold 3, `RelRecord::is_func_call`).
const KINDS: &[u32] = &[4, 9];
pub trait Rel { fn r_type(&self) -> u32; }
pub fn is_call<R: Rel>(r: &R) -> bool { KINDS.contains(&r.r_type()) }
// The same for a trait impl's associated constant, evaluated on demand (`R_FUNCALL: &'static [u32] = &[..]`).
pub trait Target { const R_FUNCALL: &'static [u32]; }
pub struct X86;
impl Target for X86 { const R_FUNCALL: &'static [u32] = &[4, 9]; }
pub fn is_func_call<E: Target, R: Rel>(r: &R) -> bool { E::R_FUNCALL.contains(&r.r_type()) }
