pub mod result;
pub mod encrypt;
pub mod decrypt;

use crate::types::CompressionType;
use chacha20poly1305::XChaCha20Poly1305;
use std::sync::Arc;
use crate::parallelism::pool::EnkryptitPool;
use crate::parallelism::EnkryptitJob;
use crate::errors::EnkryptitError;
use crate::encryption::chunk_job::encrypt::EncryptChunkJob;
use crate::encryption::chunk_job::decrypt::DecryptChunkJob;

 
/// Helper that creates an `EncryptChunkJob` and submits it too the `EnkryptitPool<>`.
pub fn submit_encrypt_chunk(pool: &EnkryptitPool<EncryptChunkJob>, index: u64, data: Vec<u8>, nonce: Arc<[u8; 24]>, compression: Arc<CompressionType>, cipher: Arc<XChaCha20Poly1305>) -> Result<(), EnkryptitError> {
    // We create the job
    let job = EncryptChunkJob {
        index,
        data,
        master_nonce: nonce,
        compression,
        cipher
    };

    // And submit it to the pool
    pool.submit(EnkryptitJob {
        index,
        task: job
    })?;

    Ok(())
}

/// Helper that creates a `DecryptChunkJob` and submits it too the `EnkryptitPool<>`.
pub fn submit_decrypt_chunk(pool: &EnkryptitPool<DecryptChunkJob>, index: u64, data: Vec<u8>, nonce: Arc<[u8; 24]>, compression: Arc<CompressionType>, cipher: Arc<XChaCha20Poly1305>) -> Result<(), EnkryptitError> {
    // We create the job
    let job = DecryptChunkJob {
        index,
        data,
        master_nonce: nonce,
        compression,
        cipher
    };

    // We submit the job to the pool
    pool.submit(EnkryptitJob {
        index,
        task: job
    })?;

    Ok(())
}