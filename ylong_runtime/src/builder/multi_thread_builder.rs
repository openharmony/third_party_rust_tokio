use crate::builder::common_builder::{impl_common, CommonBuilder};
use tokio::io;

pub struct MultiThreadBuilder {
    pub(crate) common: CommonBuilder,
    pub(crate) thread_num: Option<u8>,
}

impl MultiThreadBuilder {
    pub(crate) fn new() -> Self {
        MultiThreadBuilder {
            common: CommonBuilder::new(),
            thread_num: None,
        }
    }

    /// Initializes the runtime and returns its instance.
    #[cfg(feature = "multi_thread_runtime")]
    pub fn build(&mut self) -> io::Result<crate::executor::Runtime> {
        self.build_inner()
    }

    pub(crate) fn build_inner(&mut self) -> io::Result<crate::executor::Runtime> {
        let mut runtime = self.build_common(Builder::new_multi_thread());

        if let Some(thread_num) = self.thread_num {
            runtime.worker_threads(thread_num as usize);
        };
        Ok(crate::executor::Runtime(runtime.build()?))
    }

    /// Sets the number of core worker threads.
    ///
    /// The boundary of the thread number is 1-64:
    /// If sets a number smaller than 1, then thread number would be set to 1.
    /// If sets a number larger than 64, then thread number would be set to 64.
    /// The default number of the core threads is the number of cores of the cpu.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_runtime::builder::RuntimeBuilder;
    ///
    /// let runtime = RuntimeBuilder::new_multi_thread()
    ///     .thread_number(8);
    /// ```
    pub fn thread_number(mut self, thread_num: u8) -> Self {
        if thread_num < 1 {
            self.thread_num = Some(1);
        } else if thread_num > 64 {
            self.thread_num = Some(64);
        } else {
            self.thread_num = Some(thread_num)
        }
        self
    }
}

impl_common!(MultiThreadBuilder);
