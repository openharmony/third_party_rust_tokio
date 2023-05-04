use crate::builder::common_builder::{impl_common, CommonBuilder};
use std::io;

pub struct CurrentThreadBuilder {
    pub(crate) common: CommonBuilder,
}

impl CurrentThreadBuilder {
    pub(crate) fn new() -> Self {
        CurrentThreadBuilder {
            common: CommonBuilder::new(),
        }
    }

    /// Initializes the runtime and returns its instance.
    pub fn build(&mut self) -> io::Result<crate::executor::Runtime> {
        let mut runtime = self.build_common(Builder::new_current_thread());

        Ok(crate::executor::Runtime(runtime.build()?))
    }
}

impl_common!(CurrentThreadBuilder);
