// compile-flags: --edition 2021
// An inherent method on `E<T, T>` must not be picked for a receiver whose two parameters are still unknown
// (the arms of a `match` that are not unified yet): rustc picks `Iterator::map` here (either 1.18, rayon 1.12).
// When both parameters are the same type, the inherent method still applies.
enum E<L, R> { A(L), B(R) }
impl<T> E<T, T> {
    fn map<M, F: FnOnce(T) -> M>(self, f: F) -> E<M, M> {
        match self { E::A(x) => E::A(f(x)), E::B(x) => E::B(f(x)) }
    }
}
impl<L: Iterator, R: Iterator<Item = L::Item>> Iterator for E<L, R> {
    type Item = L::Item;
    fn next(&mut self) -> Option<L::Item> {
        match self { E::A(l) => l.next(), E::B(r) => r.next() }
    }
}
fn sum(flag: bool, v: &Vec<u32>) -> u32 {
    let it = match flag {
        true => E::A(v.iter()),
        false => E::B(v.iter().skip(1)),
    };
    it.map(|x| *x).sum()
}
fn same(flag: bool) -> u32 {
    let e = if flag { E::A(1u32) } else { E::B(2u32) };
    match e.map(|x| x * 10) { E::A(v) => v, E::B(v) => v + 1 }
}
fn main() {
    let v = vec![1, 2, 3];
    assert_eq!(sum(true, &v), 6);
    assert_eq!(sum(false, &v), 5);
    assert_eq!(same(true), 10);
    assert_eq!(same(false), 21);
}
