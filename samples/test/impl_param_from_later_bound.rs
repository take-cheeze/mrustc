// compile-flags: --edition 2021
// An impl parameter fixed only by a later bound (`U` in `F: Fn(T) -> U`), used in an earlier bound's trait
// parameters (`C: Sink<U::Item>`): the impl must still be found when it is instantiated (rayon 1.12,
// `FlatMapIterConsumer` in `par_chunks(..).flat_map_iter(..).collect()`).
trait Sink<T> { fn put(&mut self, t: T); }
trait Consumer<T> { fn feed(&mut self, t: T); }
struct VecSink(Vec<u8>);
impl Sink<u8> for VecSink { fn put(&mut self, t: u8) { self.0.push(t) } }
struct FlatMap<C, F> { base: C, f: F }
impl<T, U, C, F> Consumer<T> for FlatMap<C, F>
where
    C: Sink<U::Item>,
    F: Fn(T) -> U,
    U: IntoIterator,
{
    fn feed(&mut self, t: T) {
        for x in (self.f)(t) { self.base.put(x); }
    }
}
fn drive<T, C: Consumer<T>>(items: Vec<T>, mut c: C) -> C {
    struct Callback<C> { consumer: C }
    trait Run<T> { fn run(self, items: Vec<T>) -> Self; }
    impl<T, C: Consumer<T>> Run<T> for Callback<C> {
        fn run(mut self, items: Vec<T>) -> Self { for i in items { self.consumer.feed(i); } self }
    }
    c = Callback { consumer: c }.run(items).consumer;
    c
}
fn main() {
    let data = [1u8, 2, 3];
    let out = drive(data.chunks(2).collect(), FlatMap { base: VecSink(Vec::new()), f: |s: &[u8]| (s.len() as u16).to_le_bytes() });
    assert_eq!(out.base.0, [2, 0, 1, 0]);
}
