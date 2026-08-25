use crate::cli::Output;
use crate::encryption::file_encryption::{decrypt_file, encrypt_file};
use crate::errors::EnkryptitError;
use crate::parameters::params::EnkryptitParams;
use crate::types::KeyType::{self};

/// Public helper for encrypting a file (Converts Ok<>/EnkryptitError to Output)
pub fn encrypt_file_case(
    parameters: &EnkryptitParams,
    path: &str,
    key: [u8; 32],
    key_type: KeyType,
) -> Result<Output, EnkryptitError> {
    match encrypt_file(path, key, parameters, key_type) {
        Ok(path) => Ok(Output::Success {
            message: format!("file was encrypted at {} !", &path).to_string(),
        }),
        Err(e) => return Ok(Output::Error { error: e }),
    }
}

/// Public helper for decrypting a file (Converts Ok<>/EnkryptitError to Output)
pub fn decrypt_file_case(
    path: &str,
    meta: Vec<u8>,
    key: [u8; 32],
    key_type: KeyType,
    payload_offset: u64,
) -> Result<Output, EnkryptitError> {
    match decrypt_file(path, &meta, key, payload_offset, key_type) {
        Ok(path) => Ok(Output::Success {
            message: format!("file was decrypted at {} !", &path).to_string(),
        }),
        Err(e) => return Ok(Output::Error { error: e }),
    }
}
