// compile-flags: --edition 2021
// Inherent methods with a `self: &mut &'a mut Self` receiver (`<[T]>::split_off_mut`, Rust 1.87; mold 3).
fn main() {
    let mut buf = [1u8, 2, 3, 4, 5, 6];
    let mut rest = &mut buf[..];
    let a = rest.split_off_mut(..2).unwrap();
    a[0] = 9;
    let b = rest.split_off_first_mut().unwrap();
    *b = 7;
    assert_eq!(rest, [4, 5, 6]);
    let mut r2: &[u8] = &[10, 20, 30];
    let c = r2.split_off(1..).unwrap();
    assert_eq!((c, r2), (&[20u8, 30][..], &[10u8][..]));
    let mut all = [0u8; 4];
    let mut rest2 = &mut all[..];
    let parts: Vec<&mut [u8]> = [1usize, 2].iter().map(|&n| rest2.split_off_mut(..n).unwrap()).collect();
    assert_eq!(parts.len(), 2);
    assert_eq!(buf, [9, 2, 7, 4, 5, 6]);
}
