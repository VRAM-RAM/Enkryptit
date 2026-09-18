use std::fmt::Display;
use serde::{Deserialize, Serialize};
use hex::ToHex;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// KeyType enum. Contains :
/// - Password
/// - Pwd256(salt)
/// - FromFile
/// - FromOS
/// - None
pub enum KeyType {
    Password,
    Pwd256([u8; 16]),
    FromFile,
    FromOS,
    None,
}

impl KeyType {
    /// Description of the key type. Single source of truth used
    /// by both `Display` and the `String` conversion.
    pub fn description(&self) -> String {
        match self {
            Self::FromFile => "from file".to_string(),
            Self::FromOS => "from os keyring".to_string(),
            Self::None => "no keytype used (should not happen if the file is encrypted)".to_string(),
            Self::Password => "password".to_string(),
            Self::Pwd256(salt) => format!("hashed password, with the following salt : {}", salt.encode_hex::<String>()),
        }
    }
}

impl From<KeyType> for String {
    fn from(value: KeyType) -> Self {
        value.description()
    }
}

impl Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}