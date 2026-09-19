use crate::errors::EnkryptitError;

use crate::parallelism::executable::EnkryptitExecutable;

pub mod executable;
pub mod pool;
pub mod worker;

pub struct EnkryptitJob<T: EnkryptitExecutable> {
    pub task: T,
}

impl<T: EnkryptitExecutable> EnkryptitJob<T> {
    #[allow(dead_code)]
    /// Returns an [`EnkryptitJob`]. Unused in the binary, but used in tests.
    pub fn new(task: T) -> Self {
        Self { task }
    }

    pub fn execute(self) -> Result<T::Output, EnkryptitError> {
        self.task.execute()
    }
}
