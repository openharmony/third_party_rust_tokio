use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A handle to the actual spawned task.
///
/// This can be considered as the equivalent of [`std::thread::JoinHandle`]
/// for a ylong task rather than a thread.
///
/// It could be used to join the corresponding task or cancel it.
/// If a `JoinHandle` is dropped, then the task continues executing in the background
/// and its return value is lost. There is no way to join the task after its JoinHandle
/// is dropped.
///
/// # Examples
/// ```
/// let handle = ylong_runtime::spawn(async {
///     let handle2 = ylong_runtime::spawn(async {1});
///     assert_eq!(handle2.await.unwrap(), 1);
/// });
/// ylong_runtime::block_on(handle).unwrap();
/// ```
pub struct JoinHandle<R>(pub(crate) tokio::task::JoinHandle<R>);

impl<R> Future for JoinHandle<R> {
    type Output = io::Result<R>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        Pin::new(&mut this.0).poll(cx).map_err(io::Error::from)
    }
}
