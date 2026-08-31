//! Parallelism Unit Tests
//!
//! Test the generic job/worker/pool abstractions (EnkryptitPool, EnkryptitJob) and the
//! `EncryptChunkJob` / `DecryptChunkJob` executables in isolation.

use std::sync::Arc;

use chacha20poly1305::{KeyInit, XChaCha20Poly1305};
use eck::encryption::encrypt_chunk_job::{ChunkResult, DecryptChunkJob, EncryptChunkJob};
use eck::errors::EnkryptitError;
use eck::parallelism::EnkryptitJob;
use eck::parallelism::executable::EnkryptitExecutable;
use eck::parallelism::pool::EnkryptitPool;
use eck::types::{CHUNK_SIZE, CompressionType};

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
}
