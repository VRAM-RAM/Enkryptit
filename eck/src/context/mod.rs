pub mod compression;
pub mod parallelism;

use std::fs::File;
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
/// Used for resolving :
/// - The password (during encryption / decryption)
/// - The compression type
/// - The parallelism type
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
    /// Returns an [`EnkryptitContext`] with the given values. 
    /// \
    /// **password** is an `Option<String>` because the user may not have provided any password. In that case, the context is created with `password : None` and the password is resolved later.
    /// \
    /// **compression_type** and **parallelism** can have the value `Auto`. In that case, they are resolved by the context later.
    pub fn new(interface: Interface, password: Option<String>, compression_type: CompressionType, parallelism: ParallelismType) -> Self {
        Self {
            interface,
            password: password.map(Zeroizing::new),
            compression_type,
            parallelism,
        }
    }

    /// Resolves the password and returns a `&Zeroizing<String>` (if `Ok<>`) that contains the password.
    /// \
    /// Its current flow is :
    /// - If password was not resolved (the stored `password` value is `None`) : 
    ///     - If the user is in `Cli` : it prompts the `enter_password` text, read user's input and stores the resolved password in `password`.
    ///     - If the user is using the `Tui` : it gets the password using [`rpassword::prompt_password`] and stores the resolved password in `password`.
    /// - Then, it returns the value that `password` contains (unwrapping it)
    /// 
    /// It is safe because the password is **always** resolved before unwrapping.
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

    /// Returns the `CompressionType` (if `Ok<>`).
    /// \
    /// - If the stored value of `compression_type` is [`CompressionType::Auto`], it infers the compression calling [`infer_compression()`].
    /// - Else, it returns the stored `CompressionType`
    pub fn resolve_compression(
        &self,
        path: &str,
    ) -> Result<CompressionType, EnkryptitError> {
        match self.compression_type {
            CompressionType::Auto => infer_compression(path),
            compression => Ok(compression),
        }
    }

    /// Returns the `ParallelismType` (if `Ok<>`) with the path of the file as `argument`.
    /// \
    /// It opens the file and reads its **size** before calling [`EnkryptitContext::resolve_parallelism_with_size()`].
    pub fn resolve_parallelism(
        &self,
        path: &str,
    ) -> Result<ParallelismType, EnkryptitError> {
        let file = File::open(path)?;
        let len = file.metadata()?.len();
        self.resolve_parallelism_with_size(len)
    }

    /// Returns the `ParallelismType` (if `Ok<>`) with the size of the file as `argument`.
    /// \
    /// - If the stored value of `parallelism` is [`ParallelismType::Auto`], it infers the parallelism type calling [`infer_parallelism()`].
    /// - Else, it returns the stored `ParallelismType`
    pub fn resolve_parallelism_with_size(
        &self,
        size: u64
    ) -> Result<ParallelismType, EnkryptitError> {
        match self.parallelism {
            ParallelismType::Auto => infer_parallelism(size),
            parallelism => Ok(parallelism)
        }
    }
}