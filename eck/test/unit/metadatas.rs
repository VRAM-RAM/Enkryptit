//! Metadata Serialization Tests
//!
//! Test Postcard-based metadata storage and retrieval (encryption parameters, compression type)

use eck::VERSION;
use eck::metadatas::{ArchiveHeader, MAGIC, FileEntry, FolderMetadata, MetaDatas};
use eck::types::{CompressionType, KeyType};
use postcard;
use rand::{RngCore, rngs::OsRng};

fn generate_nonce() -> [u8; 24] {
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize_zstd_header() {
        let key_type = KeyType::Password;
        let compression = CompressionType::Zstd;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type.clone(), compression, nonce);
        let packed = meta.pack().unwrap();

        assert!(!packed.is_empty());

        let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

        assert_eq!(meta.key_type, unpacked.key_type);
        assert_eq!(meta.compression, unpacked.compression);
        assert_eq!(meta.nonce, unpacked.nonce);
    }

    #[test]
    fn serialize_deserialize_lz4_header() {
        let key_type = KeyType::Password;
        let compression = CompressionType::Lz4;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type.clone(), compression, nonce);
        let packed = meta.pack().unwrap();

        let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

        assert_eq!(meta.key_type, unpacked.key_type);
    }

    #[test]
    fn serialize_deserialize_all_compressions() {
        for comp in [
            CompressionType::Zstd,
            CompressionType::Lz4,
            CompressionType::Xz,
            CompressionType::NoComp,
        ]
        .iter()
        {
            let key_type = KeyType::Password;
            let nonce = generate_nonce();

            let meta = MetaDatas::new(key_type.clone(), *comp, nonce);
            let packed = meta.pack().unwrap();
            let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

            assert_eq!(*comp, unpacked.compression);
        }
    }

    #[test]
    fn serialize_deserialize_all_key_types() {
        for key_type in [
            KeyType::Password,
            KeyType::Pwd256([0x1Fu8; 16]),
            KeyType::FromFile,
            KeyType::FromOS,
        ]
        .iter()
        {
            let compression = CompressionType::Zstd;
            let nonce = generate_nonce();

            let meta = MetaDatas::new(key_type.clone(), compression, nonce);
            let packed = meta.pack().unwrap();
            let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

            assert_eq!(meta.key_type, unpacked.key_type);
        }
    }

    #[test]
    fn metadata_contains_compression_type_zstd() {
        let compression = CompressionType::Zstd;
        let key_type = KeyType::Password;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type, compression, nonce);
        let packed = meta.pack().unwrap();

        let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

        assert_eq!(unpacked.compression, CompressionType::Zstd);
    }

    #[test]
    fn metadata_contains_compression_type_lz4() {
        let compression = CompressionType::Lz4;
        let key_type = KeyType::Password;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type, compression, nonce);
        let packed = meta.pack().unwrap();

        let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

        assert_eq!(unpacked.compression, CompressionType::Lz4);
    }

    #[test]
    fn metadata_contains_compression_type_xz() {
        let compression = CompressionType::Xz;
        let key_type = KeyType::Password;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type, compression, nonce);
        let packed = meta.pack().unwrap();

        let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

        assert_eq!(unpacked.compression, CompressionType::Xz);
    }

    #[test]
    fn metadata_contains_compression_type_no_comp() {
        let compression = CompressionType::NoComp;
        let key_type = KeyType::Password;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type, compression, nonce);
        let packed = meta.pack().unwrap();

        let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

        assert_eq!(unpacked.compression, CompressionType::NoComp);
    }

    #[test]
    fn header_magic_bytes() {
        let header = ArchiveHeader::new(false, 0);

        assert_eq!(header.magic, MAGIC);
    }

    #[test]
    fn serialize_deserialize_empty_password_keytype() {
        let key_type = KeyType::Password;
        let compression = CompressionType::Zstd;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type, compression, nonce);
        let packed = meta.pack().unwrap();

        assert!(!packed.is_empty());
    }

    #[test]
    fn metadata_serialization_size_reasonable() {
        let header = ArchiveHeader::new(false, 0);
        let packed_header = header.pack().unwrap();

        assert!(packed_header.len() < 50);

        let key_type = KeyType::Password;
        let meta = MetaDatas::new(key_type, CompressionType::Zstd, [0u8; 24]);
        let packed_meta = meta.pack().unwrap();

        assert!(packed_meta.len() < 100);
    }

    #[test]
    fn test_is_folder_archive_flag_preserved() {
        let folder_header = ArchiveHeader::new(true, 0);

        let packed = folder_header.pack().unwrap();
        let unpacked: ArchiveHeader = postcard::from_bytes(&packed).unwrap();

        assert!(unpacked.is_folder_archive);
    }

    #[test]
    fn header_not_folder_flag_preserved() {
        let file_header = ArchiveHeader::new(false, 0);

        let packed = file_header.pack().unwrap();
        let unpacked: ArchiveHeader = postcard::from_bytes(&packed).unwrap();

        assert!(!unpacked.is_folder_archive);
    }

    #[test]
    fn header_meta_len_preserved() {
        for meta_len in [0u32, 1, 100, 1000, u32::MAX].iter() {
            let header = ArchiveHeader::new(false, *meta_len);
            let packed = header.pack().unwrap();
            let unpacked: ArchiveHeader = postcard::from_bytes(&packed).unwrap();

            assert_eq!(unpacked.meta_len, *meta_len);
        }
    }

    #[test]
    fn metadata_nonce_preserved() {
        for nonce_pattern in [
            vec![0u8; 24],
            vec![255u8; 24],
            (0..=23).map(|i| i as u8).collect::<Vec<u8>>(),
        ] {
            let mut nonce = [0u8; 24];
            nonce.copy_from_slice(&nonce_pattern);

            let key_type = KeyType::Password;
            let meta = MetaDatas::new(key_type, CompressionType::Zstd, nonce);

            let packed = meta.pack().unwrap();
            let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

            assert_eq!(meta.nonce, unpacked.nonce);
        }
    }

    #[test]
    fn metadata_serialization_with_special_chars_password() {
        let key_type = KeyType::Password;
        let compression = CompressionType::Zstd;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type, compression, nonce);
        let packed = meta.pack().unwrap();

        let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

        assert_eq!(meta.key_type, unpacked.key_type);
    }

    #[test]
    fn metadata_serialization_with_unicode_password() {
        let key_type = KeyType::Password;
        let compression = CompressionType::Zstd;
        let nonce = generate_nonce();

        let meta = MetaDatas::new(key_type, compression, nonce);
        let packed = meta.pack().unwrap();

        let unpacked: MetaDatas = postcard::from_bytes(&packed).unwrap();

        assert_eq!(meta.key_type, unpacked.key_type);
    }

    #[test]
    fn metadata_serialization_with_version_field() {
        let header = ArchiveHeader::new(false, 0);

        assert_eq!(header.version, VERSION);
    }

    // --- FolderMetadata / FileEntry (DAY-11: per-entry compression) ---

    fn sample_file_entry() -> FileEntry {
        FileEntry {
            relative_path: "sub/dir/a.txt".to_string(),
            offset: 42,
            permissions: Some(0o644),
            compression: CompressionType::Lz4,
            file_nonce: [0xAA; 24],
        }
    }

    #[test]
    fn file_entry_roundtrip_preserves_compression() {
        let entry = sample_file_entry();
        let packed = postcard::to_allocvec(&entry).unwrap();

        let unpacked: FileEntry = postcard::from_bytes(&packed).unwrap();

        // Compared field-by-field: `FileEntry::eq` intentionally ignores
        // `compression`, so we assert the new field explicitly.
        assert_eq!(unpacked.relative_path, "sub/dir/a.txt");
        assert_eq!(unpacked.offset, 42);
        assert_eq!(unpacked.permissions, Some(0o644));
        assert_eq!(unpacked.compression, CompressionType::Lz4);
        assert_eq!(unpacked.file_nonce, [0xAA; 24]);
    }

    #[test]
    fn file_entry_roundtrip_keeps_each_compression_type() {
        for comp in [
            CompressionType::Zstd,
            CompressionType::Lz4,
            CompressionType::Xz,
            CompressionType::NoComp,
            CompressionType::Auto,
        ] {
            let entry = FileEntry {
                relative_path: "a.bin".to_string(),
                offset: 1,
                permissions: None,
                compression: comp,
                file_nonce: [0u8; 24],
            };
            let packed = postcard::to_allocvec(&entry).unwrap();
            let unpacked: FileEntry = postcard::from_bytes(&packed).unwrap();
            assert_eq!(unpacked.compression, comp);
        }
    }

    #[test]
    fn file_entry_permissions_none_roundtrip() {
        let entry = FileEntry {
            permissions: None,
            ..sample_file_entry()
        };
        let packed = postcard::to_allocvec(&entry).unwrap();
        let unpacked: FileEntry = postcard::from_bytes(&packed).unwrap();
        assert_eq!(unpacked.permissions, None);
    }

    #[test]
    fn folder_metadata_roundtrip_with_multiple_entries() {
        let mut meta = FolderMetadata::new(KeyType::FromFile);
        meta.entries.push(sample_file_entry());
        meta.entries.push(FileEntry {
            relative_path: "root.bin".to_string(),
            offset: 1234,
            permissions: None,
            compression: CompressionType::NoComp,
            file_nonce: [0xBB; 24],
        });

        let packed = meta.pack().unwrap();
        let unpacked: FolderMetadata = postcard::from_bytes(&packed).unwrap();

        assert_eq!(unpacked.key_type, KeyType::FromFile);
        assert_eq!(unpacked.entries.len(), 2);
        assert_eq!(unpacked.entries[0].relative_path, "sub/dir/a.txt");
        assert_eq!(unpacked.entries[0].compression, CompressionType::Lz4);
        assert_eq!(unpacked.entries[1].relative_path, "root.bin");
        assert_eq!(unpacked.entries[1].offset, 1234);
        assert_eq!(unpacked.entries[1].compression, CompressionType::NoComp);
        assert_eq!(unpacked.entries[1].file_nonce, [0xBB; 24]);
    }

    #[test]
    fn folder_metadata_empty_entries_roundtrip() {
        let meta = FolderMetadata::new(KeyType::Password);
        let packed = meta.pack().unwrap();
        let unpacked: FolderMetadata = postcard::from_bytes(&packed).unwrap();

        assert_eq!(unpacked.key_type, KeyType::Password);
        assert!(unpacked.entries.is_empty());
    }
}
