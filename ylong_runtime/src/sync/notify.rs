use tokio::sync::Notify as TokioNotify;

/// Notifies one or multiple tasks to wake up.
///
/// `Notify` itself does not protect any data. Its only purpose is to signal other
/// tasks to perform an operation.
///
/// # Examples
/// ```
/// use std::sync::Arc;
/// use ylong_runtime::sync::Notify;
/// let notify = Arc::new(Notify::new());
/// let notify2 = notify.clone();
///
/// let _ = ylong_runtime::block_on(async {
///     ylong_runtime::spawn(async move {
///         notify2.notified().await;
///     });
///     notify.notify_one();
/// });
/// ```
pub struct Notify(TokioNotify);

impl Notify {
    /// Creates a new Notify.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::sync::Notify;
    /// let notify = Notify::new();
    /// ```
    pub fn new() -> Notify {
        Notify(TokioNotify::new())
    }

    /// Asynchronously waits for this Notify to get signaled.
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use ylong_runtime::sync::Notify;
    /// let notify = Arc::new(Notify::new());
    /// let notify2 = notify.clone();
    /// ylong_runtime::block_on(async {
    ///     ylong_runtime::spawn(async move {
    ///         notify2.notified().await;
    ///     });
    ///     notify.notify_one();
    /// });
    /// ```
    pub async fn notified(&self) {
        self.0.notified().await
    }

    /// Notifies one task waiting on this `Notify`
    ///
    /// If this method gets called when there is no task waiting on this `Notify`,
    /// then the next task called `notified` on it will not get blocked.
    ///
    /// If the method gets called multiple times, only one task will get passed straightly
    /// when calling `notified`. Any other task still has to asynchronously wait for it to be
    /// released.
    pub fn notify_one(&self) {
        self.0.notify_one()
    }

    /// Notifies all tasks waiting on this `Notify`
    ///
    /// Unlike `notify_one`, if this method gets called when there is no task waiting on this `Notify`,
    /// the next task called `notified` on it will still get blocked.
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use std::sync::atomic::AtomicI8;
    /// use ylong_runtime::sync::Notify;
    /// let notify = Arc::new(Notify::new());
    /// let notify2 = notify.clone();
    /// let notify3 = notify.clone();
    ///
    /// ylong_runtime::block_on(async {
    ///     let handle = ylong_runtime::spawn(async move {
    ///         notify2.notified().await;
    ///
    ///     });
    ///     let handle2 = ylong_runtime::spawn(async move {
    ///         notify3.notified().await;
    ///     });
    ///
    ///     notify.notify_all();
    /// });
    /// ```
    pub fn notify_all(&self) {
        self.0.notify_waiters()
    }
}
