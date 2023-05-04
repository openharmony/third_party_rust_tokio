use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
pub use tokio::sync::SemaphorePermit;
use tokio::sync::{Semaphore as TokioSemaphore, TryAcquireError};

const MAX_PERMITS: usize = usize::MAX >> 1;

/// Asynchronous counting semaphore. It allows more than one caller to access the shared resource.
/// Semaphore contains a set of permits. Call `acquire` method and get a permit to access the
/// shared resource. When permits are used up, new requests to acquire permit will wait until
/// [`Semaphore::release`] method is called or permit acquired before gets dropped. When no request
/// is waiting, calling `release` method will add a permit to semaphore.
pub struct Semaphore(TokioSemaphore);

#[derive(Debug, Eq, PartialEq)]
pub enum SemaphoreError {
    Overflow,
    Empty,
    Closed,
}

impl Display for SemaphoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SemaphoreError::Overflow => write!(f, "permit overflowed"),
            SemaphoreError::Empty => write!(f, "no permit inside"),
            SemaphoreError::Closed => write!(f, "semaphore closed"),
        }
    }
}

impl Error for SemaphoreError {}

impl Semaphore {
    /// Creates a semaphore with an initial permit value.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::Semaphore;
    /// let sem = Semaphore::new(4).unwrap();
    /// ```
    pub fn new(permits: usize) -> Result<Semaphore, SemaphoreError> {
        if permits >= MAX_PERMITS {
            return Err(SemaphoreError::Overflow);
        }
        Ok(Semaphore(TokioSemaphore::new(permits)))
    }

    /// Gets the number of remaining permits.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::Semaphore;
    /// let sem = Semaphore::new(4).unwrap();
    /// assert_eq!(sem.current_permits(), 4);
    /// ```
    pub fn current_permits(&self) -> usize {
        self.0.available_permits()
    }

    /// Adds a permit to the semaphore.
    ///
    /// # Exampels
    ///
    /// ```
    /// use ylong_runtime::sync::Semaphore;
    /// let sem = Semaphore::new(4).unwrap();
    /// assert_eq!(sem.current_permits(), 4);
    /// sem.release();
    /// assert_eq!(sem.current_permits(), 5);
    /// ```
    pub fn release(&self) {
        self.0.add_permits(1);
    }

    /// Attempts to acquire a permit from semaphore.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::Semaphore;
    /// let sem = Semaphore::new(4).unwrap();
    /// assert_eq!(sem.current_permits(), 4);
    /// let permit = sem.try_acquire().unwrap();
    /// assert_eq!(sem.current_permits(), 3);
    /// drop(permit);
    /// assert_eq!(sem.current_permits(), 4);
    /// ```
    pub fn try_acquire(&self) -> Result<SemaphorePermit, SemaphoreError> {
        self.0.try_acquire().map_err(|e| match e {
            TryAcquireError::Closed => SemaphoreError::Closed,
            TryAcquireError::NoPermits => SemaphoreError::Empty,
        })
    }

    /// Asynchronously acquires a permit from semaphore.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::Semaphore;
    /// ylong_runtime::block_on(async {
    ///     let sem = Semaphore::new(2).unwrap();
    ///     ylong_runtime::spawn(async move {
    ///         let _permit = sem.acquire().await.unwrap();
    ///     });
    /// });
    /// ```
    pub async fn acquire(&self) -> Result<SemaphorePermit, SemaphoreError> {
        self.0.acquire().await.map_err(|_| SemaphoreError::Closed)
    }

    /// Checks whether semaphore is closed. If so, the semaphore could not be acquired anymore.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::Semaphore;
    /// let sem = Semaphore::new(4).unwrap();
    /// assert!(!sem.is_closed());
    /// sem.close();
    /// assert!(sem.is_closed());
    /// ```
    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    /// Closes the semaphore so that it could not be acquired anymore,
    /// and it notifies all requests in the waiting list.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::Semaphore;
    /// let sem = Semaphore::new(4).unwrap();
    /// assert!(!sem.is_closed());
    /// sem.close();
    /// assert!(sem.is_closed());
    /// ```
    pub fn close(&self) {
        self.0.close()
    }
}
