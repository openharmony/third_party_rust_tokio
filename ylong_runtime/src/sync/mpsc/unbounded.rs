use crate::sync::error::{RecvError, SendError};
#[cfg(feature = "timer")]
use crate::timer::timeout::timeout;
#[cfg(feature = "timer")]
use std::time::Duration;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::UnboundedReceiver as TokioReceiver;
use tokio::sync::mpsc::UnboundedSender as TokioSender;

/// The sender of the unbounded channel.
/// A [`UnboundedSender`] and [`UnboundedReceiver`] handle pair are created by the
/// [`unbounded_channel`] function.
/// There could be multiple senders for a channel.
#[derive(Clone)]
pub struct UnboundedSender<T>(TokioSender<T>);

/// The receiver of the unbounded channel.
/// There could be only one receiver for a channel.
pub struct UnboundedReceiver<T>(TokioReceiver<T>);

/// Creates a new mpsc channel, and returns the `Sender` and `Receiver` handle pair.
/// The channel is unbounded.
///
/// # Examples
/// ```
/// use ylong_runtime::sync::mpsc::unbounded_channel;
///
/// ylong_runtime::block_on(async move {
///     let (tx, mut rx) = unbounded_channel();
///     let handle = ylong_runtime::spawn(async move {
///         assert_eq!(rx.recv().await, Ok(1));
///     });
///     let handle2 = ylong_runtime::spawn(async move {
///         assert!(tx.send(1).is_ok());
///     });
///     let _ = handle.await;
///     let _ = handle2.await;
/// });
/// ```
pub fn unbounded_channel<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (s, r) = tokio::sync::mpsc::unbounded_channel();
    (UnboundedSender(s), UnboundedReceiver(r))
}

impl<T> UnboundedSender<T> {
    /// Sends a value to the associated receiver.
    ///
    /// If the receiver has been closed, this method will return an error containing the
    /// sent value.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::mpsc::unbounded_channel;
    /// use ylong_runtime::sync::error::RecvError;
    /// ylong_runtime::block_on(async move {
    ///     let (tx, mut rx) = unbounded_channel();
    ///     let handle = ylong_runtime::spawn(async move {
    ///       assert_eq!(rx.recv().await, Ok(1));
    ///     });
    ///     let handle2 = ylong_runtime::spawn(async move {
    ///         assert!(tx.send(1).is_ok());
    ///     });
    ///     let _ = handle.await;
    ///     let _ = handle2.await;
    ///  });
    /// ```
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.0.send(value).map_err(|e| SendError::Closed(e.0))
    }

    /// Checks whether the channel is closed. If so, the sender could not
    /// send values anymore.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::mpsc::unbounded_channel;
    /// let (tx, rx) = unbounded_channel::<isize>();
    /// assert!(!tx.is_closed());
    /// drop(rx);
    /// assert!(tx.is_closed());
    /// ```
    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    /// Checks whether the sender and another send belong to the same channel.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::mpsc::unbounded_channel;
    /// let (tx, rx) = unbounded_channel::<isize>();
    /// let tx2 = tx.clone();
    /// assert!(tx.is_same(&tx2));
    /// ```
    pub fn is_same(&self, other: &Self) -> bool {
        self.0.same_channel(&other.0)
    }
}

impl<T> UnboundedReceiver<T> {
    /// Attempts to receive a value from the assocaited [`UnboundedSender`].
    ///
    /// # Return value
    /// * `Ok(T)` if receiving a value successfully.
    /// * `Err(RecvError::Empty)` if no value has been sent yet.
    /// * `Err(RecvError::Closed)` if all senders have been dropped.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::mpsc::unbounded_channel;
    /// use ylong_runtime::sync::error::RecvError;
    /// let (tx, mut rx) = unbounded_channel();
    /// match rx.try_recv() {
    ///     Err(RecvError::Empty) => {},
    ///     _ => unreachable!(),
    /// }
    /// tx.send(1).unwrap();
    /// match rx.try_recv() {
    ///     Ok(_) => {},
    ///      _ => unreachable!(),
    /// }
    pub fn try_recv(&mut self) -> Result<T, RecvError> {
        self.0.try_recv().map_err(|e| match e {
            TryRecvError::Empty => RecvError::Empty,
            TryRecvError::Disconnected => RecvError::Empty,
        })
    }

    /// Receives a value from the associated [`UnboundedSender`].
    ///
    /// The `receiver` can still receive all sent messages in the channel after the
    /// channel is closed.
    ///
    /// # Return value
    /// * `Ok(T)` if receiving a value successfully.
    /// * `Err(RecvError::Closed)` if all senders has been dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::mpsc::unbounded_channel;
    /// ylong_runtime::block_on(async move {
    ///     let (tx, mut rx) = unbounded_channel();
    ///     let handle = ylong_runtime::spawn(async move {
    ///         assert_eq!(rx.recv().await, Ok(1))
    ///     });
    ///     tx.send(1).unwrap();
    ///     let _ = handle.await;
    /// });
    /// ```
    pub async fn recv(&mut self) -> Result<T, RecvError> {
        match self.0.recv().await {
            None => Err(RecvError::Closed),
            Some(x) => Ok(x),
        }
    }

    /// Receives a value from the assocaited [`UnboundedSender`] in a limited amount of time.
    ///
    /// The `receiver` can still receive all sent messages in the channel after the channel
    /// is closed.
    ///
    /// # Return value
    /// * `Ok(T)` if receiving a value successfully.
    /// * `Err(RecvError::Closed)` if all senders has been dropped.
    /// * `Err(RecvError::Timeout)` if time limit has been passed.
    #[cfg(feature = "timer")]
    pub async fn recv_timeout(&mut self, time: Duration) -> Result<T, RecvError> {
        match timeout(time, self.recv()).await {
            Ok(res) => res,
            Err(_) => Err(RecvError::TimeOut),
        }
    }

    /// Closes the channel, prevents the `Sender` from sending more values.
    ///
    /// The `Sender` will fail to call `send` after the `Receiver` called
    /// `close`. It will do nothing if the channel is closed.
    ///
    /// # Exampels
    /// ```
    /// use ylong_runtime::sync::mpsc::unbounded_channel;
    /// ylong_runtime::block_on(async move {
    ///     let (tx, mut rx) = unbounded_channel();
    ///     assert!(!tx.is_closed());
    ///
    ///     rx.close();
    ///     assert!(tx.is_closed());
    ///     assert!(tx.send("no receive").is_err());
    /// });
    /// ```
    pub fn close(&mut self) {
        self.0.close()
    }
}
