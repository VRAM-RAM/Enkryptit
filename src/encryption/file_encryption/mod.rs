use crate::encryption::encryption_flow::{decrypt_stream, encrypt_stream};
use crate::encryption::encryption_primitives::generate_nonce;
use crate::errors::EnkryptitError;
use crate::keygen::{derive_key, key_from_password};
use crate::metadatas::{ArchiveHeader, MetaDatas};
use crate::parameters::params::EnkryptitParams;
use crate::types::KeyType::{self, Pwd256};
use std::fs::File;
use std::io::BufReader;
use std::io::{BufWriter, Write};
use std::io::{Seek, SeekFrom};
use zeroize::Zeroize;

/// Public function that encrypts a file (it also resolves the key and keytype)
pub fn encrypt_file(
    path: &str,
    key: [u8; 32],
    parameters: &EnkryptitParams,
    key_type: KeyType,
) -> Result<String, EnkryptitError> {
    // Resolves both key and keytype
    let (mut key, new_key_type) = match key_type {
        KeyType::Password(pwd) => {
            let (key, salt) = key_from_password(&pwd)?;
            (key, Pwd256(salt))
        }
        _ => (key, key_type.clone()),
    };

    // Opens the file
    let file = File::open(path)?;
    let total_size: u64 = file.metadata()?.len();

    let reader = BufReader::new(file);

    // Generates the nonce
    let mut master_nonce = generate_nonce();

    // Builds the metadata, and serialize it
    let metadata = MetaDatas::new(new_key_type, parameters.compression, master_nonce).pack()?;

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
        reader,
        master_nonce,
        key,
        parameters.compression,
        total_size,
    )?;

    // And finally, we `zeroize` both the master nonce and the key.
    master_nonce.zeroize();
    key.zeroize();

    writer.flush()?;

    Ok(encrypted_path)
}

/// Public function that decrypts a file
pub fn decrypt_file(
    path: &str,
    meta_bytes: &[u8],
    key: [u8; 32],
    payload_offset: u64,
    key_type: KeyType,
) -> Result<String, EnkryptitError> {
    // First, we deserialize the metadata
    let metadatas: MetaDatas = postcard::from_bytes(meta_bytes)?;
    // And extract the compression type...
    let compression_type = metadatas.compression;
    // ... and the nonce
    let master_nonce = metadatas.nonce;

    // We resolve the key
    let mut key = if let KeyType::Password(pwd) = &key_type {
        let salt = match &metadatas.key_type {
            KeyType::Pwd256(s) => *s,
            _ => return Err(EnkryptitError::CorruptedFile),
        };

        derive_key(&pwd, salt)?
    } else {
        key
    };

    // We open the file
    let file = File::open(path)?;
    let total_size: u64 = file.metadata()?.len();
    let mut reader = BufReader::new(file);

    let plain_path = path.strip_suffix(".encky").unwrap_or(path);
    // Create a placeholder for the new file
    let new_file = std::fs::File::create(plain_path)?;
    let mut writer = BufWriter::new(new_file);

    reader.seek(SeekFrom::Start(payload_offset))?;

    // Decrypts the stream in-place
    let _ = decrypt_stream(
        &mut writer,
        reader,
        total_size,
        key,
        compression_type,
        master_nonce,
    )?;

    key.zeroize();
    writer.flush()?;

    Ok(plain_path.to_string())
}
