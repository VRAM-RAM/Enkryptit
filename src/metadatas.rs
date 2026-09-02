use crate::{
    VERSION,
    errors::EnkryptitError,
    types::{CompressionType, KeyType, Version},
};

use postcard::to_allocvec;
use serde::{Deserialize, Serialize};

/// Magic number of **Enkryptit!**.
pub const MAGIC: [u8; 4] = [0x45, 0x4E, 0x4B, 0x31];

#[derive(Serialize, Deserialize, Clone)]
/// Metadata structure. Contains :
/// - The Keytype
/// - The CompressionType
/// - The Nonce
pub struct MetaDatas {
    pub key_type: KeyType,
    pub compression: CompressionType,
    pub nonce: [u8; 24],
}

impl MetaDatas {
    pub fn new(key_type: KeyType, compression: CompressionType, nonce: [u8; 24]) -> Self {
        Self {
            key_type,
            compression,
            nonce,
        }
    }

    /// Method that serializes the `MetaData`
    pub fn pack(&self) -> Result<Vec<u8>, EnkryptitError> {
        Ok(to_allocvec(self)?)
    }
}

/// An archive header.
/// Contains :
/// - The Magic number
/// - The version of **Enkryptit!**
/// - a boolean that indicates if the archive is a folder or a file
/// - the len of the metadata
#[derive(Clone, Serialize, Deserialize)]
pub struct ArchiveHeader {
    pub magic: [u8; 4],
    pub version: Version,
    pub is_folder_archive: bool,
    pub meta_len: u32,
}

impl ArchiveHeader {
    pub fn new(is_folder_archive: bool, meta_len: u32) -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
            is_folder_archive,
            meta_len: meta_len, // Will be set after serialization
        }
    }

    pub fn pack(&self) -> Result<Vec<u8>, EnkryptitError> {
        Ok(to_allocvec(self)?)
    }
}

/// A file entry header. Contains :
/// - The relative path
/// - The offset
/// - The permissions
/// - The file nonce
#[derive(Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub relative_path: String,
    pub offset: u64,
    pub permissions: Option<u32>,
    pub compression: CompressionType,
    pub file_nonce: [u8; 24],
}

impl PartialEq for FileEntry {
    fn eq(&self, other: &Self) -> bool {
        self.relative_path == other.relative_path
            && self.offset == other.offset
            && self.permissions == other.permissions
            && self.file_nonce == other.file_nonce
    }
}

impl Eq for FileEntry {}

/// The Metadata of a Folder. Contains :
/// - The CompressionType
/// - The KeyType
/// - The entries that the folder contains
#[derive(Serialize, Deserialize)]
pub struct FolderMetadata {
    pub key_type: KeyType,
    pub entries: Vec<FileEntry>,
}

impl FolderMetadata {
    pub fn new(key_type: KeyType) -> Self {
        Self {
            key_type,
            entries: Vec::new(),
        }
    }

    pub fn pack(&self) -> Result<Vec<u8>, EnkryptitError> {
        Ok(to_allocvec(self)?)
    }
}
