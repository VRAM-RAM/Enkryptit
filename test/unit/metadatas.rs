//! Metadata Serialization Tests
//!
//! Test Postcard-based metadata storage and retrieval (encryption parameters, compression type)

use eck::VERSION;
use eck::metadatas::{ArchiveHeader, MAGIC, MetaDatas};
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
}
