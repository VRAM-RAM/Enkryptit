use crate::compression::{EnkryptitCompress, EnkryptitDecompress};
use crate::encryption::encryption_primitives::{decrypt_chunk, encrypt_chunk};
use crate::errors::EnkryptitError;
use crate::types::CHUNK_SIZE;
use crate::types::CompressionType;
use chacha20poly1305::KeyInit;
use chacha20poly1305::XChaCha20Poly1305;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Read, Write};
use std::ops::Deref;
use zeroize::Zeroize;

/// The heart of **Enkryptit** : it compresses and encrypts a stream of data.
pub fn encrypt_stream<R: Read, W: Write>(
    writer: &mut W,
    mut reader: R,
    master_nonce: [u8; 24],
    key: &[u8; 32],
    compression: CompressionType,
    total_size: u64,
) -> Result<u64, EnkryptitError> {
    // Steps counter
    let mut step: u64 = 0;
    // Buffer
    let mut buffer = vec![0u8; CHUNK_SIZE];
    // Bytes written
    let mut bytes_written: u64 = 0;

    let mut n = reader.read(&mut buffer)?;

    println!();
    // ProgressBar creation.
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Processed counter
    let mut total_processed: u64 = 0;

    // Cipher algorithm.
    let cipher = XChaCha20Poly1305::new(key.deref().into());

    // Output (for in-place compression)
    let mut output = vec![0u8; CHUNK_SIZE];

    while n > 0 {
        let mut next_buffer = vec![0u8; CHUNK_SIZE];
        let next_n = reader.read(&mut next_buffer)?;

        let mut data = buffer[..n].to_vec();
        if next_n == 0 {
            // If this is the end of the file, we add an 'end magic number'
            data.append(&mut b"ENK1END".to_vec());
        }

        // We compress, in-place, the data
        data.compress(&mut output, compression)?;

        // We encrypt the chunk
        encrypt_chunk(&mut output, &master_nonce, &cipher, step)?;

        let chunk_len = output.len() as u32;

        // We write the chunk len and the compressed + encrypted data
        writer.write_all(&chunk_len.to_le_bytes())?;
        writer.write_all(&output)?;

        // We update the output data
        bytes_written += 4 + output.len() as u64;

        // We update the processed total
        total_processed += n as u64;
        pb.set_position(total_processed);

        buffer = next_buffer;
        n = next_n;
        step += 1;
    }

    pb.finish();
    println!();
    Ok(bytes_written)
}

/// The heart of **Enkryptit** : it decompresses and decrypts a stream of data.
pub fn decrypt_stream<R: Read, W: Write>(
    writer: &mut W,
    mut reader: R,
    total_size: u64,
    key: &[u8; 32],
    compression: CompressionType,
    master_nonce: [u8; 24],
) -> Result<u64, EnkryptitError> {
    let mut step: u64 = 0;
    let mut bytes_consumed: u64 = 0;

    // ProgressBar creation.
    println!();
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut total_processed: u64 = 0;

    let cipher = XChaCha20Poly1305::new(key.deref().into());

    let mut output = vec![0u8; CHUNK_SIZE];

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

        bytes_consumed += 4;

        let len = u32::from_le_bytes(len_buf) as usize;

        let mut payload = vec![0u8; len];

        reader.read_exact(&mut payload)?;

        bytes_consumed += len as u64;

        // First, we decrypt the chunk in-place
        decrypt_chunk(&mut payload, &cipher, &master_nonce, step)?;

        // Then we decompress it in-place
        payload.decompress(&mut output, compression)?;

        // If the outputs ends with `ENK1END`, the process is finished
        if output.ends_with(b"ENK1END") {
            let new_len = output.len() - 7;
            output.truncate(new_len);
            writer.write_all(&output)?;
            total_processed += len as u64;
            pb.set_position(total_processed);
            pb.finish();
            println!();
            return Ok(bytes_consumed);
        }

        total_processed += len as u64;
        pb.set_position(total_processed);

        writer.write_all(&output)?;
        step += 1;
    }

    // We should add a new kind of error, something like `EndMagicNumberNotFound`, with a warning : file may have been alterated.
    pb.finish();
    println!();
    Ok(bytes_consumed)
}
