// The 128-bit multiply of mrustc's C prelude when the C compiler has no 128-bit type (emulated_i128): the low 128 bits
// of the product, and the overflow flag (the high 128 bits not zero).
#[inline(never)]
fn id<T>(v: T) -> T { v }

fn main() {
    let a: u128 = id(0xFFFF_FFFF_FFFF_FFFF_1234_5678_9ABC_DEF0);
    let b: u128 = id(0x0000_0000_0000_0001_0000_0000_0000_0003);
    assert_eq!(a.wrapping_mul(b), 0x1234_5678_9abc_deed_369d_0369_d036_9cd0);
    let c: u128 = id(0xDEAD_BEEF_CAFE_BABE_0123_4567_89AB_CDEF);
    let d: u128 = id(0x0FED_CBA9_8765_4321);
    assert_eq!(c.wrapping_mul(d), 0xfa3b_db11_bf83_49f2_2223_6d88_fe56_18cf);
    assert_eq!(c.checked_mul(d), None);
    assert_eq!(id(u64::MAX as u128) * id(u64::MAX as u128), 0xffff_ffff_ffff_fffe_0000_0000_0000_0001);
    // A bit of the multiplicand shifted out: overflows although the low 128 bits are zero.
    assert_eq!(id(1u128 << 100).checked_mul(id(1u128 << 28)), None);
    assert_eq!(id(1u128 << 64).checked_mul(id(1u128 << 63)), Some(1u128 << 127));
    assert_eq!(id(-3i128) * id(5i128), -15);
    assert_eq!(id(i128::MIN).wrapping_mul(id(-1i128)), i128::MIN);
    println!("ok");
}
