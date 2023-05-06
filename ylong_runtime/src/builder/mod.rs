#[cfg(feature = "current_thread_runtime")]
use crate::builder::current_thread_builder::CurrentThreadBuilder;
use crate::builder::multi_thread_builder::MultiThreadBuilder;

pub(crate) mod common_builder;
#[cfg(feature = "current_thread_runtime")]
pub mod current_thread_builder;
pub mod multi_thread_builder;

/// Builder to build the runtime. Provides methods to customize the runtime, such
/// as setting thread pool size, worker thread stack size, work thread name prefix and etc.
///
/// if `multi_thread_runtime` or `current_thread_runtime` features is turned on:
/// After setting the RuntimeBuilder, a call to `build` will initialize the runtime
/// and returns its instance. If there is an invalid parameter during the build, an
/// error would be returned.
///
/// Otherwise:
/// RuntimeBuilder wiull not have the `build()` method, instead, this builder should
/// be passed to set the global executor (not implemented yet)
///
/// # Examples
///
/// ```no run
/// #![cfg(feature = "multi_thread_runtime")]
///
/// use ylong_runtime::builder::RuntimeBuilder;
/// use ylong_runtime::executor::Runtime;
///
/// let runtime = RuntimeBUilder::new_multi_thread()
///     .thread_number(4)
///     .thread_stack_size(1024 * 30)
///     .build()
///     .unwrap();
/// ```
pub struct RuntimeBuilder;

impl RuntimeBuilder {
    /// Initializes a new RuntimeBuilder with current_thread settings.
    ///
    /// All tasks will run on the current thread, which means it does not create any other
    /// worker threads.
    #[cfg(feature = "current_thread_runtime")]
    pub fn new_current_thread() -> CurrentThreadBuilder {
        CurrentThreadBuilder::new()
    }

    /// Initializes a new RuntimeBuilder with multi_thread settings.
    ///
    /// When running, worker threads will be created according to the builder configuration,
    /// and tasks will be allocated and run in the newly created worker thread pool.
    pub fn new_multi_thread() -> MultiThreadBuilder {
        MultiThreadBuilder::new()
    }
}
