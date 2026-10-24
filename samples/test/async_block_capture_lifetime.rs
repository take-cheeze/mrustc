// compile-flags: --edition 2021
// An async block in a function with lifetime parameters, capturing a `&mut dyn Iterator` argument by reference: the
// capture expression stays in the function, with the function's lifetimes (cc 1.4, `run_commands_in_parallel`).
use std::future::Future;
use std::task::{Context, Poll, Waker};
fn block_on<F: Future>(f: F) -> F::Output {
    let mut f = Box::pin(f);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}
fn sum(cmds: &mut dyn Iterator<Item = Result<u32, String>>) -> Result<u32, String> {
    let fut = async {
        let mut t = 0;
        for c in cmds {
            t += c?;
        }
        Ok(t)
    };
    block_on(fut)
}
fn main() {
    assert_eq!(sum(&mut vec![Ok(1), Ok(2), Ok(3)].into_iter()), Ok(6));
    assert_eq!(sum(&mut vec![Ok(1), Err("x".to_string())].into_iter()), Err("x".to_string()));
}
