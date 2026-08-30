//! Multithreaded File Encryption Integration Tests
//!
//! Exercises `encrypt_file` / `decrypt_file` with `ParallelismType::MultiThread`
//! for a variety of compression algorithms, thread counts, and file sizes, and
//! verifies interoperability between the single-threaded and multithreaded code
//! paths (a file encrypted on one path decrypts on the other).

use std::fs;
use std::io::{BufReader, Read};

use eck::context::EnkryptitContext;
use eck::encryption::file_encryption::{decrypt_file, encrypt_file};
use eck::metadatas::ArchiveHeader;
use eck::parameters::params::EnkryptitParams;
use eck::types::{CompressionType, KeyParams, KeyType, ParallelismType};
use postcard::from_bytes;
use tempfile::NamedTempFile;

const KEY_FROM_FILE: KeyType = KeyType::FromFile;

fn params_multi(compression: CompressionType, threads: u8) -> EnkryptitParams {
    EnkryptitParams {
        compression,
        key_params: KeyParams::File,
        parallelism: ParallelismType::MultiThread(threads),
    }
}

/// Reads the header + metadata of a single-file `.encky` archive and returns
/// the deserialized metadata plus the byte offset at which the encrypted payload
/// starts (`1 + header_len + meta_len`).
fn read_file_archive_meta(archive_path: &str) -> (Vec<u8>, u64) {
    let file = fs::File::open(archive_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut len_buf = [0u8; 1];
    reader.read_exact(&mut len_buf).unwrap();
    let header_len = len_buf[0] as usize;

    let mut header_bytes = vec![0u8; header_len];
    reader.read_exact(&mut header_bytes).unwrap();
    let header: ArchiveHeader = from_bytes(&header_bytes).unwrap();

    let meta_len = header.meta_len as usize;
    let mut meta = vec![0u8; meta_len];
    reader.read_exact(&mut meta).unwrap();

    let payload_offset = (1 + header_len as u64 + meta_len as u64) as u64;
    (meta, payload_offset)
}

/// Round-trips `content` through `encrypt_file`/`decrypt_file` using the
/// parallelism selected during both phases. Asserts the restored bytes equal
/// the originals.
fn assert_roundtrip(
    content: &[u8],
    enc_parallelism: ParallelismType,
    dec_parallelism: ParallelismType,
    compression: CompressionType,
) {
    let tmp_file = NamedTempFile::new().unwrap();
    fs::write(tmp_file.path(), content).unwrap();

    let enc_ctx = &mut EnkryptitContext::new(eck::types::Interface::Cli, None);
    let enc_params = EnkryptitParams {
        compression,
        key_params: KeyParams::File,
        parallelism: enc_parallelism,
    };

    let archive_path = encrypt_file(
        tmp_file.path().to_str().unwrap(),
        &enc_params,
        &KEY_FROM_FILE,
        enc_ctx,
    )
    .expect("multithread encryption must succeed");

    let (meta, payload_offset) = read_file_archive_meta(&archive_path);

    let dec_ctx = &mut EnkryptitContext::new(eck::types::Interface::Cli, None);
    decrypt_file(
        &archive_path,
        &meta,
        payload_offset,
        dec_ctx,
        dec_parallelism,
    )
    .expect("decryption must succeed");

    let restored = fs::read(tmp_file.path()).unwrap();
    assert_eq!(restored, content, "roundtrip must restore original bytes");
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Same-mode roundtrips (encrypt & decrypt with the same parallelism) ----

    #[test]
    fn multithread_no_comp_roundtrip() {
        assert_roundtrip(
            b"multithread nocomp content",
            ParallelismType::MultiThread(4),
            ParallelismType::MultiThread(4),
            CompressionType::NoComp,
        );
    }

    #[test]
    fn multithread_zstd_roundtrip() {
        assert_roundtrip(
            b"multithread zstd content",
            ParallelismType::MultiThread(4),
            ParallelismType::MultiThread(4),
            CompressionType::Zstd,
        );
    }

    #[test]
    fn multithread_lz4_roundtrip() {
        assert_roundtrip(
            b"multithread lz4 content",
            ParallelismType::MultiThread(4),
            ParallelismType::MultiThread(4),
            CompressionType::Lz4,
        );
    }

    #[test]
    fn multithread_xz_roundtrip() {
        assert_roundtrip(
            b"multithread xz content",
            ParallelismType::MultiThread(4),
            ParallelismType::MultiThread(4),
            CompressionType::Xz,
        );
    }

    // ---- Single worker (degenerate pool) must still work ----

    #[test]
    fn multithread_with_single_worker_roundtrip() {
        assert_roundtrip(
            b"single worker multithread path",
            ParallelismType::MultiThread(1),
            ParallelismType::MultiThread(1),
            CompressionType::Zstd,
        );
    }

    // ---- Worker-count interoperability (multithread -> multithread) ----

    #[test]
    fn encrypt_multithread_decrypt_with_different_thread_counts() {
        // Different thread counts across the two phases should interoperate.
        assert_roundtrip(
            b"different thread counts",
            ParallelismType::MultiThread(2),
            ParallelismType::MultiThread(8),
            CompressionType::Lz4,
        );
    }

    // ---- Cross-mode interoperability (single <-> multithread) ----
    // Since both the single-threaded and multithreaded paths write the same
    // on-disk format (each chunk length-prefixed, `ENK1END` written at the end),
    // files must be freely interchangeable between the two.

    #[test]
    fn encrypt_multithread_decrypt_single() {
        for compression in [
            CompressionType::NoComp,
            CompressionType::Zstd,
            CompressionType::Lz4,
            CompressionType::Xz,
        ] {
            assert_roundtrip(
                b"encrypt multithread, decrypt single",
                ParallelismType::MultiThread(4),
                ParallelismType::Single,
                compression,
            );
        }
    }

    #[test]
    fn encrypt_single_decrypt_multithread() {
        for compression in [
            CompressionType::NoComp,
            CompressionType::Zstd,
            CompressionType::Lz4,
            CompressionType::Xz,
        ] {
            assert_roundtrip(
                b"encrypt single, decrypt multithread",
                ParallelismType::Single,
                ParallelismType::MultiThread(4),
                compression,
            );
        }
    }

    #[test]
    fn encrypt_single_decrypt_multithread_large_file() {
        // A large (multi-chunk) file encrypted single-threaded must decrypt
        // correctly through the multithreaded worker pool.
        let large = vec![0xFAu8; 3 * eck::types::CHUNK_SIZE];
        assert_roundtrip(
            &large,
            ParallelismType::Single,
            ParallelismType::MultiThread(4),
            CompressionType::NoComp,
        );
    }

    // ---- Large multi-chunk files ----

    #[test]
    fn multithread_large_file_roundtrip() {
        // Large enough to span several CHUNK_SIZE blocks so multiple workers
        // process distinct chunks.
        let large = vec![0xCDu8; 3 * eck::types::CHUNK_SIZE];
        assert_roundtrip(
            &large,
            ParallelismType::MultiThread(4),
            ParallelismType::MultiThread(4),
            CompressionType::NoComp,
        );
    }

    #[test]
    fn multithread_file_just_over_chunk_size_roundtrip() {
        let content = vec![0xEEu8; eck::types::CHUNK_SIZE + 1];
        assert_roundtrip(
            &content,
            ParallelismType::MultiThread(4),
            ParallelismType::MultiThread(4),
            CompressionType::Zstd,
        );
    }

    #[test]
    fn multithread_many_small_chunks_roundtrip() {
        // Highly compressible content that nonetheless produces many chunks.
        let pattern: Vec<u8> = (0..1024u32).map(|i| (i % 251) as u8).collect();
        let content = pattern.repeat(eck::types::CHUNK_SIZE / 1024 * 3);
        assert_roundtrip(
            &content,
            ParallelismType::MultiThread(8),
            ParallelismType::MultiThread(8),
            CompressionType::Xz,
        );
    }

    // ---- Error handling ----

    #[test]
    fn encrypt_multithread_nonexistent_file_errors() {
        let ctx = &mut EnkryptitContext::new(eck::types::Interface::Cli, None);
        let params = params_multi(CompressionType::NoComp, 4);
        let result = encrypt_file(
            "/this/path/does/not/exist.txt",
            &params,
            &KEY_FROM_FILE,
            ctx,
        );
        assert!(result.is_err());
    }
}
