//! One-shot channel is used to send a single message from a single sender to a single recevier.
//! The [`channel`] function returns a [`Sender`] and [`Receiver`] handle pair that controls channel.
//!
//! The [`Sender`] handle is used by the producer to send a message.
//! The [`Receiver`] handle is used by the consumer to receive the message. It has implemented
//! the `Future` trait
//!
//! The [`Sender::send`] method is not async. It can be called from non-async context.

use crate::sync::error::RecvError;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::Receiver as TokioReceiver;
use tokio::sync::oneshot::Sender as TokioSender;

/// Sends a single value to the associated [`Receiver`].
/// A [`Sender`] and [`Receiver`] handle pair is created by the [`channel`] function.
///
/// The receiver will fail with a [`RecvError`] if the sender is dropped without sending a value.
#[derive(Debug)]
pub struct Sender<T>(TokioSender<T>);

/// Receives a single value from the associated [`Sender`].
/// A [`Sender`] and [`Receiver`] handle pair is created by the [`channel`] function.
///
/// There is no `recv` method to receive the message because the receiver iteslf implements
/// the [`Future`] trait. To receive a value, `.await` the `Receiver` object directly.
#[derive(Debug)]
pub struct Receiver<T>(TokioReceiver<T>);

/// Creates a new one-shot channel and returns the `Sender` and `Receiver` handle pair
///
/// The `Sender` could only send a single value to the `Receiver`
///
/// # Examples
///
/// ```
/// use ylong_runtime::sync::oneshot;
/// ylong_runtime::block_on(async move {
///     let (tx, rx) = oneshot::channel();
///     ylong_runtime::spawn(async move {
///         assert!(tx.send(6).is_ok());
///     });
///     let recv = rx.await.unwrap();
///     assert_eq!(recv, 6);
/// });
/// ```
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (s, r) = tokio::sync::oneshot::channel();
    (Sender(s), Receiver(r))
}

impl<T> Sender<T> {
    /// Sends a single value to the associated [`Receiver`], returns the value back
    /// if it fails to send.
    ///
    /// The sender will consume itself when calling this method. It can send a single value
    /// in synchronous code as it doesn't need waiting.
    pub fn send(self, value: T) -> Result<(), T> {
        self.0.send(value)
    }

    /// Checks whether the channel is closed. If so, the sender could not
    /// send value anymore.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::oneshot;
    /// let (tx, rx) = oneshot::channel::<i8>();
    /// assert!(!tx.is_closed());
    /// drop(rx);
    /// assert!(tx.is_closed());
    /// ```
    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }
}

impl<T> Receiver<T> {
    /// Attempts to receive a value from the associated [`Sender`].
    ///
    /// The method will still receive the result if the `Sender` gets dropped after
    /// sending the message.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::error::RecvError;
    /// use ylong_runtime::sync::oneshot;
    /// let (tx, mut rx) = oneshot::channel();
    /// assert_eq!(rx.try_recv(), Err(RecvError::Empty));
    ///
    /// tx.send("Hello").unwrap();
    /// assert_eq!(rx.try_recv(), Ok("Hello"));
    /// ```
    pub fn try_recv(&mut self) -> Result<T, RecvError> {
        self.0.try_recv().map_err(|e| match e {
            TryRecvError::Empty => RecvError::Empty,
            TryRecvError::Closed => RecvError::Closed,
        })
    }

    /// Closes the channel, prevents the `Sender` from sending a value.
    ///
    /// The `Sender` will fail to call [`send`] after the `Receiver` called `close`.
    /// It will do nothing if the channel is already closed or the message has been
    /// already received.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::oneshot;
    /// let (tx, mut rx) = oneshot::channel();
    /// assert!(!tx.is_closed());
    ///
    /// rx.close();
    /// assert!(tx.is_closed());
    /// assert!(tx.send("no receive").is_err());
    /// ```
    pub fn close(&mut self) {
        self.0.close();
    }
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        Pin::new(&mut this.0)
            .poll(cx)
            .map_err(|_| RecvError::TimeOut)
    }
}
