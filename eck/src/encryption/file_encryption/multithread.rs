use crate::encryption::chunk_job::{
    decrypt::DecryptChunkJob, encrypt::EncryptChunkJob, submit_decrypt_chunk, submit_encrypt_chunk,
};
use std::collections::BTreeMap;
use crate::encryption::encryption_primitives::generate_nonce;
use crate::encryption::file::read_file;
use crate::encryption::shared_cipher;
use crate::errors::EnkryptitError;
use crate::key::EnkryptitKey;
use crate::metadatas::{ArchiveHeader, MetaDatas};
use crate::parallelism::pool::EnkryptitPool;
use crate::types::{CHUNK_SIZE, CompressionType};
use gradient_bar::GradientProgressBar;
use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};
use std::io::{Seek, SeekFrom};
use std::sync::Arc;

/// Public function that `encrypts` a file, using a pool of workers. `num_threads` determines the number of workers.
/// The function first initialize the pool of workers. Then, when processing, it submits a jobs to the pool of workers.
/// When the number of submitted jobs is equal to the number of threads, we receive every result of *chunk encryption and compression*,
/// sort it in the right order, and write it in the encrypted file.
pub fn encrypt_multithread_file(
    path: &str,
    compression: CompressionType,
    enkryptit_key: EnkryptitKey,
    num_threads: u8,
) -> Result<String, EnkryptitError> {
    // First, we initialize the workers pool
    let pool = EnkryptitPool::<EncryptChunkJob>::new(num_threads as usize)?;

    // Then, we open the file and create the reader
    let mut file = read_file(path)?;

    // Generates the nonce
    let master_nonce = generate_nonce();

    // We prepare the shared cipher for Multithreading
    let cipher = shared_cipher(enkryptit_key.key_as_ref());

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

    // The pending results, out-of-order completed chunks
    let mut pending: BTreeMap<u64, Vec<u8>> = BTreeMap::new();
    
    // number of chunks written (and index of the next chunk to write)
    let mut next_to_write: u64 = 0;

    let max_in_flight: u64 = 2 * num_threads as u64;

    let pb = GradientProgressBar::with_total_steps(file.estimated_steps, "Encrypting...");

    loop {
        let bytes_read = file.reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        while step - next_to_write >= max_in_flight {
            let r = pool.recv()??;
            pending.insert(r.index, r.data);
            flush_ready(&mut pending, &mut next_to_write, &mut writer, true)?;
        }

        // We create the job and submit it
        submit_encrypt_chunk(
            &pool,
            step,
            buffer[..bytes_read].to_vec(),
            arc_nonce.clone(),
            arc_compression.clone(),
            cipher.clone(),
        )?;

        // We increment
        step += 1;
        pb.update(step);
    }

    // last drain
    while next_to_write < step {
        let r = pool.recv()??;
        pending.insert(r.index, r.data);
        flush_ready(&mut pending, &mut next_to_write, &mut writer, true)?;
    }

    // We write the ending 'magic'
    writer.write_all(b"ENK1END")?;
    writer.flush()?;

    pb.finish();

    Ok(encrypted_path)
}

/// Private function that decrypts a file using a pool of workers. `num_threads` determines the number of workers.
/// The function first initialize the pool of workers. Then, when processing, it submits a jobs to the pool of workers.
/// When the number of submitted jobs is equal to the number of threads, we receive every result of *chunk encryption and compression*,
/// sort it in the right order, and write it in the encrypted file.
/// If we *read* `ENK1`, we catch it and try to find `END` (that makes the `ENK1END` magic number). If so, we break.
pub fn decrypt_multithread_file(
    path: &str,
    payload_offset: u64,
    enkryptit_key: EnkryptitKey,
    master_nonce: [u8; 24],
    compression: CompressionType,
    num_threads: u8,
) -> Result<String, EnkryptitError> {
    // First, we initialize the workers pool
    let pool = EnkryptitPool::<DecryptChunkJob>::new(num_threads as usize)?;

    // We open the file
    let mut file = read_file(path)?;

    let plain_path = path.strip_suffix(".encky").unwrap_or(path);

    // If the plain path is the same path as the encrypted path, we abort and return an error
    if plain_path == path {
        return Err(EnkryptitError::InvalidEncryptedFileName)
    }
    
    // Create a placeholder for the new file
    let new_file = std::fs::File::create(plain_path)?;
    let mut writer = BufWriter::new(new_file);

    file.reader.seek(SeekFrom::Start(payload_offset))?;

    // We prepare the shared cipher for Multithreading
    let cipher = shared_cipher(enkryptit_key.key_as_ref());

    // We create the Arc<> wrappers around compression type and master nonce
    let arc_compression = Arc::new(compression);
    let arc_nonce = Arc::new(master_nonce);

    // We read the file in chunks, and submit jobs to the pool
    let mut step: u64 = 0;
    
    // The pending results, out-of-order completed chunks
    let mut pending: BTreeMap<u64, Vec<u8>> = BTreeMap::new();
    
    // number of chunks written (and index of the next chunk to write)
    let mut next_to_write: u64 = 0;

    let max_in_flight: u64 = 2 * num_threads as u64;

    let pb = GradientProgressBar::with_total_bytes(file.estimated_steps, "Decrypting...");

    loop {
        let mut len_buf = [0u8; 4];

        match file.reader.read_exact(&mut len_buf) {
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
            file.reader.read_exact(&mut end)?;

            if &end != b"END" {
                // invalid magic
                // EndMagicNumberNotFound / other error
            }

            break;
        }

        let len = u32::from_le_bytes(len_buf) as usize;

        let mut payload = vec![0u8; len];

        file.reader.read_exact(&mut payload)?;

        // If we submitted as jobs as we have workers, we receive and write the results.
        while step - next_to_write >= max_in_flight {
            let r = pool.recv()??;
            pending.insert(r.index, r.data);
            flush_ready(&mut pending, &mut next_to_write, &mut writer, false)?;
        }

        submit_decrypt_chunk(
            &pool,
            step,
            payload,
            arc_nonce.clone(),
            arc_compression.clone(),
            cipher.clone(),
        )?;

        // We increment
        step += 1;
        pb.update(step);
    }

    // last drain
    while next_to_write < step {
        let r = pool.recv()??;
        pending.insert(r.index, r.data);
        flush_ready(&mut pending, &mut next_to_write, &mut writer, false)?;
    }

    pb.finish();

    writer.flush()?;

    Ok(plain_path.to_string())
}


/// Helper that writes the pending results of the encryption/decryption
/// \
/// When encrypting, `with_len_prefix` is `true`, and, when decrypting, it is `false`.
/// \
/// Returns the number of bytes written.
pub fn flush_ready(
    pending: &mut BTreeMap<u64, Vec<u8>>,
    next_to_write: &mut u64,
    writer: &mut BufWriter<File>,
    with_len_prefix: bool,
) -> Result<u64, EnkryptitError> {
    let mut written: u64 = 0;
    while let Some(data) = pending.remove(next_to_write) {
        if with_len_prefix {
            writer.write_all(&(data.len() as u32).to_le_bytes())?;
            written += 4;
        }

        writer.write_all(&data)?;
        written += data.len() as u64;
        *next_to_write += 1;
    }
    Ok(written)
}