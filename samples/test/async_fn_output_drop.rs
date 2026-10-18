// compile-flags: --edition 2021
// An async fn whose output (or captured argument) is not Copy must return the value it was given, and each
// captured value must be dropped exactly once, whether the future completes, is suspended, or is never polled.
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

static DROPS: AtomicUsize = AtomicUsize::new(0);

struct Counted(u32);
impl Drop for Counted {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

struct YieldOnce(bool);
impl Future for YieldOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn noop_raw() -> RawWaker {
    fn clone(_: *const ()) -> RawWaker { noop_raw() }
    fn noop(_: *const ()) {}
    static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    RawWaker::new(std::ptr::null(), &VT)
}

fn poll_once<F: Future>(f: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<F::Output> {
    f.poll(cx)
}

fn block_on<F: Future>(mut f: F) -> F::Output {
    let waker = unsafe { Waker::from_raw(noop_raw()) };
    let mut cx = Context::from_waker(&waker);
    let mut f = unsafe { Pin::new_unchecked(&mut f) };
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

async fn make<T>(x: T) -> T { x }
async fn make_s(s: String) -> String { s }
async fn pass(c: Counted) -> Counted { c }
async fn hold(c: Counted) -> u32 {
    YieldOnce(false).await;
    c.0
}

fn main() {
    assert_eq!(block_on(make_s(String::from("hello"))), "hello");
    let s = block_on(make(String::from("abc")));
    assert_eq!(s.len(), 3);
    assert_eq!(s, "abc");
    assert_eq!(block_on(make(Some(3u8))), Some(3));
    assert_eq!(block_on(make(41u32)) + 1, 42);

    // Never polled: the captured value is dropped with the future
    drop(pass(Counted(1)));
    assert_eq!(DROPS.load(Ordering::SeqCst), 1);

    // Completed: the returned value is moved out, so it is dropped once (by the caller)
    let c = block_on(pass(Counted(2)));
    assert_eq!(c.0, 2);
    assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    drop(c);
    assert_eq!(DROPS.load(Ordering::SeqCst), 2);

    // Suspended at an await with the argument still live: dropped once with the future
    {
        let waker = unsafe { Waker::from_raw(noop_raw()) };
        let mut cx = Context::from_waker(&waker);
        let mut fut = hold(Counted(3));
        let mut pinned = unsafe { Pin::new_unchecked(&mut fut) };
        assert!(poll_once(pinned.as_mut(), &mut cx).is_pending());
        assert_eq!(DROPS.load(Ordering::SeqCst), 2);
    }
    assert_eq!(DROPS.load(Ordering::SeqCst), 3);

    // Completed after the await: the argument is dropped once, in the body
    assert_eq!(block_on(hold(Counted(4))), 4);
    assert_eq!(DROPS.load(Ordering::SeqCst), 4);
}
