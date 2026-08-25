use crate::cli::Output;
use crate::encryption::folder_encryption::{decrypt_folder, encrypt_folder};
use crate::errors::EnkryptitError;
use crate::parameters::params::EnkryptitParams;
use crate::types::KeyType::{self};

/// Public helper for encrypting a folder. (Converts Ok<>/EnkryptitError to Output).
pub fn encrypt_folder_case(
    path: &str,
    parameters: &EnkryptitParams,
    key: [u8; 32],
    key_type: KeyType,
) -> Result<Output, EnkryptitError> {
    match encrypt_folder(path, parameters, key, key_type) {
        Ok(path) => Ok(Output::Success {
            message: format!("folder was encrypted at {} !", path),
        }),
        Err(e) => Ok(Output::Error { error: e }),
    }
}

/// Public helper for decrypting a folder. (Converts Ok<>/EnkryptitError to Output)
pub fn decrypt_folder_case(
    path: &str,
    key: [u8; 32],
    metadatas_bytes: Vec<u8>,
    payload_offset: u64,
    key_type: KeyType,
    version: u8,
) -> Result<Output, EnkryptitError> {
    match decrypt_folder(
        path,
        &metadatas_bytes,
        key,
        payload_offset,
        key_type,
        version,
    ) {
        Ok(path) => Ok(Output::Success {
            message: format!("folder was decrypted at {} !", path),
        }),
        Err(e) => Ok(Output::Error { error: e }),
    }
}
