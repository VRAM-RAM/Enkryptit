pub mod multithread;
pub mod single;

use crate::context::EnkryptitContext;
use crate::encryption::file_encryption::multithread::{
    decrypt_multithread_file, encrypt_multithread_file,
};
use crate::encryption::file_encryption::single::{decrypt_file_single, encrypt_file_single};
use crate::errors::EnkryptitError;
use crate::key::EnkryptitKey;
use crate::metadatas::MetaDatas;
use crate::parameters::params::EnkryptitParams;
use crate::types::KeyType::{self};
use crate::types::{Mode, ParallelismType};

/// Public function that encrypts a file (it also resolves the key and keytype)
pub fn encrypt_file(
    path: &str,
    parameters: &EnkryptitParams,
    keytype: &KeyType,
    context: &mut EnkryptitContext,
) -> Result<String, EnkryptitError> {
    // Creates the enkryptit key (and resolves keytype & key)
    let enkryptit_key: EnkryptitKey =
        EnkryptitKey::resolve(Mode::Encrypting, keytype, context, path)?;

    // We match the parallelism type
    match parameters.parallelism {
        ParallelismType::Single => encrypt_file_single(path, parameters.compression, enkryptit_key),
        ParallelismType::MultiThread(threads) => {
            encrypt_multithread_file(path, parameters.compression, enkryptit_key, threads)
        }
    }
}

/// Public function that decrypts a file
pub fn decrypt_file(
    path: &str,
    meta_bytes: &[u8],
    payload_offset: u64,
    context: &mut EnkryptitContext,
    parallelism: ParallelismType,
) -> Result<String, EnkryptitError> {
    // First, we deserialize the metadata
    let metadatas: MetaDatas = postcard::from_bytes(meta_bytes)?;
    // And extract the compression type...
    let compression_type = metadatas.compression;
    // ... and the nonce
    let master_nonce = metadatas.nonce;

    // We resolve the key
    let enkryptit_key =
        EnkryptitKey::resolve(Mode::Decrypting, &metadatas.key_type, context, path)?;

    match parallelism {
        ParallelismType::Single => decrypt_file_single(
            path,
            payload_offset,
            enkryptit_key,
            master_nonce,
            compression_type,
        ),
        ParallelismType::MultiThread(threads) => decrypt_multithread_file(
            path,
            payload_offset,
            enkryptit_key,
            master_nonce,
            compression_type,
            threads,
        ),
    }
}
