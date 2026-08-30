use crate::errors::EnkryptitError;

use crate::parallelism::executable::EnkryptitExecutable;

pub mod worker;
pub mod pool;
pub mod executable;

pub struct EnkryptitJob<T: EnkryptitExecutable> {
    pub index: u64,
    pub task: T
}

impl<T: EnkryptitExecutable> EnkryptitJob<T> {
    pub fn new(index: u64, task: T) -> Self {
        Self { index, task }
    }

    pub fn execute(self) -> Result<T::Output, EnkryptitError> {
        self.task.execute()
    }
}

