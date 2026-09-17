pub mod compression;
pub mod context;
pub mod conversions;
pub mod encryption;
pub mod errors;
pub mod frontend;
pub mod key;
pub mod metadatas;
pub mod parallelism;
pub mod parameters;
pub mod treatment;
pub mod types;
pub mod diagnostic;
pub mod directory;

use crate::types::Version;

/// The version of `Enkryptit!`
/// Enkryptit versions are separated between :
/// - Nightly versions
/// - Stable versions
/// \
/// This constant is only for `Nightly` version.
/// \
/// So, for example, if you download **Enkryptit!** v0.1.4, it is the 4th nightly version of 1st stable version.
pub const VERSION: Version = 3;
