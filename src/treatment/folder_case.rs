use crate::context::EnkryptitContext;
use crate::encryption::folder_encryption::{decrypt_folder, encrypt_folder};
use crate::errors::EnkryptitError;
use crate::frontend::cli::Output;
use crate::types::KeyType::{self};

/// Public helper for encrypting a folder. (Converts Ok<>/EnkryptitError to Output).
pub fn encrypt_folder_case(
    path: &str,
    context: &mut EnkryptitContext,
    key_type: &KeyType,
) -> Result<Output, EnkryptitError> {
    // We resolve the path by suppressing the suffix '/', because if we don't do that, the ` encrypt_folder()`
    // function would write 'path/to/my/folder/.encky' instead of 'path/to/my/folder.encky'
    let path = match path.strip_suffix("/") {
        Some(p) => p,
        None => path,
    };

    match encrypt_folder(path, context, key_type) {
        Ok(path) => Ok(Output::Success {
            message: format!("folder was encrypted at {} !", path),
        }),
        Err(e) => Ok(Output::Error { error: e }),
    }
}

/// Public helper for decrypting a folder. (Converts Ok<>/EnkryptitError to Output)
pub fn decrypt_folder_case(
    path: &str,
    context: &mut EnkryptitContext,
    metadatas_bytes: Vec<u8>,
    payload_offset: u64,
    version: u8,
) -> Result<Output, EnkryptitError> {
    match decrypt_folder(path, &metadatas_bytes, payload_offset, version, context) {
        Ok(path) => Ok(Output::Success {
            message: format!("folder was decrypted at {} !", path),
        }),
        Err(e) => Ok(Output::Error { error: e }),
    }
}
