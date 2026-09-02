use serde::{Deserialize, Serialize};

/// Version type
pub type Version = u8;

/// Chunk size of a bloc to compress & encrypt
pub const CHUNK_SIZE: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// KeyType enum. Contains :
/// - Password
/// - Pwd256(salt)
/// - FromFile
/// - FromOS
/// - None
pub enum KeyType {
    Password,
    Pwd256([u8; 16]),
    FromFile,
    FromOS,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// KeyParams enum. Contains :
/// - PassWord
/// - File
/// - Os
pub enum KeyParams {
    PassWord,
    File,
    Os,
}

impl KeyParams {
    pub fn to_type(&self) -> KeyType {
        match self {
            &Self::File => KeyType::FromFile,
            &Self::Os => KeyType::FromOS,
            &Self::PassWord => KeyType::Password,
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Encryption mode
pub enum Mode {
    Encrypting,
    Decrypting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// To provide something clean when `matching` the interface in `EnkryptitContext`
pub enum Interface {
    Cli,
    Tui,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelismType {
    Auto,
    MultiThread(u8),
    Single,
}
