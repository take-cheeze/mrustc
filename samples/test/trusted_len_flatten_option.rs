// compile-flags: --edition 2021 -O
// `Flatten` over `Option`s is not `TrustedLen` (only arrays have a constant size), so `collect` must not take the
// `TrustedLen` path: it asserted `TrustedLen iterator's size hint is not exact` (mold 3, `sort_dynsyms`).
fn main() {
    let v: Vec<Option<u32>> = vec![None, Some(1), None, Some(2), Some(3)];
    let a: Vec<u32> = v[1..].iter().flatten().copied().collect();
    assert_eq!(a, [1, 2, 3]);
    let c: Vec<u32> = [[1u32, 2], [3, 4]].iter().flatten().copied().collect();
    assert_eq!(c, [1, 2, 3, 4]);
}
