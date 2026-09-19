//! # Enkryptit!
//! \
//! **Enkryptit!** is a Rust-written cli-tool / interactive manager for file and folder encryption. 
//!
//! > This project is currently a work in progress. It is **not audited** for production security.
//! 
//! ## Code Layout
//! ```txt
//! src/
//! ├── main.rs                 # entry point, clap CLI definition & dispatch
//! ├── lib.rs                  # library entry point (used by tests)
//! ├── types/                  # enums: CompressionType, KeyType, ParallelismType, KeyParams
//! │   ├── compression_type.rs
//! │   ├── key_type.rs
//! │   ├── parallelism_type.rs
//! │   └── key_params.rs
//! ├── frontend/               # everything user-facing
//! │   ├── cli/                # command-line interface
//! │   │   ├── inspection.rs   # eck inspect
//! │   │   ├── params_helpers.rs
//! │   │   └── treatment.rs    # object treatment (single / multiple paths)
//! │   ├── tui/                # terminal UI (inquire + rfd)
//! │   │   ├── action.rs
//! │   │   ├── browse.rs       # file/folder pickers
//! │   │   ├── help.rs
//! │   │   ├── input.rs        # TuiInput trait (mockable)
//! │   │   ├── parameters.rs
//! │   │   └── treatment.rs
//! │   ├── treat_output.rs     # renders success / error / info badges
//! │   └── mod.rs
//! ├── diagnostic/             # logs, reports, errors & error rendering
//! │   ├── logging/            # EnkryptitLogger (tracing, daily rotation → log.txt)
//! │   ├── output/             # EnkryptitOutput, diagnostics & snippets (miette)
//! │   └── report/             # comfy-table Report rendering
//! ├── context/                # EnkryptitContext (password, config resolution)
//! │   ├── compression.rs
//! │   └── parallelism.rs      # Auto inference helpers
//! ├── key/                    # key resolution, derivation, storage
//! │   ├── derivation.rs       # Argon2id key_from_password / derive_key
//! │   ├── generation.rs
//! │   ├── resolve.rs          # key from file / OS keyring
//! │   ├── storage.rs
//! │   └── mod.rs              # EnkryptitKey + LockedKey (mlock / zeroize)
//! ├── encryption/
//! │   ├── encryption_primitives.rs  # XChaCha20-Poly1305 chunk encrypt/decrypt, nonce derivation
//! │   ├── encryption_flow.rs
//! │   ├── chunk_job/          # EncryptChunkJob / DecryptChunkJob / ChunkResult
//! │   ├── file.rs
//! │   ├── file_encryption/    # single & multithreaded file encryption
//! │   │   ├── single.rs
//! │   │   ├── multithread.rs
//! │   │   └── mod.rs
//! │   ├── folder_encryption/  # folder ↔ .encky archive
//! │   │   ├── collect_entry.rs
//! │   │   ├── entry/          # FileEntry handling
//! │   │   ├── intern_archive_encryption/  # per-entry single/multi treatment
//! │   │   ├── single.rs       # entry loop
//! │   │   └── mod.rs
//! │   └── mod.rs
//! ├── compression.rs          # zstd / lz4 / xz / none (with retry logic)
//! ├── parallelism/            # worker pool for chunk jobs
//! │   └── pool.rs             # EnkryptitPool
//! ├── parameters/             # persisted parameters (config.json)
//! │   ├── argon2id_parameters.rs  # static Argon2id params (128 MiB, 3, 1)
//! │   └── params.rs           # EnkryptitParams load / save
//! ├── metadatas.rs            # ArchiveHeader, MetaDatas, FolderMetadata, FileEntry, MAGIC
//! ├── treatment/              # high-level object treatment & inspection
//! ├── conversions.rs
//! ├── directory.rs            # ProjectDirs (com.olruix.Enkryptit)
//! └── errors.rs               # EnkryptitError
//! ```
//!
//! ## License
//!
//! This project is dual-licensed under:
//!
//! * **CeCILL-B License** (French law compliant, fully compatible with GNU GPL/Apache)
//! * **Apache License, Version 2.0**
//!
//! Choose the one that best fits your needs.
//! 
//! ## Contact
//! 
//!* **Developer:** Olruix ([VRAM-RAM](https://github.com/VRAM-RAM))


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
