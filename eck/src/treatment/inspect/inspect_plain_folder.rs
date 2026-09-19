use std::{fs::{metadata},  path::Path};


use crate::{treatment::inspect::InspectionReport};

pub fn inspect_plain_folder(path: &str) -> InspectionReport {
    let pathstd = Path::new(path);

    let mut size = None;
    let mut permissions = None;

    match metadata(path) {
        Ok(m) => {
            size = Some(m.len());
            permissions = {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    Some(m.permissions().mode())
                }

                #[cfg(not(unix))]
                {
                    None // No permissions on Windows
                } 
            };
        }

        Err(e) => tracing::warn!("{}", e)
    }

    let name = pathstd.file_name().map(|p| p.to_string_lossy().to_string());

    let directory = pathstd.parent().map(|p| p.to_string_lossy().to_string());

    InspectionReport::Folder { name, directory, size, permissions }
}