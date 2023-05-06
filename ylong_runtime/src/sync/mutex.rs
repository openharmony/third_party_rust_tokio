use crate::sync::LockError;
pub use tokio::sync::Mutex as TokioMutex;
pub use tokio::sync::MutexGuard;

/// An async version of [`std::sync::Mutex`]
///
/// Often it's considered as normal to use [`std::sync::Mutex`] on an asynchronous environment.
/// The primal purpose of this async mutex is to protect shared reference of io, which contains
/// a lot await point during reading and writing. If you only wants to protect a data across
/// different threads, [`std::sync::Mutex`] will probably gain you better performance.
///
/// When using across different futures, users need to wrap the mutex inside an Arc,
/// just like the use of [`std::sync::Mutex`]
#[derive(Debug)]
pub struct Mutex<T: ?Sized>(TokioMutex<T>);

impl<T: Sized> Mutex<T> {
    /// Creates a mutex that protects the data passed in.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::Mutex;
    /// let _lock = Mutex::new(2);
    /// ```
    pub fn new(t: T) -> Mutex<T> {
        Mutex(TokioMutex::new(t))
    }
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T: ?Sized> Mutex<T> {
    /// Locks the mutex.
    /// If the mutex is already held by others, asynchronously waits for it to release.
    ///
    /// # Examples
    /// ```
    /// use ylong_runtime::sync::Mutex;
    /// ylong_runtime::block_on(async move {
    ///     let lock = Mutex::new(2);
    ///     let mut val = lock.lock().await;
    ///     *val += 1;
    ///     assert_eq!(*val, 3);
    /// });
    /// ```
    pub async fn lock(&self) -> MutexGuard<'_, T> {
        self.0.lock().await
    }

    /// Attempts to get the mutex.
    /// If the lock is already held by others, then LockError will be returned.
    /// Otherwise, the mutex guard will be returned.
    ///
    pub fn try_lock(&self) -> Result<MutexGuard<'_, T>, LockError> {
        match self.0.try_lock() {
            Ok(x) => Ok(x),
            Err(_) => Err(LockError),
        }
    }

    /// Gets the mutable reference of the data protected by the lock without actually
    /// holding the lock.
    ///
    /// This method takes the mutable reference of the mutex, so there is no need to actually
    /// lcok the mutex -- the mutable borrow statically guarantees no locks exist.
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}
