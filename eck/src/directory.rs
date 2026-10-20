use std::path::{Path, PathBuf};
use std::path::Component;
use directories::ProjectDirs;

use crate::errors::EnkryptitError;

/// Returns the project directory of **Enkryptit**. If it fails, return [`EnkryptitError::ConfigError`]
pub fn project_dir() -> Result<ProjectDirs, EnkryptitError> {
    ProjectDirs::from("com", "olruix", "Enkryptit").ok_or(EnkryptitError::ConfigError)
}

/// Retrieves the project directory using [`project_dir`], and converts it into a [`PathBuf`]
pub fn project_dir_path() -> Result<PathBuf, EnkryptitError> {
    Ok(project_dir()?.config_dir().to_path_buf())
}

/// Safely joins a destination path and a relative path (`dest.join(relative)`). 
/// \
/// If the relative path is incorrect, it returns [`EnkryptitError::PathIsIncorrect`].
pub fn safe_join(dest: &str, relative: &str) -> Result<PathBuf, EnkryptitError> {
    let rel = Path::new(relative);
    if rel.is_absolute() { return Err(EnkryptitError::PathIsIncorrect(relative.to_string())) }
    for component in rel.components() {
        match component {
            Component::Normal(_) | Component::CurDir => (),
            _ => return Err(EnkryptitError::PathIsIncorrect(relative.to_string()))
        }
    }

    Ok(Path::new(dest).join(rel))
}