use crate::encryption::chunk_job::{encrypt::EncryptChunkJob, decrypt::DecryptChunkJob, submit_decrypt_chunk, submit_encrypt_chunk};
use crate::encryption::file::read_file;
use crate::errors::EnkryptitError;
use crate::parallelism::pool::EnkryptitPool;
use crate::types::CompressionType;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::types::CHUNK_SIZE;
use gradient_bar::GradientProgressBar;
use crate::encryption::file_encryption::multithread::receive_results;
use crate::encryption::file_encryption::multithread::write_batch;
use chacha20poly1305::{KeyInit, XChaCha20Poly1305};
use std::io::Write;
use crate::encryption::file_encryption::multithread::write_batch_plain;

/// Encrypt a single file into the archive stream with unique nonce per file, using multithreading
pub fn encrypt_multithreading_file_into_archive(
    folder_path: &str,
    relative_path: &str,
    file_nonce: [u8; 24],
    compression: CompressionType,
    cipher_key: &[u8; 32],
    archive_path: &str,
    pool: &EnkryptitPool<EncryptChunkJob>,
    num_threads: u8,
) -> Result<u64, EnkryptitError> {
    let full_file_path = Path::new(folder_path).join(relative_path);

    if !PathBuf::from(&full_file_path).exists() {
        return Ok(0); // File no longer exists - skip silently
        // TODO! Add a smooth skipping + logging system !!!!!!!
    }

    let mut file = read_file(full_file_path)?;
    
    // We prepare the shared cipher for Multithreading
    let cipher = Arc::new(XChaCha20Poly1305::new(cipher_key.into()));

    // Bytes written
    let mut bytes_written: u64 = 0;

    let mut archive = BufWriter::new(File::options().append(true).open(archive_path)?);

    let arc_compression = Arc::new(compression);
    let arc_nonce = Arc::new(file_nonce);

    // We read the file in chunks, and submit jobs to the pool
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut step: u64 = 0;
    let mut results = Vec::with_capacity(num_threads as usize);
    let mut submitted = 0u8;
    let pb = GradientProgressBar::with_total_steps(file.estimated_steps, "Encrypting file...");


    loop {
        let bytes_read = file.reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        // If we submitted as many jobs as we have workers, we receive and write the results.
        if submitted >= num_threads {
            receive_results(&mut results, &pool, num_threads)?;

            bytes_written += write_batch(&mut results, &mut archive)?;

            submitted = 0;
        }

        submit_encrypt_chunk(pool, step, buffer[..bytes_read].to_vec(), arc_nonce.clone(), arc_compression.clone(), cipher.clone())?;

        submitted += 1;
        step += 1;
        pb.update(step);
    }

    // At the end of the loop{}, if we have still pending jobs, we receive and treat their output.
    if submitted > 0 {
        receive_results(&mut results, &pool, submitted)?;
        bytes_written += write_batch(&mut results, &mut archive)?;
    }

    // We write the ending 'magic'
    archive.write_all(b"ENK1END")?;
    bytes_written += 7;

    Ok(bytes_written)
}

/// Decrypt a single file from the archive stream using its unique nonce  
pub fn decrypt_multithreading_file_from_archive(
    archive_path: &str,
    folder_path: &str,
    permissions: Option<u32>,
    relative_path: &str,
    file_nonce: [u8; 24],
    compressed_size: u64,
    compression: CompressionType,
    cipher_key: &[u8; 32],
    offset: u64,
    pool: &EnkryptitPool<DecryptChunkJob>,
    num_threads: u8
) -> Result<u64, EnkryptitError> {
    let archive = File::open(Path::new(archive_path))?;
    let mut reader = BufReader::new(archive);
    reader.seek(SeekFrom::Start(offset))?;
    let full_file_path = Path::new(folder_path).join(relative_path);

    if let Some(parent) = full_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(full_file_path)?;
    let estimated_max_steps = compressed_size / CHUNK_SIZE as u64;

    if let Some(p) = permissions {
        file.set_permissions(std::fs::Permissions::from_mode(p))?;
    }

    // We prepare the shared cipher for Multithreading
    let cipher = Arc::new(XChaCha20Poly1305::new(cipher_key.into()));

    // We create the Arc<> wrappers around compression type and master nonce
    let arc_compression = Arc::new(compression);
    let arc_nonce = Arc::new(file_nonce);

    // We read the file in chunks, and submit jobs to the pool
    let mut step: u64 = 0;
    let mut results = Vec::with_capacity(num_threads as usize);
    let mut submitted = 0u8;

    // Bytes written
    let mut bytes_written: u64 = 0;

    let pb = GradientProgressBar::with_total_bytes(estimated_max_steps, "Decrypting...");

    let mut writer = BufWriter::new(file);

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
                // TODO! Add a smooth error system + logging system
            }

            break;
        }

        let len = u32::from_le_bytes(len_buf) as usize;

        let mut payload = vec![0u8; len];

        reader.read_exact(&mut payload)?;

        // If we submitted as jobs as we have workers, we receive and write the results.
        if submitted >= num_threads {
            receive_results(&mut results, &pool, num_threads)?;

            bytes_written += write_batch_plain(&mut results, &mut writer)?;

            submitted = 0;
        }

        // We create the job
        submit_decrypt_chunk(pool, step, payload, arc_nonce.clone(), arc_compression.clone(), cipher.clone())?;

        // We increment
        submitted += 1;
        step += 1;
        pb.update(step);
    }

    // At the end of the loop{}, if we have still pending jobs, we receive and treat their output.
    if submitted > 0 {
        receive_results(&mut results, &pool, submitted)?;
        bytes_written += write_batch_plain(&mut results, &mut writer)?;
    }

    pb.finish();

    writer.flush()?;

    Ok(bytes_written)
}
