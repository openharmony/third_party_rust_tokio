use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod error;
pub mod mpsc;
pub(crate) mod mutex;
pub(crate) mod notify;
pub mod oneshot;
pub(crate) mod rwlock;
pub(crate) mod semaphore;

pub use mutex::{Mutex, MutexGuard};
pub use notify::Notify;
pub use rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use semaphore::{Semaphore, SemaphoreError, SemaphorePermit};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LockError;

impl Display for LockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The attempt to get the mutex failed")
    }
}

impl Error for LockError {}
