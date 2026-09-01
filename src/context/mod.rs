mod compression;

use crate::context::compression::{infer_compression};
use crate::enter_password;
use crate::errors::EnkryptitError;
use crate::types::{CompressionType, Interface};
use zeroize::Zeroizing;


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
}

impl EnkryptitContext {
    pub fn new(interface: Interface, password: Option<String>, compression_type: CompressionType) -> Self {
        Self {
            interface,
            password: password.map(Zeroizing::new),
            compression_type
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
}