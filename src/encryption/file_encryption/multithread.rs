use crate::encryption::encryption_flow::{decrypt_stream, encrypt_stream};
use crate::encryption::encryption_primitives::generate_nonce;
use crate::errors::EnkryptitError;
use crate::frontend::cli::Output;
use crate::key::EnkryptitKey;
use crate::metadatas::{ArchiveHeader, MetaDatas};
use crate::parallelism::EnkryptitJob;
use crate::parallelism::pool::EnkryptitPool;
use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use std::io::{Seek, SeekFrom};
use std::sync::Arc;
use chacha20poly1305::{KeyInit, XChaCha20Poly1305};
use zeroize::Zeroize;
use crate::types::{CHUNK_SIZE, CompressionType};
use crate::encryption::encrypt_chunk_job::{EncryptChunkJob, DecryptChunkJob};
use crate::encryption::encrypt_chunk_job::ChunkResult;

pub fn encrypt_multithread(path: &str, compression: CompressionType, enkryptit_key: EnkryptitKey, num_threads: u8) -> Result<String, EnkryptitError> {
    // First, we initialize the workers pool
    let mut pool = EnkryptitPool::<EncryptChunkJob>::new(num_threads as usize)?;

    // Then, we open the file and create the reader
    let file = File::open(path)?;
    let total_size: u64 = file.metadata()?.len();
    let mut reader = BufReader::new(file);

    // Generates the nonce
    let mut master_nonce = generate_nonce();
    // We prepare the shared context for Multithreading
    let cipher = Arc::new(XChaCha20Poly1305::new(enkryptit_key.key_as_ref().into()));

    // And creates the metadata
    let metadata = MetaDatas::new(enkryptit_key.key_type(), compression, master_nonce).pack()?;

    let encrypted_path = format!("{}.encky", path);

    // Creates the `cipherfile` placeholder
    let cipherfile = std::fs::File::create(&encrypted_path)?;

    let mut writer = BufWriter::new(cipherfile);

    let actual_meta_len = metadata.len() as u32;

    // Creates and pack the header
    let header = ArchiveHeader::new(false, actual_meta_len).pack()?;

    // First, we write the header's len
    writer.write_all(&[header.len() as u8])?;

    // Then, we write the serialized header
    writer.write_all(&header)?;

    // And the metadata
    writer.write_all(&metadata)?;

    
    // We read the file in chunks, and submit jobs to the pool
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut step: u64 = 0;
    let mut collected = 0u64;
    let mut results = Vec::with_capacity(num_threads as usize);
    let mut n = 0;
    while {n = reader.read(&mut buffer)?; n > 0} {
        let is_last_chunk = n < CHUNK_SIZE;

        if is_last_chunk && step > 0 {
            buffer.extend_from_slice("ENK1END".as_bytes());
        }
        if step - collected >= num_threads as u64 {
            match pool.recv() {
                Ok(result) => {
                    match result {
                        Ok(chunk_result) => results.push(chunk_result),
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => return Err(e)
            }

            if results.len() >= num_threads as usize {
                results.sort_by_key(|result| result.index);

                for chunk_result in &results {
                    let len = (chunk_result.data.len() as u32).to_le_bytes();
                    writer.write_all(&len)?;
                    writer.write_all(&chunk_result.data)?;
                }

                // We free memory immediately
                results.clear();
            }
            collected += 1;
        }

        let job = EncryptChunkJob {
            index: step,
            data: buffer.clone(),
            master_nonce: Arc::new(master_nonce),
            compression: Arc::new(compression),
            cipher: cipher.clone()
        };

        pool.submit(EnkryptitJob::new(step, job))?;
        step += 1;
    }

    // Drain remaining results from workers (may still be processing)
    while collected < submitted {
        match pool.recv()? {
            Ok(result) => results.push(result),
            Err(e) => return Err(e.into()),
        }
        
        // Write in batches to avoid too many small writes
        if results.len() >= num_threads as usize {
            results.sort_by_key(|r| r.index);
            
            for chunk_result in &results {
                let len = (chunk_result.data.len() as u32).to_le_bytes();
                writer.write_all(&len)?;
                writer.write_all(&chunk_result.data)?;
            }
            
            results.clear();
        }
    }

    // Write any remaining chunks that didn't fill the buffer
    if !results.is_empty() {
        results.sort_by_key(|r| r.index);
        
        for chunk_result in &results {
            let len = (chunk_result.data.len() as u32).to_le_bytes();
            writer.write_all(&len)?;
            writer.write_all(&chunk_result.data)?;
        }
    }

    master_nonce.zeroize();
    Ok(encrypted_path)

}

// Helper function to sort and write a batch of chunks
fn write_batch(results: &mut Vec<ChunkResult>, writer: &mut BufWriter<File>) -> Result<(), EnkryptitError> {
    results.sort_by_key(|r| r.index);
    
    for chunk_result in results.iter() {
        let data = if chunk_result.data.ends_with(b"ENK1END") {
            // Strip ENK1END marker before writing
            &chunk_result.data[..(chunk_result.data.len() - 7)]
        } else {
            &chunk_result.data[..]
        };
        
        let len = (data.len() as u32).to_le_bytes();
        writer.write_all(&len)?;
        writer.write_all(data)?;
    }
    
    Ok(())
}