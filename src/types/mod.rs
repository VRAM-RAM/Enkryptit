mod key_type;
mod key_params;
mod compression_type;
mod parallelism_type;

/// Public exports
pub use key_type::KeyType;
pub use key_params::KeyParams;
pub use compression_type::CompressionType;
pub use parallelism_type::ParallelismType;

/// Version type
pub type Version = u8;

/// Chunk size of a bloc to compress & encrypt
pub const CHUNK_SIZE: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Encryption mode, used when resolving password in [`Enkryptitcontext`]
pub enum Mode {
    Encrypting,
    Decrypting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// To provide something clean when `matching` the interface in [`EnkryptitContext`]
pub enum Interface {
    Cli,
    Tui,
}