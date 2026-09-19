use serde::{Deserialize, Serialize};
use crate::types::key_type::KeyType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Key Parameters, used **before** encryption / decryption, that describes user's preference to *Enkryptit!*'s backend, and stored in 
/// the parameters.
/// 
/// Contains :
/// - PassWord
/// - File
/// - Os
pub enum KeyParams {
    PassWord,
    File,
    Os,
}

impl KeyParams {
    /// Converts a [`KeyParams`] into a [`KeyType`]
    pub fn to_type(&self) -> KeyType {
        match self {
            Self::File => KeyType::FromFile,
            Self::Os => KeyType::FromOS,
            Self::PassWord => KeyType::Password,
        }
    }
}