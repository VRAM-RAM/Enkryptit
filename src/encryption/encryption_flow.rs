use crate::compression::{EnkryptitCompress, EnkryptitDecompress};
use crate::encryption::encryption_primitives::{decrypt_chunk, encrypt_chunk};
use crate::errors::EnkryptitError;
use gradient_bar::progress_bar::{GradientProgressBar};
use crate::types::CHUNK_SIZE;
use crate::types::CompressionType;
use chacha20poly1305::KeyInit;
use chacha20poly1305::XChaCha20Poly1305;
use std::io::{Read, Write};

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

    let pb = GradientProgressBar::with_total_bytes(total_size, "Encrypting...");

    // Processed counter
    let mut total_processed: u64 = 0;

    // Cipher algorithm.
    let cipher = XChaCha20Poly1305::new(key.into());

    // Output (for in-place compression)
    let mut output = vec![0u8; CHUNK_SIZE];

    while n > 0 {
        let mut next_buffer = vec![0u8; CHUNK_SIZE];
        let next_n = reader.read(&mut next_buffer)?;

        let data = buffer[..n].to_vec();

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
        pb.update(total_processed);

        buffer = next_buffer;
        n = next_n;
        step += 1;
    }

    writer.write_all(b"ENK1END")?;
    bytes_written += 7;

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
    let pb = GradientProgressBar::with_total_bytes(total_size, "Decrypting...");


    let mut total_processed: u64 = 0;

    let cipher = XChaCha20Poly1305::new(key.into());

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

        bytes_consumed += len as u64;

        // First, we decrypt the chunk in-place
        decrypt_chunk(&mut payload, &cipher, &master_nonce, step)?;

        // Then we decompress it in-place
        payload.decompress(&mut output, compression)?;

        total_processed += len as u64;
        pb.update(total_processed);

        writer.write_all(&output)?;
        step += 1;
    }

    // We should add a new kind of error, something like `EndMagicNumberNotFound`, with a warning : file may have been alterated.
    pb.finish();
    println!();
    Ok(bytes_consumed)
}
