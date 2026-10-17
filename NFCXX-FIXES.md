# nfcxx's fixes to mrustc

Base: upstream `thepowersgang/mrustc` at `d85e86b` (PR #441), the commit nfcxx pins in
`scripts/setup-mrustc.sh` of take-cheeze/nfcxx. Each entry is a bug found while compiling Rust programs and crates
with mrustc, with a minimal reproduction. A fix is one commit on this branch; the status column says which.

Every fix changes mrustc's output, so the nfcxx rewrites in `scripts/mrustc-prep.rb` are checked against the new output
(the census in `docs/notes/mrustc-frontend.md`, section 2) before the pin moves.

| # | Symptom | Minimal reproduction | Where | Status |
| --- | --- | --- | --- | --- |
| 1 | 128-bit `checked_mul` misses overflow when bits of the multiplicand are shifted out: `(1u128 << 100).checked_mul(1 << 28)` gives `Some(0)`. | `(1u128 << 100).checked_mul(1 << 28)` should be `None`. `u128::checked_mul` is compiled in `libcore`, so the check on the helper itself is the test (below). | `src/trans/codegen_c.cpp`, the `mul128_o` prelude helper | fixed in this branch (`mul128_o`: the top bit of `b` shifted out now sets the overflow flag if `a` has a bit above) |
| 2 | Constant evaluation folds `(-30000i16).checked_mul(2)` to `Some(5536)`; at run time it is `None`. Also wrong for `300u16 * 300`, `46341i32 * 46341` and `-128i8 * -1`. | `println!("{:?}", (-30000i16).checked_mul(2));` should print `None` | `src/hir_conv/constant_evaluation.cpp`, the MUL cases of `do_arith_checked` | fixed in this branch (below 128 bits, the exact product must survive truncation and sign extension) |
| 3 | A macro fragment `$i:item` does not match `async fn`: `m! { async fn f() {} }` gives `No arm matched`. A plain `fn` matches. | `macro_rules! m { ($($i:item)*) => { $($i)* } } m! { async fn f() -> u32 { 7 } pub async unsafe fn h() {} fn main() {} }` | `src/macro_rules/eval.cpp`, `consume_item` (the item matcher had no case for `async`) | fixed in this branch (`async [unsafe] fn` is accepted, as `const fn` is) |
| 4 | `&'static` to a generic associated constant holding function pointers fails: `Lifetime bound '#local0: 'static failed`. (`bytes` 1.12.1, `owned_clone`.) | `struct VTable { drop: unsafe fn(*mut ()) }`, `impl<T> Owned<T> { const VTABLE: VTable = VTable { drop: owned_drop::<T> }; }`, `Bytes { vtable: &Owned::<T>::VTABLE }` in a generic `fn make<T>()` | `src/hir_expand/static_borrow_constants.cpp`, `ExprNode_PathValue` visitor: a constant path that is still generic is never promoted (the `monomorphise_path_needed` branch) | open. An experiment that treats generic constants as promotable makes the repro pass and the fork's std build (`make LIBS`) succeed, but the sample suite cannot run here (it needs `alloc_system` and reports `Unknown macro panic`), so it was not committed. Needs a regression run first. |
| 5 | `f64` and `f32` mismatch in a type check: `Mismatched types - f64 and f32` (zmij 1.0.23, the float formatter of serde_json 1.0.154). | `trait T { const M: i32; const N: i32 = Self::M * 2; } impl T for f32 { const M: i32 = 1; } impl T for f64 { const M: i32 = 2; } struct S { d: [u8; f64::N as usize] }` with `fn main() { let s = S { d: [0; 4] }; println!("{}", s.d.len()); }` (prints `4`; `zmij` 1.0.23 is the crates.io source) | `src/hir_conv/constant_evaluation.cpp`, `get_const`: the trait's provided value is evaluated with `Self` set to the type the path resolved to; before, `Self` was an unbound ivar that matched every impl of the trait | fixed in this branch |
| 6 | `No arm matched` in libc 0.2.191's `custom_struct` macro. | libc 0.2.191 (not minimised) | `src/macro_rules/eval.cpp` | open |
| 7 | An `async fn` inside a function body is parsed as an async block: `async fn inner() {}` in a block gives `Unexpected token TOK_RWORD_FN, expected TOK_BRACE_OPEN`. Found by tokio's `wait_with_output` (`tokio/src/process/mod.rs`). | `fn outer() -> u32 { async fn inner(x: u32) -> u32 { x + 1 } 5 } fn main() { println!("{}", outer()); }` | `src/parse/expr.cpp`, statement parsing: `async [unsafe] fn` is an item, as `unsafe fn` already was | fixed in this branch (`async {` and `async move {` remain expressions) |
| 8 | A macro item fragment that is itself a macro call, substituted into an `impl` body, is a TODO: `Parse_Impl_Item - Interpolated item into impl: MacroInv`. Found by tokio's `cfg_unstable_metrics!`. | `macro_rules! gen { () => { pub fn hi(&self) -> u32 { 9 } }; } macro_rules! mk { ($i:item) => { impl S { $i } }; } struct S; mk! { gen!(); } fn main() { println!("{}", S.hi()); }` should print `9` | `src/parse/root.cpp`, `Parse_Impl_Item`: the interpolated `MacroInv` is added as an impl macro invocation | fixed in this branch |
| 9 | tokio 1.53.3 stops with `Unspecified lifetime in outer context`, with no source location. The method is `fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>>`: the output's elided lifetime comes from a `Pin<&mut Self>` receiver, which rustc takes as a reference receiver. | `trait Buf { fn poll_fill(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&[u8]>; }` with an impl returning `&self.get_mut().0[..]` (rustc accepts; mrustc gave the error) | `src/hir_conv/lifetime_elision.cpp`, `visit_function`: a `Pin<&T>`/`Pin<&mut T>` receiver is a reference receiver | fixed in this branch |
| 10 | tokio 1.53.3 then stops in type checking: `Failed to find an impl of Future for async[..] with Output = Result<(), RecvError>`, an async block whose output type is not inferred (tokio `sync/watch.rs`). | not minimised | `src/hir_typeck` (async block output inference) | open |

Reproductions were run with rustc accepting the program (rustc 1.9x) and mrustc at the base commit.

## Verification

- #1: the helper, old and fixed, compiled as C and checked against gcc's `unsigned __int128` multiply overflow flag on
  eight fixed cases and 200,000 random pairs. Old: 928 wrong (including `2^100 * 2^28`). Fixed: 0 wrong, and the product
  is right whenever no overflow is flagged. The compiler builds from this branch (`make`).
- #2: a sweep of `checked_mul` on literal operands, 873 cases over the boundary values of i8, u8, i16, u16, i32, u32 and
  i64, compared with rustc's output. Old compiler: 304 wrong. Fixed compiler: 0 wrong. The run-time path is unchanged.
- #3: the repro above compiled with rustc and printed `3`; the old compiler aborts with `No arm matched` and the fixed one
  prints `3`. `consume_item` had no case for `async`, so its default returned no match.
- #7: the repro prints `7` with rustc and the fixed compiler. `unsafe` blocks, `async {}` and `async move {}` are unchanged.
- #8: the repro prints `9` with rustc and the fixed compiler; the pinned compiler aborts with the TODO.
- #9: the `Pin<&mut Self>` repro prints the same as rustc with the fixed compiler.
