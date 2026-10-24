// compile-flags: --edition 2021
// A trait method's generic can have another name in the impl (`fn fold_with<F>` / `fn fold_with<G>`); a bound on it
// must still match the impl's own bound (rayon 1.12, `MapProducer::fold_with`).
trait Folder<Item>: Sized {
    type Result;
    fn consume(self, item: Item) -> Self;
    fn complete(self) -> Self::Result;
}
trait Producer: Sized {
    type Item;
    fn items(self) -> Vec<Self::Item>;
    fn fold_with<F>(self, folder: F) -> F where F: Folder<Self::Item> {
        let mut f = folder;
        for i in self.items() { f = f.consume(i); }
        f
    }
}
struct MapFolder<'f, C, F> { base: C, map_op: &'f F }
impl<'f, T, R, C, F> Folder<T> for MapFolder<'f, C, F> where C: Folder<F::Output>, F: Fn(T) -> R {
    type Result = C::Result;
    fn consume(self, item: T) -> Self {
        let mapped = (self.map_op)(item);
        MapFolder { base: self.base.consume(mapped), map_op: self.map_op }
    }
    fn complete(self) -> C::Result { self.base.complete() }
}
struct MapProducer<'f, P, F> { base: P, map_op: &'f F }
impl<'f, P, F, R> Producer for MapProducer<'f, P, F> where P: Producer, F: Fn(P::Item) -> R {
    type Item = R;
    fn items(self) -> Vec<R> { self.base.items().into_iter().map(self.map_op).collect() }
    fn fold_with<G>(self, folder: G) -> G where G: Folder<Self::Item> {
        let folder1 = MapFolder { base: folder, map_op: self.map_op };
        self.base.fold_with(folder1).base
    }
}
struct VecP(Vec<u32>);
impl Producer for VecP { type Item = u32; fn items(self) -> Vec<u32> { self.0 } }
struct Sum(u64);
impl Folder<u64> for Sum {
    type Result = u64;
    fn consume(self, i: u64) -> Self { Sum(self.0 + i) }
    fn complete(self) -> u64 { self.0 }
}
fn main() {
    let f = |x: u32| x as u64 * 2;
    let p = MapProducer { base: VecP(vec![1, 2, 3]), map_op: &f };
    assert_eq!(p.fold_with(Sum(0)).complete(), 12);
}
