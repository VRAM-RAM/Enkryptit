use crate::encryption::encryption_flow::{decrypt_stream, encrypt_stream};
use crate::encryption::encryption_primitives::generate_nonce;
use crate::encryption::file::read_file;
use crate::errors::EnkryptitError;
use crate::key::EnkryptitKey;
use crate::metadatas::{ArchiveHeader, MetaDatas};
use crate::types::CompressionType;
use std::io::{BufWriter, Write};
use std::io::{Seek, SeekFrom};
use zeroize::Zeroize;

/// Public function that encrypts a file (it also resolves the key and keytype) - single thread
pub fn encrypt_file_single(
    path: &str,
    compression: CompressionType,
    enkryptit_key: EnkryptitKey,
) -> Result<String, EnkryptitError> {
    // Opens the file
    let file = read_file(path)?;

    // Generates the nonce
    let mut master_nonce = generate_nonce();

    // Builds the metadata, and serialize it
    let metadata = MetaDatas::new(
        enkryptit_key.key_type_as_ref().clone(),
        compression,
        master_nonce,
    )
    .pack()?;

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

    // We encrypt the stream in place
    let _ = encrypt_stream(
        &mut writer,
        file.reader,
        master_nonce,
        enkryptit_key.key_as_ref(),
        compression,
        file.len,
    )?;

    // And finally, we `zeroize` the master nonce (key is automatically dropped and Zeroized).
    master_nonce.zeroize();

    writer.flush()?;

    Ok(encrypted_path)
}

/// Public function that decrypts a file (single-thread)
pub fn decrypt_file_single(
    path: &str,
    payload_offset: u64,
    enkryptit_key: EnkryptitKey,
    master_nonce: [u8; 24],
    compression: CompressionType,
) -> Result<String, EnkryptitError> {
    // We open the file
    let mut file = read_file(path)?;

    let plain_path = path.strip_suffix(".encky").unwrap_or(path);
    // Create a placeholder for the new file
    let new_file = std::fs::File::create(plain_path)?;
    let mut writer = BufWriter::new(new_file);

    file.reader.seek(SeekFrom::Start(payload_offset))?;

    // Decrypts the stream in-place
    let _ = decrypt_stream(
        &mut writer,
        file.reader,
        file.len,
        enkryptit_key.key_as_ref(),
        compression,
        master_nonce,
    )?;

    writer.flush()?;

    Ok(plain_path.to_string())
}
