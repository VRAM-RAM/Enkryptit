use std::sync::Arc;
use crate::encryption::encryption_primitives::{decrypt_chunk, derive_nonce, encrypt_chunk};
use crate::parallelism::executable::EnkryptitExecutable;
use crate::types::CHUNK_SIZE;
use crate::{types::CompressionType};
use crate::compression::{EnkryptitCompress, EnkryptitDecompress};
use chacha20poly1305::XChaCha20Poly1305;


pub struct ChunkResult {
    pub index: u64,
    pub data: Vec<u8>
}

pub struct EncryptChunkJob {
    pub index: u64,
    pub data: Vec<u8>,
    pub master_nonce: Arc<[u8; 24]>,
    pub compression: Arc<CompressionType>,
    pub cipher: Arc<XChaCha20Poly1305>
}

impl EnkryptitExecutable for EncryptChunkJob {
    type Output = ChunkResult;

    fn execute(self) -> Result<Self::Output, crate::errors::EnkryptitError> {
        let nonce = derive_nonce(&self.master_nonce, self.index);

        let mut output = vec![0u8; CHUNK_SIZE];

        self.data.compress(&mut output, *self.compression);

        encrypt_chunk(&mut output, &nonce, &self.cipher, self.index)?;
        
        Ok(ChunkResult { index: self.index, data: output })
    }
}


pub struct DecryptChunkJob {
    pub index: u64,
    pub data: Vec<u8>,
    pub master_nonce: Arc<[u8; 24]>,
    pub compression: Arc<CompressionType>,
    pub cipher: Arc<XChaCha20Poly1305>
}

impl EnkryptitExecutable for DecryptChunkJob {
    type Output = ChunkResult;

    fn execute(self) -> Result<Self::Output, crate::errors::EnkryptitError> {
        let nonce = derive_nonce(&self.master_nonce, self.index);

        let mut ndata = self.data;
        let mut output = vec![0u8; CHUNK_SIZE];

        decrypt_chunk(&mut ndata, &self.cipher, &nonce, self.index)?;

        ndata.decompress(&mut output, *self.compression);

        Ok(ChunkResult { index: self.index, data: output })
    }
}

