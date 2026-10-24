// compile-flags: --edition 2021
// `impl Trait` returned by a trait method captures `Self` (and so the lifetime of `Self = &'a [T]`) in every edition
// (rayon 1.12, `ChunkBySlice`).
trait ChunkBySlice<T>: Sized {
    fn chunk_by(self, pred: &impl Fn(&T, &T) -> bool) -> impl Iterator<Item = Self>;
}
impl<T> ChunkBySlice<T> for &[T] {
    fn chunk_by(self, pred: &impl Fn(&T, &T) -> bool) -> impl Iterator<Item = Self> {
        self.chunk_by(pred)
    }
}
fn main() {
    let v = [1, 1, 2, 3, 3, 3];
    let s: &[i32] = &v;
    let n: Vec<usize> = ChunkBySlice::chunk_by(s, &|a: &i32, b: &i32| a == b).map(|c| c.len()).collect();
    assert_eq!(n, [2, 1, 3]);
}
