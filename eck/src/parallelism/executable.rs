use crate::errors::EnkryptitError;

pub trait EnkryptitExecutable {
    type Output: Send + 'static;

    fn execute(self) -> Result<Self::Output, EnkryptitError>;
}
