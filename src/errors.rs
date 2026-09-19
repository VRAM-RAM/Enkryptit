use std::{path::{PathBuf, StripPrefixError}, sync::mpsc::RecvError};

use fmodeparser::{FullPermissionError};
use thiserror::Error;

use crate::diagnostic::EnkryptitOutput;
use crate::diagnostic::output::kind::EnkryptitOutputKind;

#[allow(dead_code)]
#[derive(Debug, Error)]
/// Enum for all the errors of **Enkryptit**. Also implements `From<>` other error types.
/// \
/// Some `EnkryptitError`s are unused by the binary, but used by the tests. That's why `dead_code` is allowed.
pub enum EnkryptitError {
    #[error("operation interrupted")]
    Break,

    #[error("Error while parsing file permissions : {0}")]
    FilePermissionsParsingError(#[from] FullPermissionError),

    #[error("postcard error: {0}")]
    PostcardError(#[from] postcard::Error),

    #[error("Path is incorrect : {0}")]
    PathIsIncorrect(String),

    #[error("Directory found is the directory of the folder")]
    DirectoryIsFolder,

    #[error("Error while stripping a prefix : {0}")]
    StripPrefixError(StripPrefixError),

    #[error("Failed to read metadata of file")]
    FailedToReadMetadata(PathBuf),

    #[error("File is a SymLink (shortcut)")]
    FileIsASymLink,

    #[error("Send error to mspc channel")]
    SendError,

    #[error("Receive error from mspc channel : {0}")]
    ReceiveError(#[from] RecvError),

    #[error("Invalid worker count")]
    InvalidWorkerCount,

    #[error("encryption/decryption failed : {0}")]
    EncryptionError(chacha20poly1305::Error),

    #[error("argon2 password hash failed")]
    Argon2Error,

    #[error("key derivation error: {0}")]
    KeyDerivationError(String),

    #[error("Invalid key type. Found {0}, expected {1}")]
    InvalidKeyType(String, String),

    #[error("keyring error: {0}")]
    KeyringError(#[from] keyring::Error),

    #[error("The file containing the key was not found")]
    KeyNotFoundInFile,

    #[error("The Key couldn't be found in Os' keyring")]
    KeyNotFoundInOs,

    #[error("Inspection failed")]
    InspectionError,

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
}

impl EnkryptitError {
    pub fn code(&self) -> String {
        match *self {
            // Cryptographic error codes
            Self::Argon2Error => "crypto::argon2error".to_string(),
            Self::EncryptionError(_) => "crypto::encryption_decryption_failed".to_string(),
            Self::InvalidKeyLength => "crypto::invalid_key_length".to_string(),
            Self::InvalidKeyType(..) => "crypto::invalid_key_type".to_string(),
            Self::KeyDerivationError(_) => "crypto::key_derivation_failed".to_string(),
            Self::KeyNotFoundInFile => "crypto::key_not_found_in_file".to_string(),
            Self::KeyNotFoundInOs => "crypto::key_not_found_in_os".to_string(),
            Self::KeyringError(_) => "crypto::error_with_os_keyring".to_string(),

            // Compression-related error codes
            Self::Lz4CompressionError(_) => "comp::lz4_comp_failed".to_string(),
            Self::ZstdError(_) => "comp::zstd_comp_failed".to_string(),

            // Ui error codes
            Self::CommandNotFound => "ui::command_not_found".to_string(),
            Self::TuiError(_) => "ui:tui_error".to_string(),

            // Enkryptit! format error codes
            Self::CorruptedFile => "format::corrupted_file".to_string(),
            Self::DirectoryIsFolder => "format::directory_is_folder".to_string(),
            Self::FailedToReadMetadata(_) => "format::metadata_reading_failed".to_string(),

            // Io error codes
            Self::UnexpectedEof => "io::unexpected_eof".to_string(),
            Self::HomeNotFound => "io::home_directory_not_found".to_string(),
            Self::IoError(_) => "io::misc_io_error".to_string(),
            Self::PathIsIncorrect(_) => "io::incorrect_path".to_string(),
            Self::SpecificFileError(_) => "io::file_not_found".to_string(),
            Self::FileError => "io::file_not_found".to_string(),
            Self::FileIsASymLink => "io::file_is_a_symlink".to_string(),
            Self::FilePermissionsParsingError(_) => "io::permissions_reading_failed".to_string(),

            // Miscellaneous errors
            Self::InspectionError => "misc::inspection_failed".to_string(),
            Self::HexError(_) => "misc::hex_decoding_failed".to_string(),
            Self::ConfigError => "misc::config_directory_error".to_string(),
            Self::Break => "misc::break".to_string(),
            _ => "none".to_string(),
        }
    }

    /// A remediation hint, shown alongside the error output.
    pub fn help(&self) -> Option<String> {
        match self {
            Self::InspectionError => Some("Please ensure that the file exists, and that its name does not contain wrong unicode character.".to_string()),
            Self::Argon2Error => Some("Key derivation with Argon2 failed. Check your available memory.".to_string()),
            Self::EncryptionError(_) => Some("Check your password/key and make sure the file is not corrupted.".to_string()),
            Self::InvalidKeyLength => Some("The key must be exactly 32 bytes long.".to_string()),
            Self::InvalidKeyType(_, _) => Some("Use a supported key type: Password, Os or File.".to_string()),
            Self::KeyDerivationError(_) => Some("The password or key file seems invalid.".to_string()),
            Self::KeyNotFoundInFile => Some("Place your key file at the configured path.".to_string()),
            Self::KeyNotFoundInOs => Some("Store the key in the OS keyring (e.g. keychain / GNOME Keyring) first.".to_string()),
            Self::KeyringError(_) => Some("Make sure your OS keyring is unlocked.".to_string()),
            Self::CorruptedFile => Some("Wait for `eck recover <path>` for help.".to_string()),
            Self::DirectoryIsFolder => Some("You can't treat the current folder itself.".to_string()),
            Self::FailedToReadMetadata(_) => Some("Check that the file still exists and is readable.".to_string()),
            Self::FileIsASymLink => Some("Use the real path instead of a symbolic link.".to_string()),
            Self::PathIsIncorrect(_) => Some("Check that the path exists.".to_string()),
            Self::FileError => Some("Check that the file exists and is readable.".to_string()),
            Self::UnexpectedEof => Some("The file is truncated; it may be corrupted.".to_string()),
            Self::IoError(_) => Some("Check that the path exists and that you have the required permissions.".to_string()),
            Self::HomeNotFound => Some("Set the HOME environment variable.".to_string()),
            Self::ConfigError => Some("Check the config file location and format.".to_string()),
            Self::SerdeJsonError(_) => Some("The parameter/config file is not valid JSON.".to_string()),
            Self::Lz4CompressionError(_) | Self::Lz4DecompressionError(_) => Some("The data is corrupted or not Lz4-compressed.".to_string()),
            Self::ZstdError(_) => Some("The data is corrupted or not Zstandard-compressed.".to_string()),
            Self::InvalidWorkerCount => Some("Choose a worker count greater than 0.".to_string()),
            Self::UnknownAction(_) => Some("Use a valid command.".to_string()),
            Self::FilePermissionsParsingError(_) => Some("The permissions of this file could not be parsed.".to_string()),
            Self::PostcardError(_) => Some("The file seems corrupted or not in the Enkryptit! format.".to_string()),
            _ => None,
        }
    }

    /// A short hint about where in the code the error happened.
    pub fn location(&self) -> Option<String> {
        match self {
            Self::Argon2Error | Self::KeyDerivationError(_) | Self::HexError(_) | Self::MemoryLockError => Some("key derivation".to_string()),
            Self::EncryptionError(_) => Some("encryption/decryption".to_string()),
            Self::InvalidKeyType(_, _) | Self::InvalidKeyLength => Some("key setup".to_string()),
            Self::KeyringError(_) => Some("OS keyring".to_string()),
            Self::KeyNotFoundInFile => Some("key file lookup".to_string()),
            Self::KeyNotFoundInOs => Some("OS keyring lookup".to_string()),
            Self::CorruptedFile => Some("archive parsing".to_string()),
            Self::DirectoryIsFolder | Self::StripPrefixError(_) => Some("folder treatment".to_string()),
            Self::FailedToReadMetadata(_) => Some("metadata reading".to_string()),
            Self::FileIsASymLink => Some("path check".to_string()),
            Self::IoError(_) => Some("I/O".to_string()),
            Self::Lz4CompressionError(_) => Some("compression".to_string()),
            Self::Lz4DecompressionError(_) => Some("decompression".to_string()),
            Self::ZstdError(_) => Some("decompression".to_string()),
            Self::PostcardError(_) => Some("read_file()".to_string()),
            Self::UnexpectedEof => Some("reading archive".to_string()),
            Self::SerdeJsonError(_) => Some("configuration parsing".to_string()),
            Self::ConfigError => Some("configuration".to_string()),
            Self::HomeNotFound => Some("project directory".to_string()),
            Self::FilePermissionsParsingError(_) => Some("fmodeparser".to_string()),
            Self::SendError | Self::ReceiveError(_) => Some("parallelism".to_string()),
            Self::InvalidWorkerCount => Some("parallelism setup".to_string()),
            Self::TuiError(_) => Some("TUI".to_string()),
            Self::UnknownAction(_) => Some("command dispatch".to_string()),
            Self::PathIsIncorrect(_) | Self::SpecificFileError(_) | Self::FileError => Some("file lookup".to_string()),
            _ => None,
        }
    }

    pub fn into_output(self) -> EnkryptitOutput {
        let msg = self.to_string();
        EnkryptitOutput::new(
            EnkryptitOutputKind::Error {
                location: self.location(),
                help: self.help(),
                snippet: None,
                error: self,
            },
            msg,
        )
    }
}


impl From<EnkryptitError> for EnkryptitOutput {
    fn from(error: EnkryptitError) -> EnkryptitOutput {
        let msg = error.to_string();
        EnkryptitOutput::new(
            EnkryptitOutputKind::Error {
                location: error.location(),
                help: error.help(),
                snippet: None,
                error,
            },
            msg,
        )
    }
}

impl From<chacha20poly1305::Error> for EnkryptitError {
    fn from(e: chacha20poly1305::Error) -> Self {
        EnkryptitError::EncryptionError(e)
    }
}

impl From<argon2::Error> for EnkryptitError {
    fn from(_: argon2::Error) -> Self {
        EnkryptitError::Argon2Error
    }
}
