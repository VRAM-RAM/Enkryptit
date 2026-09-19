use crate::types::{CompressionType, KeyType, ParallelismType, Version};
mod display;


pub enum InspectionReport {
    PlaintextFile {
        name: Option<String>,
        directory: Option<String>,
        size: Option<u64>,
        permissions: Option<u32>,
        extension: Option<String>,
        mime_extension: Option<String>,
        predicted_compression_type: Option<CompressionType>,
        predicted_parallelism_type: Option<ParallelismType>,
    },

    Folder {
        name: Option<String>,
        directory: Option<String>,
        size: Option<u64>,
        permissions: Option<u32>,
    },

    EncryptedFile {
        name: Option<String>,
        directory: Option<String>,
        size: Option<u64>,
        version: Version,
        compression_type: Option<CompressionType>,
        predicted_parallelism_type: Option<ParallelismType>,
        keytype: Option<KeyType>,
        nonce: Option<[u8; 24]>,
    },

    EncryptedArchive {
        name: Option<String>,
        directory: Option<String>,
        size: Option<u64>,
        version: Version,
        entries_number: Option<u64>,
        keytype: Option<KeyType>
    }
}
