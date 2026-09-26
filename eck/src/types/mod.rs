mod compression_type;
mod key_params;
mod key_type;
mod parallelism_type;

pub use compression_type::CompressionType;
pub use key_params::KeyParams;
/// Public exports
pub use key_type::KeyType;
pub use parallelism_type::ParallelismType;

/// Version type
pub type Version = u8;

/// Chunk size of a bloc to compress & encrypt
pub const CHUNK_SIZE: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Encryption mode, used when resolving password in [`EnkryptitContext`](crate::context::EnkryptitContext)
pub enum Mode {
    Encrypting,
    Decrypting,
}
