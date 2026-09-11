use fmodeparser::FullPermission;
use hex::ToHex;
use crate::{diagnostic::{report::{Report, argument::ReportArgument}, style::EnkryptitStyle}, errors::EnkryptitError, treatment::inspect::InspectionReport, types::{CompressionType, KeyType, ParallelismType}};

impl InspectionReport {
    pub fn display(self) -> Result<(), EnkryptitError> {
        match self {
            Self::PlaintextFile { name, directory, size, permissions, extension, mime_extension, predicted_compression_type, predicted_parallelism_type } => display_plaintext_file(&name, &directory, size, permissions, &extension, &mime_extension, &predicted_compression_type, &predicted_parallelism_type)?,
            Self::EncryptedFile { name, directory, size, version, compression_type, predicted_parallelism_type, keytype, nonce } => display_encrypted_file(&name, &directory, size, version, &compression_type, &predicted_parallelism_type, &keytype, &nonce)?,
            Self::EncryptedArchive { name, directory, size, version, entries_number, keytype } => display_encrypted_archive(&name, &directory, size, version, entries_number, &keytype)?,
            Self::Folder { name, directory, size, permissions } => display_plaintext_folder(&name, &directory, size, permissions)?,
        }
        Ok(())
    }
}

fn display_plaintext_file(name: &str, directory: &str, size: usize, permissions: Option<u32>, extension: &str, mime_extension: &str, ct: &CompressionType, pt: &ParallelismType) -> Result<(), EnkryptitError> {
    let mut report = Report::new(ReportArgument::title("Inspection Report"))
        .field(ReportArgument::label("Type"), ReportArgument::new("Plaintext File (Not encrypted with Enkryptit!)", EnkryptitStyle::Orange))
        .field(ReportArgument::label("Name"), ReportArgument::value(name))
        .field(ReportArgument::label("Directory"), ReportArgument::value(directory))
        .field(ReportArgument::label("Size"), ReportArgument::value_in_kib(to_kb(size).to_string()));

    if permissions.is_some() {
        let fullperms =  FullPermission::new(permissions.unwrap())?;
        report = report.field(ReportArgument::label("Permissions"), ReportArgument::value(fullperms.to_string()));    
    }

    report.field(ReportArgument::label("Shown extension"), ReportArgument::value(extension))
        .field(ReportArgument::label("Real extension"), ReportArgument::accent(mime_extension))
        .field(ReportArgument::label("Recommanded compression type  (for encryption)"), ReportArgument::accent(ct.to_string()))
        .field(ReportArgument::label("Recommanded parallelism type (for encryption)"), ReportArgument::accent(pt.to_string()))
        .display();

    Ok(())
}

fn display_encrypted_file(name: &str, directory: &str, size: usize, version: u8, ct: &CompressionType, pt: &ParallelismType, keytype: &KeyType, nonce: &[u8; 24]) -> Result<(), EnkryptitError> {
    Report::new(ReportArgument::title("Inspection Report"))
        .field(ReportArgument::label("Type"), ReportArgument::value("File encrypted with Enkryptit!"))
        .field(ReportArgument::label("Version"), ReportArgument::accent(version.to_string()))
        .field(ReportArgument::label("Name"), ReportArgument::value(name))
        .field(ReportArgument::label("Directory"), ReportArgument::value(directory))
        .field(ReportArgument::label("Size"), ReportArgument::value_in_kib(to_kb(size).to_string()))
        .field(ReportArgument::label("Compression type"), ReportArgument::accent(ct.to_string()))
        .field(ReportArgument::label("Recommended parallelism type (for decrypting)"), ReportArgument::accent(pt.to_string()))
        .field(ReportArgument::label("Key Type"), ReportArgument::accent(keytype.to_string()))
        .field(ReportArgument::label("Nonce"), ReportArgument::accent(nonce.encode_hex::<String>()))
        .display();

    Ok(())
}

fn display_encrypted_archive(name: &str, directory: &str, size: usize, version: u8, entries_number: u64, keytype: &KeyType) -> Result<(), EnkryptitError> {
    Report::new(ReportArgument::title("Inspection Report"))
        .field(ReportArgument::label("Type"), ReportArgument::value("Folder (Archive) encrypted with Enkryptit!"))
        .field(ReportArgument::label("Version"), ReportArgument::accent(version.to_string()))
        .field(ReportArgument::label("Name"), ReportArgument::value(name))
        .field(ReportArgument::label("Directory"), ReportArgument::value(directory))
        .field(ReportArgument::label("Size"), ReportArgument::value_in_kib(to_kb(size).to_string()))
        .field(ReportArgument::label("Entries number"), ReportArgument::accent(entries_number.to_string()))
        .field(ReportArgument::label("Key Type"), ReportArgument::accent(keytype.to_string()))
        .display();

    Ok(())
}

fn display_plaintext_folder(name: &str, directory: &str, size: usize, permissions: Option<u32>) -> Result<(), EnkryptitError> {
    let mut report = Report::new(ReportArgument::title("Inspection Report"))
        .field(ReportArgument::label("Type"), ReportArgument::value("Plaintext Folder (Not encrypted with Enkryptit!)"))
        .field(ReportArgument::label("Name"), ReportArgument::value(name))
        .field(ReportArgument::label("Directory"), ReportArgument::value(directory))
        .field(ReportArgument::label("Size"), ReportArgument::value_in_kib(to_kb(size).to_string()));

    if permissions.is_some() {
        let fullperms = FullPermission::new(permissions.unwrap())?;
        report = report.field(ReportArgument::label("Permissions"), ReportArgument::value(fullperms.to_string()));    
    }

    report.display();

    Ok(())
}

fn to_kb(size: usize) -> f32 {
    size as f32 /  1024.0
}