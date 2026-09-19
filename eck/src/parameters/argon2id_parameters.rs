use crate::errors::EnkryptitError;

/// Argon2id parameters are statics, so that it doesn't create any serialization nor metadata problems.
/// (For example, we don't need to add a `argon2params` field in metadata nor force the user to specify the argon2id parameters).
/// This function returns the static argon2id parameters, with the following specifications :
///
/// memory cost : 128 MiB
/// iterations: 3
/// parallelism : 1
pub fn argon2id_parameters() -> Result<argon2::Params, EnkryptitError> {
    Ok(argon2::Params::new(128 * 1024, 3, 1, Some(32))?)
}
