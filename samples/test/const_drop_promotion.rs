// compile-flags: --test

static mut DROPS: u32 = 0;

struct D(u8);
impl Drop for D {
    fn drop(&mut self) {
        unsafe { DROPS += 1; }
    }
}

const X: D = D(1);
const NONE: Option<D> = None;

// `&X` is not promoted: the temporary copy is dropped at the end of `k`
fn k() -> u8 {
    let r = &X;
    r.0
}

// `&NONE` has no value that needs drop, so it is still promoted to a `'static`
fn none_ref() -> &'static Option<D> {
    &NONE
}

#[test]
fn const_with_drop_glue_is_dropped() {
    assert_eq!(k(), 1);
    assert_eq!(unsafe { DROPS }, 1);
}

#[test]
fn const_without_drop_value_is_promoted() {
    assert!(none_ref().is_none());
}
