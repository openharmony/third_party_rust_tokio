use crate::builder::RuntimeBuilder;
use crate::join_handle::JoinHandle;
use std::future::Future;
use std::mem::MaybeUninit;
use std::sync::Once;

pub struct Runtime(pub(crate) tokio::runtime::Runtime);

impl Runtime {
    /// Spawns a future onto the runtime, returning a [`JoinHandle`] for it.
    ///
    /// The `future` will be later polled by the executor, which is usually implemented as a thread
    /// pool. The executor will run the future until finished.
    ///
    /// Awaits on the JoinHandle will return the result of the future, but users don't have to
    /// explicitly `.await` the `JoinHandle` in order to run the future, since the future will
    /// be executed in the background once it's spawned. Dropping the JoinHandle will throw away
    /// the returned value.
    ///
    /// # Examples
    ///
    /// ```no run
    /// use ylong_runtime::builder::RuntimeBuilder;
    /// let runtime = RuntimeBuilder::new_current_thread().build().unwrap();
    /// let handle = runtime.spawn(async move {
    ///     1
    /// });
    /// assert_eq!(runtime.block_on(handle).unwrap(), 1);
    /// ```
    pub fn spawn<T, R>(&self, task: T) -> JoinHandle<R>
    where
        T: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let handle = self.0.spawn(task);
        JoinHandle(handle)
    }

    /// Spawns the provided function or closure onto the runtime.
    ///
    /// It's usually used for cpu-bounded computation that does not return pending and takes
    /// a relatively long time.
    ///
    /// # Examples
    ///
    /// ```no run
    /// use ylong_runtime::builder::RuntimeBuilder;
    /// let runtime = RuntimeBuilder::new_current_thread().build().unwrap();
    /// let handle = runtime.spawn_blocking(move || {
    ///     10
    /// });
    /// assert_eq!(runtime.block_on(handle).unwrap(), 10);
    /// ```
    pub fn spawn_blocking<T, R>(&self, task: T) -> JoinHandle<R>
    where
        T: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let handle = self.0.spawn_blocking(task);
        JoinHandle(handle)
    }

    /// Blocks the current thread and runs the given future (usually a JoinHandle) to completion,
    /// and gets its return value.
    ///
    /// Any code after the `block_on` will be executed once the future is done.
    ///
    /// Don't use this method on an asynchronous environment, since it will block the worker
    /// thread and may cause deadlock.
    pub fn block_on<T, R>(&self, task: T) -> R
    where
        T: Future<Output = R>,
    {
        self.0.block_on(task)
    }
}

pub(crate) fn get_global_runtime() -> &'static Runtime {
    static mut GLOBAL_DEFAULT_RT: MaybeUninit<Runtime> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let rt = match RuntimeBuilder::new_multi_thread().build_inner() {
                Ok(rt) => rt,
                Err(e) => panic!("initializing global runtime failed: {:?}", e),
            };
            GLOBAL_DEFAULT_RT = MaybeUninit::new(rt);
        });
        &*GLOBAL_DEFAULT_RT.as_ptr()
    }
}
