# nfcxx's fixes to mrustc

Base: upstream `thepowersgang/mrustc` at `d85e86b` (PR #441), the commit nfcxx pins in
`scripts/setup-mrustc.sh` of take-cheeze/nfcxx. Each entry is a bug found while compiling Rust programs and crates
with mrustc, with a minimal reproduction. A fix is one commit on this branch; the status column says which.

Every fix changes mrustc's output, so the nfcxx rewrites in `scripts/mrustc-prep.rb` are checked against the new output
(the census in `docs/notes/mrustc-frontend.md`, section 2) before the pin moves.

| # | Symptom | Minimal reproduction | Where | Status |
| --- | --- | --- | --- | --- |
| 1 | 128-bit `checked_mul` misses overflow when bits of the multiplicand are shifted out: `(1u128 << 100).checked_mul(1 << 28)` gives `Some(0)`. | `(1u128 << 100).checked_mul(1 << 28)` should be `None`. `u128::checked_mul` is compiled in `libcore`, so the check on the helper itself is the test (below). | `src/trans/codegen_c.cpp`, the `mul128_o` prelude helper | fixed in this branch (`mul128_o`: the top bit of `b` shifted out now sets the overflow flag if `a` has a bit above) |
| 2 | Constant evaluation folds `(-30000i16).checked_mul(2)` to `Some(5536)`; at run time it is `None`. | `const X: Option<i16> = (-30000i16).checked_mul(2);` (or a constant operand) | constant evaluation of `mul_with_overflow` (`src/hir/constant_evaluation.cpp`, MIR const folding) | open |
| 3 | A macro fragment `$i:item` does not match `async fn`: `m! { async fn f() {} }` gives `No arm matched`. A plain `fn` matches. | `macro_rules! m { ($($i:item)*) => { $($i)* } } m! { async fn f() {} } fn main() {}` | `src/macro_rules/eval.cpp` (item fragment parsing) | open |
| 4 | `&'static` to a generic associated constant holding function pointers fails: `Lifetime bound '#local0: 'static failed`. (`bytes` 1.12.1, `owned_clone`.) | `struct VTable { drop: unsafe fn(*mut ()) }`, `impl<T> Owned<T> { const VTABLE: VTable = VTable { drop: owned_drop::<T> }; }`, `Bytes { vtable: &Owned::<T>::VTABLE }` in a generic `fn make<T>()` | promotion of constant references in the type or borrow check | open |
| 5 | `f64` and `f32` mismatch in a type check: `Mismatched types - f64 and f32` (zmij 1.0.23, the float formatter of serde_json 1.0.154). | `zmij` 1.0.23 from crates.io | `src/hir/hir_ops.cpp:286` | open |
| 6 | `No arm matched` in libc 0.2.191's `custom_struct` macro. | libc 0.2.191 (not minimised) | `src/macro_rules/eval.cpp` | open |

Reproductions were run with rustc accepting the program (rustc 1.9x) and mrustc at the base commit.

## Verification

- #1: the helper, old and fixed, compiled as C and checked against gcc's `unsigned __int128` multiply overflow flag on
  eight fixed cases and 200,000 random pairs. Old: 928 wrong (including `2^100 * 2^28`). Fixed: 0 wrong, and the product
  is right whenever no overflow is flagged. The compiler builds from this branch (`make`).
