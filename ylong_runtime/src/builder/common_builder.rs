use std::time::Duration;

pub(crate) struct CommonBuilder {
    pub(crate) thread_name: Option<String>,

    pub(crate) keep_alive_time: Option<Duration>,

    pub(crate) stack_size: Option<usize>,
}

impl CommonBuilder {
    pub(crate) fn new() -> Self {
        Self {
            thread_name: None,
            keep_alive_time: None,
            stack_size: None,
        }
    }
}

macro_rules! impl_common {
    ($self:ident) => {
        use std::time::Duration;
        use tokio::runtime::Builder;

        impl $self {
            /// Sets the name prefix for all worker threads
            pub fn thread_name(mut self, name: String) -> Self {
                self.common.thread_name = Some(name);
                self
            }

            /// Sets the stack size for every worker thread that gets spawned by the runtime
            /// The minimum stack size is 1.
            pub fn thread_stack_size(mut self, stack_size: usize) -> Self {
                if stack_size < 1 {
                    self.common.stack_size = Some(1);
                } else {
                    self.common.stack_size = Some(stack_size);
                }
                self
            }

            /// Sets how long will the thread be kept alvie inside the blocking pool after
            /// it becomes idle.
            pub fn keep_alive_time(mut self, keep_alive_time: Duration) -> Self {
                self.common.keep_alive_time = Some(keep_alive_time);
                self
            }

            pub(crate) fn build_common(&self, mut builder: Builder) -> Builder {
                if let Some(stack_size) = self.common.stack_size {
                    builder.thread_stack_size(stack_size);
                }

                if let Some(keep_alive) = self.common.keep_alive_time {
                    builder.thread_keep_alive(keep_alive);
                }

                if let Some(name) = &self.common.thread_name {
                    builder.thread_name(name);
                }

                #[cfg(any(feature = "net", feature = "timer"))]
                builder.enable_all();

                builder
            }
        }
    };
}

pub(crate) use impl_common;
