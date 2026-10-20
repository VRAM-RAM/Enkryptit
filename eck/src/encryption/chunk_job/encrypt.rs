use crate::compression::EnkryptitCompress;
use crate::encryption::chunk_job::result::ChunkResult;
use crate::encryption::encryption_primitives::{derive_nonce, encrypt_chunk};
use crate::parallelism::executable::EnkryptitExecutable;
use crate::types::CHUNK_SIZE;
use crate::types::CompressionType;
use chacha20poly1305::XChaCha20Poly1305;
use std::sync::Arc;

pub struct EncryptChunkJob {
    pub index: u64,
    pub data: Vec<u8>,
    pub master_nonce: Arc<[u8; 24]>,
    pub compression: Arc<CompressionType>,
    pub cipher: Arc<XChaCha20Poly1305>,
}

impl EnkryptitExecutable for EncryptChunkJob {
    type Output = ChunkResult;

    fn execute(self) -> Result<Self::Output, crate::errors::EnkryptitError> {
        let nonce = derive_nonce(&self.master_nonce, self.index);

        // If the treatment does not require compression, we dont allocate an `output` vector, 
        // and directly encrypt
        if *self.compression == CompressionType::NoComp {
            let mut data = self.data;                    
            encrypt_chunk(&mut data, &nonce, &self.cipher, self.index)?;
            return Ok(ChunkResult { index: self.index, data });
        }

        let mut output = Vec::with_capacity(CHUNK_SIZE);  

        self.data.compress(&mut output, *self.compression)?;

        encrypt_chunk(&mut output, &nonce, &self.cipher, self.index)?;

        Ok(ChunkResult { index: self.index, data: output })
    }
}
