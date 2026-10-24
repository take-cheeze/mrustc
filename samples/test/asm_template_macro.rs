// compile-flags: --edition 2021
// `asm!` template pieces built by macros (`concat!(...)`) and the x86 operand modifiers `x` (16 bits) and `e`
// (portable-atomic 1.15).
#[cfg(target_arch = "x86_64")]
fn main() {
    use std::arch::asm;
    macro_rules! add_reg {
        ($dst:tt) => {{
            let mut out: u64 = 5;
            unsafe { asm!(concat!("add ", $dst, ", 3"), inout("rax") out); }
            out
        }};
    }
    assert_eq!(add_reg!("rax"), 8);
    let mut v: u16 = 0x1234;
    let one: u16 = 1;
    unsafe { asm!("add {0:x}, {1:x}", inout(reg) v, in(reg) one); }
    assert_eq!(v, 0x1235);
    let mut w: u32 = 7;
    unsafe { asm!("shl {0:e}, 1", inout(reg) w); }
    assert_eq!(w, 14);
}
#[cfg(not(target_arch = "x86_64"))]
fn main() {}
