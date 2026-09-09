use serde::{Deserialize, Serialize};
use hex::{ToHex};

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

impl ToString for KeyType {
    fn to_string(&self) -> String {
        match self {
            Self::FromFile => "from file".to_string(),
            Self::FromOS => "from os keyring".to_string(),
            Self::None => "no keytype used (should not happen if the file is encrypted".to_string(),
            Self::Password => "password".to_string(),
            Self::Pwd256(salt) => format!("hashed password, with the following salt : {}", salt.encode_hex::<String>()),
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

impl ToString for CompressionType {
    fn to_string(&self) -> String {
        match self {
            Self::Auto => "Automatic".to_string(),
            Self::Lz4 => "Lz4 (fastest)".to_string(),
            Self::Xz => "Xz (slowest but most efficient)".to_string(),
            Self::NoComp => "No compression".to_string(),
            Self::Zstd => "Zstd (Balanced)".to_string()
        }
    }
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

impl ToString for ParallelismType {
    fn to_string(&self) -> String {
        match self {
            Self::Auto => "Automatic".to_string(),
            Self::MultiThread(n) => format!("MultiThreading with {} threads", n),
            Self::Single => "SingleThread".to_string()
        }
    }
}
