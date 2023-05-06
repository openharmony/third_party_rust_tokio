use crate::executor::get_global_runtime;
use crate::join_handle::JoinHandle;
use crate::task::PriorityLevel;
use std::future::Future;

/// Tasks attribute
pub struct TaskBuilder {
    pub(crate) name: Option<String>,
    pub(crate) pri: Option<PriorityLevel>,
}

impl Default for TaskBuilder {
    fn default() -> Self {
        TaskBuilder::new()
    }
}

impl TaskBuilder {
    /// Creates a new `TaskBuilder` with a default setting.
    pub fn new() -> Self {
        TaskBuilder {
            name: None,
            pri: None,
        }
    }

    /// Sets the name of the task
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the priority of the task
    pub fn pri(mut self, pri_level: PriorityLevel) -> Self {
        self.pri = Some(pri_level);
        self
    }

    /// Using the current task setting, spawns a task onto the global runtime
    pub fn spawn<T, R>(&self, task: T) -> JoinHandle<R>
    where
        T: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        get_global_runtime().spawn(task)
    }

    /// Using the current task setting, spawns a task onto the blocking pool.
    pub fn spawn_blocking<T, R>(&self, task: T) -> JoinHandle<R>
    where
        T: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        get_global_runtime().spawn_blocking(task)
    }
}
