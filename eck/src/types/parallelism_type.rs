use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelismType {
    Auto,
    MultiThread(u8),
    Single,
}

impl ParallelismType {
    /// Description of the parallelism type. Single source of
    /// truth used by both `Display` and the `String` conversion.
    pub fn description(&self) -> String {
        match self {
            Self::Auto => "Automatic".to_string(),
            Self::MultiThread(n) => format!("MultiThreading with {} threads", n),
            Self::Single => "SingleThread".to_string()
        }
    }
}

impl From<ParallelismType> for String {
    fn from(value: ParallelismType) -> Self {
        value.description()
    }
}

impl Display for ParallelismType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
