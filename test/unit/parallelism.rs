//! Parallelism Unit Tests
//!
//! Test the generic job/worker/pool abstractions (EnkryptitPool, EnkryptitJob) and the
//! `EncryptChunkJob` / `DecryptChunkJob` executables in isolation.

use std::sync::Arc;

use chacha20poly1305::{KeyInit, XChaCha20Poly1305};
use eck::context::{
    EnkryptitContext, LOW_BOUNDARY, MID_INFERIOR_BOUNDARY, MID_SUPERIOR_BOUNDARY, SUPERIOR_BOUNDARY,
};
use eck::encryption::chunk_job::{
    decrypt::DecryptChunkJob, encrypt::EncryptChunkJob, result::ChunkResult,
    submit_decrypt_chunk, submit_encrypt_chunk,
};
use eck::errors::EnkryptitError;
use eck::parallelism::EnkryptitJob;
use eck::parallelism::executable::EnkryptitExecutable;
use eck::parallelism::pool::EnkryptitPool;
use eck::types::{CHUNK_SIZE, CompressionType, Interface, ParallelismType};
use tempfile::TempDir;

/// A trivial executable that returns its index, so we can check that results
/// are correctly routed back from the workers.
struct EchoJob {
    value: u64,
}

impl EnkryptitExecutable for EchoJob {
    type Output = u64;

    fn execute(self) -> Result<Self::Output, EnkryptitError> {
        Ok(self.value)
    }
}

/// An executable that always fails, to check error propagation through the pool.
struct FailingJob;

impl EnkryptitExecutable for FailingJob {
    type Output = ();

    fn execute(self) -> Result<Self::Output, EnkryptitError> {
        Err(EnkryptitError::Encryption)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enkryptit_job_executes_its_task() {
        let job = EnkryptitJob::new(3, EchoJob { value: 42 });
        assert_eq!(job.index, 3);
        assert_eq!(job.execute().unwrap(), 42);
    }

    #[test]
    fn pool_rejects_zero_workers() {
        use eck::errors::EnkryptitError;
        match EnkryptitPool::<EchoJob>::new(0) {
            Err(EnkryptitError::InvalidWorkerCount) => {}
            _ => panic!("expected InvalidWorkerCount from a zero-sized pool"),
        }
    }

    #[test]
    fn pool_returns_all_outputs_with_four_workers() {
        let pool = EnkryptitPool::<EchoJob>::new(4).unwrap();
        for i in 0..20u64 {
            pool.submit(EnkryptitJob::new(i, EchoJob { value: i }))
                .unwrap();
        }

        let mut received = Vec::new();
        for _ in 0..20 {
            received.push(pool.recv().unwrap().unwrap());
        }
        received.sort_unstable();
        assert_eq!(received, (0..20).collect::<Vec<_>>());
    }

    #[test]
    fn pool_returns_all_outputs_with_single_worker() {
        let pool = EnkryptitPool::<EchoJob>::new(1).unwrap();
        for i in 0..5u64 {
            pool.submit(EnkryptitJob::new(i, EchoJob { value: i }))
                .unwrap();
        }

        let mut received = Vec::new();
        for _ in 0..5 {
            received.push(pool.recv().unwrap().unwrap());
        }
        received.sort_unstable();
        assert_eq!(received, (0..5).collect::<Vec<_>>());
    }

    #[test]
    fn pool_more_jobs_than_workers() {
        // More jobs than workers: the bounded channel buffers, workers drain,
        // and every result must still be received.
        let pool = EnkryptitPool::<EchoJob>::new(2).unwrap();
        for i in 0..50u64 {
            pool.submit(EnkryptitJob::new(i, EchoJob { value: i }))
                .unwrap();
        }

        let mut received = Vec::new();
        for _ in 0..50 {
            received.push(pool.recv().unwrap().unwrap());
        }
        received.sort_unstable();
        assert_eq!(received, (0..50).collect::<Vec<_>>());
    }

    #[test]
    fn pool_propagates_execution_errors() {
        let pool = EnkryptitPool::<FailingJob>::new(2).unwrap();
        for _ in 0..3 {
            pool.submit(EnkryptitJob::new(0, FailingJob)).unwrap();
        }

        for _ in 0..3 {
            let inner = pool.recv().unwrap();
            assert!(inner.is_err(), "executable error must be surfaced");
        }
    }

    #[test]
    fn encrypt_chunk_job_roundtrip() {
        let key = [0x11u8; 32];
        let cipher = Arc::new(XChaCha20Poly1305::new(&key.into()));
        let master_nonce = Arc::new([0x22u8; 24]);
        let compression = Arc::new(CompressionType::NoComp);

        let data = b"parallel chunk roundtrip".to_vec();

        let encrypt_job = EncryptChunkJob {
            index: 0,
            data,
            master_nonce: master_nonce.clone(),
            compression: compression.clone(),
            cipher: cipher.clone(),
        };
        let ChunkResult {
            index,
            data: encrypted,
        } = encrypt_job.execute().unwrap();
        assert_eq!(index, 0);

        let decrypt_job = DecryptChunkJob {
            index: 0,
            data: encrypted,
            master_nonce,
            compression,
            cipher,
        };
        let ChunkResult { data: restored, .. } = decrypt_job.execute().unwrap();
        assert_eq!(restored, b"parallel chunk roundtrip");
    }

    #[test]
    fn encrypt_chunk_job_keeps_index() {
        let key = [0x33u8; 32];
        let cipher = Arc::new(XChaCha20Poly1305::new(&key.into()));
        let master_nonce = Arc::new([0x44u8; 24]);
        let compression = Arc::new(CompressionType::Lz4);

        for (idx, payload) in [b"a".to_vec(), b"bb".to_vec(), b"ccc".to_vec()]
            .into_iter()
            .enumerate()
        {
            let job = EncryptChunkJob {
                index: idx as u64,
                data: payload,
                master_nonce: master_nonce.clone(),
                compression: compression.clone(),
                cipher: cipher.clone(),
            };
            let result = job.execute().unwrap();
            assert_eq!(result.index, idx as u64);
        }
    }

    #[test]
    fn chunk_output_length_is_bounded() {
        // Encryption/compression writes into a pre-allocated CHUNK_SIZE buffer,
        // so the produced chunk must never exceed CHUNK_SIZE.
        let key = [0x55u8; 32];
        let cipher = Arc::new(XChaCha20Poly1305::new(&key.into()));
        let master_nonce = Arc::new([0x66u8; 24]);
        let compression = Arc::new(CompressionType::NoComp);

        let data = vec![0xABu8; 1024];
        let job = EncryptChunkJob {
            index: 0,
            data,
            master_nonce,
            compression,
            cipher,
        };
        let result = job.execute().unwrap();
        assert!(result.data.len() <= CHUNK_SIZE);
        assert!(!result.data.is_empty());
    }

    // Kept reference-free: just demonstrates the EnkryptitExecutable trait bound
    // used by the pool is object-safe and usable from the crate root.
    #[test]
    fn executable_trait_is_usable() {
        let job = EnkryptitJob::new(7, EchoJob { value: 7 });
        let output: u64 = job.task.execute().unwrap();
        assert_eq!(output, 7);
    }

    // --- Auto parallelism inference (DAY-10) ---

    fn cpus() -> u8 {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1) as u8
    }

    fn auto_context() -> EnkryptitContext {
        EnkryptitContext::new(Interface::Cli, None, CompressionType::NoComp, ParallelismType::Auto)
    }

    #[test]
    fn auto_parallelism_tiny_files_are_single() {
        let ctx = auto_context();
        assert_eq!(
            ctx.resolve_parallelism_with_size(0).unwrap(),
            ParallelismType::Single
        );
        assert_eq!(
            ctx.resolve_parallelism_with_size(1).unwrap(),
            ParallelismType::Single
        );
        assert_eq!(
            ctx.resolve_parallelism_with_size(LOW_BOUNDARY - 1).unwrap(),
            ParallelismType::Single
        );
    }

    #[test]
    fn auto_parallelism_low_zone_is_multithread_4() {
        let ctx = auto_context();
        assert_eq!(
            ctx.resolve_parallelism_with_size(LOW_BOUNDARY).unwrap(),
            ParallelismType::MultiThread(4.min(cpus()))
        );
        assert_eq!(
            ctx.resolve_parallelism_with_size(MID_INFERIOR_BOUNDARY - 1).unwrap(),
            ParallelismType::MultiThread(4.min(cpus()))
        );
    }

    #[test]
    fn auto_parallelism_mid_inferior_zone_is_multithread_6() {
        let ctx = auto_context();
        assert_eq!(
            ctx.resolve_parallelism_with_size(MID_INFERIOR_BOUNDARY).unwrap(),
            ParallelismType::MultiThread(6.min(cpus()))
        );
        assert_eq!(
            ctx.resolve_parallelism_with_size(MID_SUPERIOR_BOUNDARY - 1).unwrap(),
            ParallelismType::MultiThread(6.min(cpus()))
        );
    }

    #[test]
    fn auto_parallelism_mid_superior_zone_is_multithread_8() {
        let ctx = auto_context();
        assert_eq!(
            ctx.resolve_parallelism_with_size(MID_SUPERIOR_BOUNDARY).unwrap(),
            ParallelismType::MultiThread(8.min(cpus()))
        );
        assert_eq!(
            ctx.resolve_parallelism_with_size(SUPERIOR_BOUNDARY - 1).unwrap(),
            ParallelismType::MultiThread(8.min(cpus()))
        );
    }

    #[test]
    fn auto_parallelism_superior_zone_uses_every_core() {
        let ctx = auto_context();
        assert_eq!(
            ctx.resolve_parallelism_with_size(SUPERIOR_BOUNDARY).unwrap(),
            ParallelismType::MultiThread(cpus())
        );
        assert_eq!(
            ctx.resolve_parallelism_with_size(SUPERIOR_BOUNDARY * 2).unwrap(),
            ParallelismType::MultiThread(cpus())
        );
    }

    #[test]
    fn explicit_parallelism_bypasses_size_inference() {
        let ctx = EnkryptitContext::new(
            Interface::Cli,
            None,
            CompressionType::NoComp,
            ParallelismType::MultiThread(3),
        );
        // Even with a 0-byte "file", the explicit type is returned untouched.
        assert_eq!(
            ctx.resolve_parallelism_with_size(0).unwrap(),
            ParallelismType::MultiThread(3)
        );

        let ctx = EnkryptitContext::new(Interface::Cli, None, CompressionType::NoComp, ParallelismType::Single);
        assert_eq!(
            ctx.resolve_parallelism_with_size(SUPERIOR_BOUNDARY * 999).unwrap(),
            ParallelismType::Single
        );
    }

    #[test]
    fn resolve_parallelism_uses_path_file_size() {
        let dir = TempDir::new().unwrap();
        let small = dir.path().join("small.bin");
        std::fs::File::create(&small).unwrap().set_len(16).unwrap();
        let ctx = auto_context();
        assert_eq!(
            ctx.resolve_parallelism(small.to_str().unwrap()).unwrap(),
            ParallelismType::Single
        );

        let big = dir.path().join("big.bin");
        std::fs::File::create(&big)
            .unwrap()
            .set_len(LOW_BOUNDARY + 1)
            .unwrap();
        assert_eq!(
            ctx.resolve_parallelism(big.to_str().unwrap()).unwrap(),
            ParallelismType::MultiThread(4.min(cpus()))
        );
    }

    // --- chunk_job submit helpers (DAY-12) ---

    #[test]
    fn chunk_job_submit_helpers_roundtrip_through_pool() {
        let encrypt_pool = EnkryptitPool::<EncryptChunkJob>::new(2).unwrap();
        let key = [0x99u8; 32];
        let cipher = Arc::new(XChaCha20Poly1305::new(&key.into()));
        let master_nonce = Arc::new([0x44u8; 24]);
        let compression = Arc::new(CompressionType::NoComp);

        let payloads = [
            b"alpha".to_vec(),
            b"beta-beta".to_vec(),
            b"gamma chunk payload".to_vec(),
        ];

        for (i, data) in payloads.iter().cloned().enumerate() {
            submit_encrypt_chunk(
                &encrypt_pool,
                i as u64,
                data,
                master_nonce.clone(),
                compression.clone(),
                cipher.clone(),
            )
            .unwrap();
        }

        let mut encrypted = Vec::new();
        for _ in 0..payloads.len() {
            encrypted.push(encrypt_pool.recv().unwrap().unwrap());
        }
        encrypted.sort_by_key(|r| r.index);

        let decrypt_pool = EnkryptitPool::<DecryptChunkJob>::new(2).unwrap();
        for result in encrypted {
            submit_decrypt_chunk(
                &decrypt_pool,
                result.index,
                result.data,
                master_nonce.clone(),
                compression.clone(),
                cipher.clone(),
            )
            .unwrap();
        }

        let mut restored = Vec::new();
        for _ in 0..payloads.len() {
            restored.push(decrypt_pool.recv().unwrap().unwrap());
        }
        restored.sort_by_key(|r| r.index);

        let restored: Vec<&[u8]> = restored.iter().map(|r| r.data.as_slice()).collect();
        let expected: Vec<&[u8]> = payloads.iter().map(|p| p.as_slice()).collect();
        assert_eq!(restored, expected);
    }
}
