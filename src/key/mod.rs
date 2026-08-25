use crate::key::derivation::{derive_key, key_from_password};
use crate::key::resolve::{resolve_key_from_file, resolve_key_from_os};
use crate::{context::EnkryptitContext, types::KeyType};
use crate::errors::EnkryptitError;
pub mod generation;
pub mod storage;
pub mod derivation;
pub mod resolve;
use libc::{mlock, munlock};
use zeroize::Zeroize;
use crate::types::Mode;

pub struct EnkryptitKey {
    key: LockedKey,
    key_type: KeyType,
}

impl EnkryptitKey {
    /// Function that, given the encryption mode (Encrypting / Decrypting), the original KeyType,
    ///  the context and the path of the file / folder to encrypt / decrypt, resolves the pair (key, keytype)
    ///  and returns an `EnkryptitKey`.
    pub fn resolve(mode: Mode, keytype: &KeyType, context: &mut EnkryptitContext, path: &str) -> Result<Self, EnkryptitError> {
        let (key, key_type) = match keytype {
            KeyType::Password => {
                // This option is only available when encrypting (because `Password`` only exists before keygen, so at encryption), 
                // so we return return an error if the mode is wrong.
                if mode != Mode::Encrypting {
                    return Err(EnkryptitError::InvalidKeyType("Password".to_string(), "Pwd256".to_string()));
                }
                let password = context.resolve_password()?;
                let (key, salt) = key_from_password(password)?;
                (LockedKey::new(key)?, KeyType::Pwd256(salt))
            },

            KeyType::FromFile => {
               let key = resolve_key_from_file(mode, path)?;
               (LockedKey::new(key)?, KeyType::FromFile)
            },

            KeyType::FromOS => {
                let key = resolve_key_from_os(mode, path)?;
                (LockedKey::new(key)?, KeyType::FromOS)
            }
            
            KeyType::Pwd256(salt) => {
                // This option is only available when decrypting (because `Pwd256` only exists in metadata, and contains the salt), 
                // so we return KeyType::None. (and return an error if the mode is wrong)
                if mode != Mode::Decrypting {
                    return Err(EnkryptitError::InvalidKeyType("Pwd256".to_string(), "Password".to_string()));
                }
                
                let password = context.resolve_password()?;
                let key = derive_key(password, *salt)?;
                (LockedKey::new(key)?, KeyType::None)
            },

            KeyType::None => {return Err(EnkryptitError::InvalidKeyType("None".to_string(), "Password, FromFile or FromOS".to_string()));}
        };

        Ok(Self { key, key_type })
    }

    pub fn key_as_ref(&self) -> &[u8; 32] {
        self.key.as_ref()
    }

    pub fn key_type(self) -> KeyType {
        self.key_type
    }

    pub fn key_type_as_ref(&self) -> &KeyType {
        &self.key_type
    }
}

struct LockedKey([u8; 32]);

impl LockedKey {
    /// Creates a new `LockedKey` from an existing key.
    fn new(key: [u8; 32]) -> Result<Self, EnkryptitError> {
        let locked = Self(key);

        unsafe {
            if mlock(locked.0.as_ptr() as *const _, locked.0.len()) != 0 {
                return Err(EnkryptitError::MemoryLockError);
            }
        }

        Ok(locked)
    }

    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Drop for LockedKey {
    fn drop(&mut self) {
        unsafe {
            munlock(self.0.as_ptr() as *const _, self.0.len());
        }

        self.0.zeroize();
    }
}
