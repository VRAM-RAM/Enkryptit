mod report;
mod inspect_encrypted_archive;
mod inspect_encrypted_file;
mod inspect_plain_file;
mod inspect_plain_folder;

use std::path::Path;
use crate::treatment::object_treatment::read_file;
use crate::errors::EnkryptitError;
use crate::frontend::Output;
use crate::treatment::inspect::inspect_encrypted_archive::inspect_encrypted_archive;
use crate::treatment::inspect::inspect_encrypted_file::inspect_encrypted_file;
use crate::treatment::inspect::inspect_plain_file::inspect_plain_file;
use crate::treatment::inspect::inspect_plain_folder::inspect_plain_folder;
use crate::treatment::object_treatment::ParsedFile;


pub use report::InspectionReport;

pub fn inspect_object(path: &str) -> Result<Output, EnkryptitError> {
    if Path::new(path).is_dir() {
        return Ok(Output::InspectionReport(inspect_plain_folder(path)?));
    }

    match read_file(path) {
        Ok(ParsedFile::Enkryptit { meta, version, is_folder_archive, .. }) => {
            if is_folder_archive {
                return Ok(Output::InspectionReport(inspect_encrypted_archive(path, &meta, version)?));
            } else {
                return Ok(Output::InspectionReport(inspect_encrypted_file(path, &meta, version)?))
            }
        }

        Ok(ParsedFile::Plain) => {
            return Ok(Output::InspectionReport(inspect_plain_file(path)?))
        }

        Err(e) => Ok(Output::Error { error: e })
    }
}

