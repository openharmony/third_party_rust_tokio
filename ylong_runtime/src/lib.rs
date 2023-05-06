pub mod builder;
pub mod executor;
#[cfg(feature = "fs")]
pub mod fs;
pub mod io;
#[cfg(feature = "net")]
pub mod net;
#[cfg(feature = "sync")]
pub mod sync;
pub mod task;
#[cfg(feature = "timer")]
pub mod timer;

pub use task::join_handle;

use crate::executor::get_global_runtime;
use crate::join_handle::JoinHandle;
use std::future::Future;

/// Spawns a task onto the global runtime.
pub fn spawn<T, R>(task: T) -> JoinHandle<R>
where
    T: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    get_global_runtime().spawn(task)
}

/// Spawns a blocking task onto the blocking pool.
pub fn spawn_blocking<T, R>(task: T) -> JoinHandle<R>
where
    T: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    get_global_runtime().spawn_blocking(task)
}

/// Blocks the current thread until the `Future` passed in is completed.
pub fn block_on<T>(task: T) -> T::Output
where
    T: Future,
{
    get_global_runtime().block_on(task)
}
