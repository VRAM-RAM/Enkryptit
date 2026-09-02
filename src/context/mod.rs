mod compression;
mod parallelism;

use crate::context::compression::{infer_compression};
use crate::context::parallelism::infer_parallelism;
use crate::enter_password;
use crate::errors::EnkryptitError;
use crate::types::{CompressionType, Interface, ParallelismType};
use zeroize::Zeroizing;

pub const LOW_BOUNDARY: u64 = 50 << 20;                // 50 MiB
pub const MID_INFERIOR_BOUNDARY: u64 = 250 << 20;      // 250 MiB
pub const MID_SUPERIOR_BOUNDARY: u64 = 1 << 30;        // 1 GiB
pub const SUPERIOR_BOUNDARY: u64 = 5 << 30;            // 5 GiB

/// The Context, passed trough the program.
/// \
/// For now, only used to resolve, if needed, the password for encryption / decryption
pub struct EnkryptitContext {
    /// The Interface currently used :
    /// \
    /// `Cli` or `Tui`
    pub interface: Interface,
    /// The context *can* contain a password (if the user provided one with `-p`)
    pub password: Option<Zeroizing<String>>,

    /// The CompressionType used by the operation, and resolved by the context.
    pub compression_type: CompressionType,

    /// The ParallelismType used by the operation, and resolved by the context.
    /// This parallelism type is only used for intern entries and files.
    /// For folder encryption itself, we infer at the beginning of the treatment,
    /// since the parallelism resolution for a folder is based on :
    /// - It's size
    /// - The number of files it contains
    pub parallelism: ParallelismType,
}

impl EnkryptitContext {
    pub fn new(interface: Interface, password: Option<String>, compression_type: CompressionType, parallelism: ParallelismType) -> Self {
        Self {
            interface,
            password: password.map(Zeroizing::new),
            compression_type,
            parallelism,
        }
    }

    pub fn resolve_password(&mut self) -> Result<&Zeroizing<String>, EnkryptitError> {
        if self.password.is_none() {
            let pwd = match self.interface {
                Interface::Cli => {
                    enter_password!();
                    let mut pwd = String::new();
                    std::io::stdin().read_line(&mut pwd)?;
                    pwd.trim().to_string()
                }

                Interface::Tui => rpassword::prompt_password("Enter password: ")
                    .map_err(EnkryptitError::TuiError)?,
            };

            self.password = Some(Zeroizing::new(pwd));
        }

        Ok(self.password.as_ref().unwrap())
    }

    pub fn resolve_compression(
        &self,
        path: &str,
    ) -> Result<CompressionType, EnkryptitError> {
        match self.compression_type {
            CompressionType::Auto => infer_compression(path),
            compression => Ok(compression),
        }
    }

    pub fn resolve_parallelism(
        &self,
        path: &str,
    ) -> Result<ParallelismType, EnkryptitError> {
        match self.parallelism {
            ParallelismType::Auto => infer_parallelism(path),
            type_ => Ok(type_)
        }
    }
}