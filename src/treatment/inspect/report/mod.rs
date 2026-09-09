use crate::types::{CompressionType, KeyType, ParallelismType, Version};
mod display;

pub enum InspectionReport {
    PlaintextFile {
        name: String,
        directory: String,
        size: usize,
        permissions: Option<u32>,
        extension: String,
        mime_extension: String,
        predicted_compression_type: CompressionType,
        predicted_parallelism_type: ParallelismType,
    },

    Folder {
        name: String,
        directory: String,
        size: usize,
        permissions: Option<u32>,
    },

    EncryptedFile {
        name: String,
        directory: String,
        size: usize,
        version: Version,
        compression_type: CompressionType,
        predicted_parallelism_type: ParallelismType,
        keytype: KeyType,
        nonce: [u8; 24],
    },

    EncryptedArchive {
        name: String,
        directory: String,
        size: usize,
        version: Version,
        entries_number: u64,
        keytype: KeyType
    }
}
