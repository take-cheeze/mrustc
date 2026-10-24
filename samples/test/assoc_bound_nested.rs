// compile-flags: --edition 2021
// Associated type bounds nested in an associated type bound (`I: Tr<Item: Tr2<Item: Copy>>`, rayon 1.12).
fn total<I>(outer: I) -> u64
where
    I: IntoIterator<Item: IntoIterator<Item: Copy + Into<u64>>>,
{
    let mut t = 0;
    for inner in outer {
        for x in inner {
            t += x.into();
        }
    }
    t
}
fn main() {
    assert_eq!(total(vec![vec![1u8, 2], vec![3]]), 6);
    assert_eq!(total([[4u32; 2]; 2]), 16);
}
