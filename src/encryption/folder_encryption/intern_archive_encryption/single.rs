use crate::encryption::encryption_flow::{decrypt_stream, encrypt_stream};
use crate::errors::EnkryptitError;
use crate::types::CompressionType;
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Encrypt a single file into the archive stream with unique nonce per file
pub fn encrypt_single_file_into_archive(
    folder_path: &str,
    relative_path: &str,
    file_nonce: [u8; 24],
    compression: CompressionType,
    cipher_key: &[u8; 32],
    archive_path: &str,
) -> Result<u64, EnkryptitError> {
    let full_file_path = Path::new(folder_path).join(relative_path);

    if !PathBuf::from(&full_file_path).exists() {
        return Ok(0); // File no longer exists - skip silently
    }

    let file = File::open(full_file_path)?;
    let total_size: u64 = file.metadata()?.len();

    let reader = BufReader::new(file);

    // Append encrypted data to archive (stream mode)
    {
        let mut archive = BufWriter::new(File::options().append(true).open(archive_path)?);

        encrypt_stream(
            &mut archive,
            reader,
            file_nonce,
            cipher_key,
            compression,
            total_size,
        )
    }
}

/// Decrypt a single file from the archive stream using its unique nonce  
pub fn decrypt_single_file_from_archive(
    archive_path: &str,
    folder_path: &str,
    permissions: Option<u32>,
    relative_path: &str,
    file_nonce: [u8; 24],
    compressed_size: u64,
    compression: CompressionType,
    cipher_key: &[u8; 32],
    offset: u64,
) -> Result<u64, EnkryptitError> {
    let archive = File::open(Path::new(archive_path))?;
    let mut reader = BufReader::new(archive);
    reader.seek(SeekFrom::Start(offset))?;
    let full_file_path = Path::new(folder_path).join(relative_path);

    if let Some(parent) = full_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(full_file_path)?;

    if let Some(p) = permissions {
        file.set_permissions(std::fs::Permissions::from_mode(p))?;
    }

    {
        let mut writer = BufWriter::new(file);

        decrypt_stream(
            &mut writer,
            reader,
            compressed_size,
            cipher_key,
            compression,
            file_nonce,
        )
    }
}
