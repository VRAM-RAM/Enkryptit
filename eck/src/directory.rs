use std::path::PathBuf;

use directories::ProjectDirs;

use crate::errors::EnkryptitError;

pub fn project_dir() -> Result<ProjectDirs, EnkryptitError> {
    ProjectDirs::from("com", "olruix", "Enkryptit").ok_or(EnkryptitError::ConfigError)
}

pub fn project_dir_path() -> Result<PathBuf, EnkryptitError> {
    Ok(project_dir()?.config_dir().to_path_buf())
}