use crate::context::EnkryptitContext;
use crate::frontend::cli::Output;
use crate::encryption::file_encryption::{decrypt_file, encrypt_file};
use crate::errors::EnkryptitError;
use crate::parameters::params::EnkryptitParams;
use crate::types::KeyType::{self};
use crate::types::ParallelismType;

/// Public helper for encrypting a file (Converts Ok<>/EnkryptitError to Output)
pub fn encrypt_file_case(
    parameters: &EnkryptitParams,
    path: &str,
    context: &mut EnkryptitContext,
    key_type: &KeyType,
) -> Result<Output, EnkryptitError> {
    match encrypt_file(path, parameters, key_type, context) {
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
    context: &mut EnkryptitContext,
    payload_offset: u64,
    parallelism: ParallelismType,
) -> Result<Output, EnkryptitError> {
    match decrypt_file(path, &meta, payload_offset, context, parallelism) {
        Ok(path) => Ok(Output::Success {
            message: format!("file was decrypted at {} !", &path).to_string(),
        }),
        Err(e) => return Ok(Output::Error { error: e }),
    }
}
