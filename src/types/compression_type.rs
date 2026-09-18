use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// CompressionType enum. Contains :
/// - Lz4
/// - Zstd
/// - Xz
/// - NoComp
/// - Auto (infered by Enkryptit itself !)
pub enum CompressionType {
    Lz4,
    Zstd,
    Xz,
    NoComp,
    Auto,
}

impl CompressionType {
    /// Description of the compression type. Single source of
    /// truth used by both `Display` and the `String` conversion.
    pub fn description(&self) -> String {
        match self {
            Self::Auto => "Automatic".to_string(),
            Self::Lz4 => "Lz4 (fastest)".to_string(),
            Self::Xz => "Xz (slowest but most efficient)".to_string(),
            Self::NoComp => "No compression".to_string(),
            Self::Zstd => "Zstd (Balanced)".to_string()
        }
    }
}

impl From<CompressionType> for String {
    fn from(value: CompressionType) -> Self {
        value.description()
    }
}

impl Display for CompressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
