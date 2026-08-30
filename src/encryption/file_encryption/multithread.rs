use crate::encryption::encryption_primitives::generate_nonce;
use crate::errors::EnkryptitError;
use crate::key::EnkryptitKey;
use crate::metadatas::{ArchiveHeader, MetaDatas};
use crate::parallelism::EnkryptitJob;
use crate::parallelism::executable::EnkryptitExecutable;
use crate::parallelism::pool::EnkryptitPool;
use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use std::io::{Seek, SeekFrom};
use std::sync::Arc;
use chacha20poly1305::{KeyInit, XChaCha20Poly1305};
use crate::types::{CHUNK_SIZE, CompressionType};
use crate::encryption::encrypt_chunk_job::{EncryptChunkJob, DecryptChunkJob};
use crate::encryption::encrypt_chunk_job::ChunkResult;

/// Public function that `encrypts` a file, using a pool of workers. `num_threads` determines the number of workers.
/// The function first initialize the pool of workers. Then, when processing, it submits a jobs to the pool of workers. 
/// When the number of submitted jobs is equal to the number of threads, we receive every result of *chunk encryption and compression*, 
/// sort it in the right order, and write it in the encrypted file. 
pub fn encrypt_multithread_file(path: &str, compression: CompressionType, enkryptit_key: EnkryptitKey, num_threads: u8) -> Result<String, EnkryptitError> {
    // First, we initialize the workers pool
    let pool = EnkryptitPool::<EncryptChunkJob>::new(num_threads as usize)?;

    // Then, we open the file and create the reader
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Generates the nonce
    let master_nonce = generate_nonce();

    // We prepare the shared cipher for Multithreading
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

    // We create the Arc<> wrappers around compression type and master nonce
    let arc_compression = Arc::new(compression);
    let arc_nonce = Arc::new(master_nonce);

    
    // We read the file in chunks, and submit jobs to the pool
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut step: u64 = 0;
    let mut results = Vec::with_capacity(num_threads as usize);
    let mut submitted = 0u8;

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        // If we submitted as many jobs as we have workers, we receive and write the results.
        if submitted >= num_threads {
            receive_results(&mut results, &pool, num_threads)?;

            write_batch(&mut results, &mut writer)?;

            submitted = 0;
        }

        // We create the job
        let job = EncryptChunkJob {
            index: step,
            data: buffer[..bytes_read].to_vec(),
            master_nonce: arc_nonce.clone(),
            compression: arc_compression.clone(),
            cipher: cipher.clone()
        };

        // We submit the job to the pool
        pool.submit(EnkryptitJob { index: step, task: job })?;
        
        // We increment
        submitted += 1;
        step += 1;
    }   

    // At the end of the loop{}, if we have still pending jobs, we receive and treat their output.
    if submitted > 0 {
        receive_results(&mut results, &pool, submitted)?;
        write_batch(&mut results, &mut writer)?;
    }

    // We write the ending 'magic'
    writer.write_all(b"ENK1END")?;

    Ok(encrypted_path)
}

/// Private function that decrypts a file using a pool of workers. `num_threads` determines the number of workers.
/// The function first initialize the pool of workers. Then, when processing, it submits a jobs to the pool of workers. 
/// When the number of submitted jobs is equal to the number of threads, we receive every result of *chunk encryption and compression*, 
/// sort it in the right order, and write it in the encrypted file. 
/// If we *read* `ENK1`, we catch it and try to find `END` (that makes the `ENK1END` magic number). If so, we break.
pub fn decrypt_multithread_file(path: &str, payload_offset: u64, enkryptit_key: EnkryptitKey, master_nonce: [u8; 24], compression: CompressionType, num_threads: u8) -> Result<String, EnkryptitError> {
    // First, we initialize the workers pool
    let pool = EnkryptitPool::<DecryptChunkJob>::new(num_threads as usize)?;
    
    // We open the file
    let file = File::open(path)?; 
    let mut reader = BufReader::new(file);

    let plain_path = path.strip_suffix(".encky").unwrap_or(path);
    // Create a placeholder for the new file
    let new_file = std::fs::File::create(plain_path)?;
    let mut writer = BufWriter::new(new_file);

    reader.seek(SeekFrom::Start(payload_offset))?;


    // We prepare the shared cipher for Multithreading
    let cipher = Arc::new(XChaCha20Poly1305::new(enkryptit_key.key_as_ref().into()));

    // We create the Arc<> wrappers around compression type and master nonce
    let arc_compression = Arc::new(compression);
    let arc_nonce = Arc::new(master_nonce);

    // We read the file in chunks, and submit jobs to the pool
    let mut step: u64 = 0;
    let mut results = Vec::with_capacity(num_threads as usize);
    let mut submitted = 0u8;

    loop {
        let mut len_buf = [0u8; 4];

        match reader.read_exact(&mut len_buf) {
            // If we got no error, we continue
            Ok(_) => {}
            // If we have an Eof, we exit the loop
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            // If we have another error, we convert it into an EnkryptitError and return it
            Err(e) => return Err(e.into()),
        }

        // We try to detect a possible ENK1END. 
        // If we have one, we break.
        // Why putting this here ? Because our format is :
        //
        // [LEN][CHUNK][ENK1END]
        // So, it'll read the [LEN], then the [CHUNK], but, when trying to read the next len, it will read `ENK1`... that isn't a length.
        // In consequence, we need to catch this case, and treat it as an exception.
        if &len_buf == b"ENK1" {
            let mut end = [0u8; 3];
            reader.read_exact(&mut end)?;

            if &end != b"END" {
                // invalid magic
                // EndMagicNumberNotFound / other error
            }

            break;
        }

        let len = u32::from_le_bytes(len_buf) as usize;

        let mut payload = vec![0u8; len];

        reader.read_exact(&mut payload)?;

        // If we submitted as jobs as we have workers, we receive and write the results.
        if submitted >= num_threads {
            receive_results(&mut results, &pool, num_threads)?;

            write_batch_plain(&mut results, &mut writer)?;

            
            submitted = 0;
        }

        // We create the job
        let job = DecryptChunkJob {
            index: step,
            data: payload.clone(),
            master_nonce: arc_nonce.clone(),
            compression: arc_compression.clone(),
            cipher: cipher.clone()
        };

        // We submit the job to the pool
        pool.submit(EnkryptitJob { index: step, task: job })?;
        
        // We increment
        submitted += 1;
        step += 1;
    }   

    // At the end of the loop{}, if we have still pending jobs, we receive and treat their output.
    if submitted > 0 {
        receive_results(&mut results, &pool, submitted)?;
        write_batch_plain(&mut results, &mut writer)?;
    }

    Ok(plain_path.to_string())
}

/// Helper function to sort and write a batch of chunks, and then clear the `results` vector. Used when encrypting only.
fn write_batch(results: &mut Vec<ChunkResult>, writer: &mut BufWriter<File>) -> Result<(), EnkryptitError> {
    results.sort_by_key(|r| r.index);
    
    for chunk_result in results.iter() {
        let data = &chunk_result.data;
        
        let len = (data.len() as u32).to_le_bytes();
        writer.write_all(&len)?;
        writer.write_all(data)?;
    }
    
    results.clear();

    Ok(())
}

/// Helper function to sort and write a batch of decrypted chunks (the plaintext),
/// without any length prefix, and then clear the `results` vector. Used when decrypting only.
fn write_batch_plain(results: &mut Vec<ChunkResult>, writer: &mut BufWriter<File>) -> Result<(), EnkryptitError> {
    results.sort_by_key(|r| r.index);

    for chunk_result in results.iter() {
        writer.write_all(&chunk_result.data)?;
    }

    results.clear();

    Ok(())
}

fn receive_results<T: EnkryptitExecutable + Send + 'static>(results: &mut Vec<T::Output>, pool: &EnkryptitPool<T>, num_threads: u8) -> Result<(), EnkryptitError> {
    for _ in 0..num_threads {
        match pool.recv() {
            Ok(result) => {
                match result {
                    Ok(chunk_result) => results.push(chunk_result),
                    Err(e) => return Err(e)
                }
            }
            Err(e) => return Err(e)
        }
    }

    Ok(())
}