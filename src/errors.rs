use std::sync::mpsc::{RecvError};

use thiserror::Error;


#[allow(dead_code)]
#[derive(Debug, Error)]
/// Enum for all the errors of **Enkryptit**. Also implements `From<>` other error types.
pub enum EnkryptitError {
    #[error("operation interrupted")]
    Break,

    #[error("postcard error: {0}")]
    PostcardError(#[from] postcard::Error),

    #[error("Send error to mspc channel")]
    SendError,

    #[error("Receive error from mspc channel : {0}")]
    ReceiveError(#[from] RecvError),

    #[error("Invalid worker count")]
    InvalidWorkerCount,

    #[error("encryption/decryption failed")]
    Encryption,

    #[error("argon2 password hash failed")]
    Argon2Error,

    #[error("key derivation error: {0}")]
    KeyDerivationError(String),

    #[error("unable to determine key type")]
    KeyTypeGetError,

    #[error("Invalid key type. Found {0}, expected {1}")]
    InvalidKeyType(String, String),
    
    #[error("keyring error: {0}")]
    KeyringError(#[from] keyring::Error),

    #[error("Key not found !")]
    KeyNotFound,

    #[error("hex decoding error: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("unexpected end of file")]
    UnexpectedEof,

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown Action : {0}")]
    UnknownAction(String),

    #[error("invalid key length")]
    InvalidKeyLength,

    #[error("Command not found")]
    CommandNotFound,

    #[error("error while searching for the file")]
    FileError,

    #[error("error while searching for this specific file")]
    SpecificFileError(String),

    #[error("unable to lock memory")]
    MemoryLockError,

    #[error("home directory not found")]
    HomeNotFound,

    #[error("configuration error")]
    ConfigError,

    #[error("Error with the Tui : {0}")]
    TuiError(std::io::Error),

    #[error("json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Lz4 compressionError: {0}")]
    Lz4CompressionError(#[from] lz4_flex::block::CompressError),

    #[error("lz4 decompression error: {0}")]
    Lz4DecompressionError(#[from] lz4_flex::block::DecompressError),

    #[error("zstd error: {0}")]
    ZstdError(#[from] oxiarc_core::error::OxiArcError),

    #[error("Corrupted File")]
    CorruptedFile,

    #[error("Thread panicked")]
    ThreadPanicked,
}

impl From<chacha20poly1305::Error> for EnkryptitError {
    fn from(_: chacha20poly1305::Error) -> Self {
        EnkryptitError::Encryption
    }
}

impl From<argon2::Error> for EnkryptitError {
    fn from(_: argon2::Error) -> Self {
        EnkryptitError::Argon2Error
    }
}
