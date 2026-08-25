use crate::context::EnkryptitContext;
use crate::encryption::encryption_flow::{decrypt_stream, encrypt_stream};
use crate::encryption::encryption_primitives::generate_nonce;
use crate::errors::EnkryptitError;
use crate::key::EnkryptitKey;
use crate::metadatas::{ArchiveHeader, MetaDatas};
use crate::parameters::params::EnkryptitParams;
use crate::types::KeyType::{self};
use std::fs::File;
use std::io::BufReader;
use std::io::{BufWriter, Write};
use std::io::{Seek, SeekFrom};
use zeroize::Zeroize;
use crate::types::Mode;


/// Public function that encrypts a file (it also resolves the key and keytype)
pub fn encrypt_file(
    path: &str,
    parameters: &EnkryptitParams,
    keytype: &KeyType,
    context: &mut EnkryptitContext
) -> Result<String, EnkryptitError> {
    // Creates the enkryptit key (and resolves keytype & key)
    let enkryptit_key = EnkryptitKey::resolve(Mode::Encrypting, keytype, context, path)?;

    // Opens the file
    let file = File::open(path)?;
    let total_size: u64 = file.metadata()?.len();

    let reader = BufReader::new(file);

    // Generates the nonce
    let mut master_nonce = generate_nonce();

    // Builds the metadata, and serialize it
    let metadata = MetaDatas::new(enkryptit_key.key_type_as_ref().clone(), parameters.compression, master_nonce).pack()?;

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
        enkryptit_key.key_as_ref(),
        parameters.compression,
        total_size,
    )?;

    // And finally, we `zeroize` the master nonce (key is automatically dropped and Zeroized).
    master_nonce.zeroize();

    writer.flush()?;

    Ok(encrypted_path)
}

/// Public function that decrypts a file
pub fn decrypt_file(
    path: &str,
    meta_bytes: &[u8],
    payload_offset: u64,
    context: &mut EnkryptitContext
) -> Result<String, EnkryptitError> {
    // First, we deserialize the metadata
    let metadatas: MetaDatas = postcard::from_bytes(meta_bytes)?;
    // And extract the compression type...
    let compression_type = metadatas.compression;
    // ... and the nonce
    let master_nonce = metadatas.nonce;

    // We resolve the key
    let enkryptit_key = EnkryptitKey::resolve(Mode::Decrypting, &metadatas.key_type, context, path)?;

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
        enkryptit_key.key_as_ref(),
        compression_type,
        master_nonce,
    )?;

    writer.flush()?;

    Ok(plain_path.to_string())
}
