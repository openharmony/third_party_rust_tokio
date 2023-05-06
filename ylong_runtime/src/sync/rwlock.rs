use crate::sync::LockError;
use tokio::sync::RwLock as TokioRwLock;
pub use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

/// An asynchronous version of `RwLock` in 'std'.
///
/// RwLock allows multiple readers or a single writer to operate concurrently.
/// Readers are only allowed to read the data, but the writer is the only one can
/// change the data inside.
///
/// This RwLock's policy is writer-first, to prevent writers from starving.
///
/// # Examples
///
/// ```
/// use ylong_runtime::sync::RwLock;
/// ylong_runtime::block_on(async {
///     let lock = RwLock::new(0);
///     let r1 = lock.read().await;
///     let r2 = lock.read().await;
///     assert_eq!(*r1, 0);
///     assert_eq!(*r2, 0);
///     drop((r1, r2));
///
///     let mut w = lock.write().await;
///     *w += 1;
///     assert_eq!(*w, 1);
/// });
/// ```
pub struct RwLock<T: ?Sized>(TokioRwLock<T>);

unsafe impl<T: ?Sized + Send> Send for RwLock<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

impl<T: Sized> RwLock<T> {
    /// Creates a new RwLock. `T` is the data that needs to be protected
    /// by this RwLock.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::RwLock;
    /// let lock = RwLock::new(0);
    /// ```
    pub fn new(t: T) -> RwLock<T> {
        RwLock(TokioRwLock::new(t))
    }
}

impl<T: ?Sized> RwLock<T> {
    /// Asynchronously acquires the read lock.
    ///
    /// If there is a writer holding the write lock, then this method will wait asynchronously
    /// for the write lock to get released.
    ///
    /// Buf if the write lock is not held, it's ok for mulitple readers to hold the read lock
    /// concurrently.
    ///
    /// ```
    /// use ylong_runtime::sync::RwLock;
    /// ylong_runtime::block_on(async {
    ///     let lock = RwLock::new(0);
    ///     let r1 = lock.read().await;
    ///     assert_eq!(*r1, 0);
    /// });
    /// ```
    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().await
    }

    /// Attempts to get the read lock. If another writer is holding the write lock, then
    /// a LockError will be returned. Otherwise, a [`RwLockReadGuard`] will be returned.
    ///
    /// ```
    /// use ylong_runtime::sync::RwLock;
    /// let lock = RwLock::new(0);
    /// let r1 = lock.try_read().unwrap();
    /// assert_eq!(*r1, 0);
    /// ```
    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, LockError> {
        self.0.try_read().map_err(|_| LockError {})
    }

    /// Asynchronously acquires the write lock.
    ///
    /// If there is other readers or writers, then this method will wait asynchronously
    /// for them to get released.
    ///
    /// ```
    /// use ylong_runtime::sync::RwLock;
    /// ylong_runtime::block_on(async {
    ///     let lock = RwLock::new(0);
    ///     let r1 = lock.read().await;
    ///
    /// });
    /// ```
    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().await
    }

    /// Attempts to acquire the write lock,
    ///
    /// If any other task holds the read/write lock, a LockError will be returned.
    ///
    /// ```
    /// use ylong_runtime::sync::RwLock;
    /// let lock = RwLock::new(0);
    /// let mut r1 = lock.try_write().unwrap();
    /// *r1 += 1;
    /// assert_eq!(*r1, 1);
    /// ```
    pub fn try_write(&self) -> Result<RwLockWriteGuard<'_, T>, LockError> {
        self.0.try_write().map_err(|_| LockError {})
    }

    /// Consumes the lock, and returns the data protected by it.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::RwLock;
    /// let lock = RwLock::new(0);
    /// assert_eq!(lock.into_inner(), 0);
    /// ```
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.0.into_inner()
    }

    /// Gets the mutable reference of the data protected by the lock.
    ///
    /// This method takes the mutable reference of the RwLock, so there is no need to actually
    /// lock the mutex -- the mutable borrow statically guarantees no locks exist.
    ///
    /// ```
    /// use ylong_runtime::sync::RwLock;
    /// ylong_runtime::block_on(async {
    ///     let mut lock = RwLock::new(0);
    ///     *lock.get_mut() = 10;
    ///     assert_eq!(*lock.write().await, 10);
    /// });
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}
