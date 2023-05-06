use crate::sync::error::{RecvError, SendError};
#[cfg(feature = "timer")]
use crate::timer::timeout::timeout;
#[cfg(feature = "timer")]
use std::time::Duration;
#[cfg(feature = "timer")]
use tokio::sync::mpsc::error::SendTimeoutError;
use tokio::sync::mpsc::error::{TryRecvError, TrySendError};
use tokio::sync::mpsc::{channel, Receiver, Sender};

/// The sender of the bounded channel.
/// A [`BoundedSender`] and [`BoundedReceiver`] handle pair are created by the
/// [`bounded_channel`] function.
/// There could be multiple senders for a channel.
#[derive(Clone)]
pub struct BoundedSender<T>(Sender<T>);

/// The receiver of the bounded channel.
/// There could be only one receiver for a channel.
pub struct BoundedReceiver<T>(Receiver<T>);

/// Creates a new mpsc channel, and returns the `Sender` and `Receiver` handle pair.
/// The channel is bounded with the passed in capacity.
///
/// # Examples
/// ```
/// use ylong_runtime::sync::mpsc::bounded_channel;
///
/// ylong_runtime::block_on(async move {
///     let (tx, mut rx) = bounded_channel(1);
///     let handle = ylong_runtime::spawn(async move {
///         assert_eq!(rx.recv().await, Ok(1));
///     });
///     let handle2 = ylong_runtime::spawn(async move {
///         assert!(tx.send(1).await.is_ok());
///     });
///     let _ = handle.await;
///     let _ = handle2.await;
/// });
/// ```
pub fn bounded_channel<T>(capacity: usize) -> (BoundedSender<T>, BoundedReceiver<T>) {
    let (s, r) = channel(capacity);
    (BoundedSender(s), BoundedReceiver(r))
}

impl<T> BoundedSender<T> {
    /// Attempts to send a value to the associated [`BoundedReceiver`].
    ///
    /// If the receiver has been closed or the channel is full, this method will
    /// return an error containing the sent value
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// use ylong_runtime::sync::error::RecvError;
    ///
    /// ylong_runtime::block_on(async move {
    ///     let (tx, mut rx) = bounded_channel(1);
    ///     match rx.try_recv() {
    ///         Err(RecvError::Empty) => {}
    ///         _ => unreachable!(),
    ///     }
    ///     tx.try_send(1).unwrap();
    ///     match rx.try_recv() {
    ///         Ok(x) => assert_eq!(x, 1),
    ///         _ => unreachable!(),
    ///     }
    /// });
    /// ```
    pub fn try_send(&self, value: T) -> Result<(), SendError<T>> {
        self.0.try_send(value).map_err(|e| match e {
            TrySendError::Full(x) => SendError::Full(x),
            TrySendError::Closed(x) => SendError::Closed(x),
        })
    }

    /// Sends a value to the associated receiver.
    ///
    /// If the receiver has been closed, this method will return an error containing the
    /// sent value.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// use ylong_runtime::sync::error::RecvError;
    /// ylong_runtime::block_on(async move {
    ///     let (tx, mut rx) = bounded_channel(1);
    ///     let handle = ylong_runtime::spawn(async move {
    ///       assert_eq!(rx.recv().await, Ok(1));
    ///     });
    ///     let handle2 = ylong_runtime::spawn(async move {
    ///         assert!(tx.send(1).await.is_ok());
    ///     });
    ///     let _ = handle.await;
    ///     let _ = handle2.await;
    ///  });
    /// ```
    pub async fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.0.send(value).await.map_err(|e| SendError::Closed(e.0))
    }

    /// Attempts to send a value to the associated receiver in a limited amount of time.
    ///
    /// If the receiver has been closed or the time limit has been passed, this method
    /// will return an error containing the sent value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// ylong_runtime::block_on(async move {
    ///     let (tx, mut rx) = bounded_channel(1);
    ///     let handle = ylong_runtime::spawn(async move {
    ///         assert_eq!(rx.recv().await, Ok(1));
    ///     });
    ///     let handle2 = ylong_runtime::spawn(async move {
    ///         assert!(tx.send_timeout(1, Duration::from_millis(10)).await.is_ok());
    ///     });
    ///     let _ = handle.await;
    ///     let _ = handle2.await;
    /// });
    /// ```
    #[cfg(feature = "timer")]
    pub async fn send_timeout(&self, value: T, time: Duration) -> Result<(), SendError<T>> {
        self.0.send_timeout(value, time).await.map_err(|e| match e {
            SendTimeoutError::Timeout(x) => SendError::Timeout(x),
            SendTimeoutError::Closed(x) => SendError::Closed(x),
        })
    }

    /// Checks whether the channel is closed. If so, the sender could not
    /// send values anymore.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// let (tx, rx) = bounded_channel::<isize>(1);
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
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// let (tx, rx) = bounded_channel::<isize>(1);
    /// let tx2 = tx.clone();
    /// assert!(tx.is_same(&tx2));
    /// ```
    pub fn is_same(&self, other: &Self) -> bool {
        self.0.same_channel(&other.0)
    }

    /// Gets the capacity of the channel.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// let (tx, rx) = bounded_channel::<isize>(5);
    /// assert_eq!(tx.capacity(), 5);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<T> BoundedReceiver<T> {
    /// Attempts to receive a value from the assocaited [`BoundedSender`].
    ///
    /// # Return value
    /// * `Ok(T)` if receiving a value successfully.
    /// * `Err(RecvError::Empty)` if no value has been sent yet.
    /// * `Err(RecvError::Closed)` if all senders have been dropped.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// use ylong_runtime::sync::error::RecvError;
    /// let (tx, mut rx) = bounded_channel(1);
    /// match rx.try_recv() {
    ///     Err(RecvError::Empty) => {},
    ///     _ => unreachable!(),
    /// }
    /// tx.try_send(1).unwrap();
    /// match rx.try_recv() {
    ///     Ok(_) => {},
    ///      _ => unreachable!(),
    /// }
    /// drop(tx);
    /// match rx.try_recv() {
    ///     Err(RecvError::Closed) => {}
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub fn try_recv(&mut self) -> Result<T, RecvError> {
        self.0.try_recv().map_err(|e| match e {
            TryRecvError::Empty => RecvError::Empty,
            TryRecvError::Disconnected => RecvError::Closed,
        })
    }

    /// Receives a value from the associated [`BoundedSender`].
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
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// ylong_runtime::block_on(async move {
    ///     let (tx, mut rx) = bounded_channel(1);
    ///     let handle = ylong_runtime::spawn(async move {
    ///         assert_eq!(rx.recv().await, Ok(1))
    ///     });
    ///     tx.try_send(1).unwrap();
    ///     let _ = handle.await;
    /// });
    /// ```
    pub async fn recv(&mut self) -> Result<T, RecvError> {
        match self.0.recv().await {
            None => Err(RecvError::Closed),
            Some(x) => Ok(x),
        }
    }

    /// Receives a value from the assocaited [`BoundedSender`] in a limited amount of time.
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
    /// The `Sender` will fail to call `send` or `try_send` after the `Receiver` called
    /// `close`. It will do nothing if the channel is closed.
    ///
    /// # Exampels
    /// ```
    /// use ylong_runtime::sync::mpsc::bounded_channel;
    /// ylong_runtime::block_on(async move {
    ///     let (tx, mut rx) = bounded_channel(1);
    ///     assert!(!tx.is_closed());
    ///
    ///     rx.close();
    ///     assert!(tx.is_closed());
    ///     assert!(tx.try_send("no receive").is_err());
    /// });
    /// ```
    pub fn close(&mut self) {
        self.0.close()
    }
}
