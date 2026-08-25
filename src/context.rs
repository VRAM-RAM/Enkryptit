use crate::errors::EnkryptitError;
use zeroize::Zeroizing;
use crate::enter_password;
use crate::types::Interface;

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
}

impl EnkryptitContext {
    pub fn new(interface: Interface, password: Option<String>) -> Self {
        Self { interface: interface, password:  password.map(Zeroizing::new) }
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

                Interface::Tui => {
                    rpassword::prompt_password("Enter password: ").map_err(|e| EnkryptitError::TuiError(e))?
                }
            };

            self.password = Some(Zeroizing::new(pwd));
        }

        Ok(self.password.as_ref().unwrap())
    }
}

