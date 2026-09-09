use fmodeparser::FullPermission;
use hex::ToHex;
use crate::{errors::EnkryptitError, treatment::inspect::InspectionReport, types::{CompressionType, KeyType, ParallelismType}};
use colored::Colorize;

impl InspectionReport {
    pub fn display(self) -> Result<(), EnkryptitError> {
        match self {
            Self::PlaintextFile { name, directory, size, permissions, extension, mime_extension, predicted_compression_type, predicted_parallelism_type } => display_plaintext_file(&name, &directory, size, permissions, &extension, &mime_extension, &predicted_compression_type, &predicted_parallelism_type)?,
            Self::EncryptedFile { name, directory, size, version, compression_type, predicted_parallelism_type, keytype, nonce } => display_encrypted_file(&name, &directory, size, version, &compression_type, &predicted_parallelism_type, &keytype, &nonce),
            Self::EncryptedArchive { name, directory, size, version, entries_number, keytype } => display_encrypted_archive(&name, &directory, size, version, entries_number, &keytype),
            Self::Folder { name, directory, size, permissions } => display_plaintext_folder(&name, &directory, size, permissions)?,
        }
        Ok(())
    }
}

fn display_plaintext_file(name: &str, directory: &str, size: usize, permissions: Option<u32>, extension: &str, mime_extension: &str, ct: &CompressionType, pt: &ParallelismType) -> Result<(), EnkryptitError> {
    println!();
    println!("{}", "Inspection Report".ansi_color(27)); // A blue (see https://github.com/fidian/ansi for ansi color informations)
    println!();
    println!("{}{}", "Type : ".ansi_color(26), "Plaintext File (Not encrypted with Enkryptit!)".ansi_color(202));
    println!("{}{}", "Name : ".ansi_color(26), name.ansi_color(202));
    println!("{}{}", "Directory : ".ansi_color(26), directory.ansi_color(202));
    println!("{}{}", "Size : ".ansi_color(26), size.to_string().ansi_color(202));

    if permissions.is_some() {
        let fullperms =  FullPermission::new(permissions.unwrap())?;
        println!("{}{}", "Permissions : ".ansi_color(26), fullperms.to_string().ansi_color(202));    
    }

    println!("{}{}", "Shown extension : ".ansi_color(26), extension.ansi_color(202));
    println!("{}{}", "Real extension : ".ansi_color(26), mime_extension.ansi_color(202));
    println!("{}{}", "Recommanded compression type (for encrypting) : ".ansi_color(26), ct.to_string().ansi_color(202));
    println!("{}{}", "Recommanded parallelism type (for encrypting) : ".ansi_color(26), pt.to_string().ansi_color(202));

    Ok(())
}

fn display_encrypted_file(name: &str, directory: &str, size: usize, version: u8, ct: &CompressionType, pt: &ParallelismType, keytype: &KeyType, nonce: &[u8; 24]) {
    println!();
    println!("{}", "Inspection Report".ansi_color(27)); // A blue (see https://github.com/fidian/ansi for ansi color informations)
    println!();
    println!("{}{}", "Type : ".ansi_color(26), "File encrypted with Enkryptit!".ansi_color(202));
    println!("{}{}", "Version : ".ansi_color(26), version.to_string().ansi_color(202));
    println!("{}{}", "Name : ".ansi_color(26), name.ansi_color(202));
    println!("{}{}", "Directory : ".ansi_color(26), directory.ansi_color(202));
    println!("{}{}", "Size : ".ansi_color(26), size.to_string().ansi_color(202));
    println!("{}{}", "Compression type : ".ansi_color(26), ct.to_string().ansi_color(202));
    println!("{}{}", "Recommanded parallelism type (for decrypting) : ".ansi_color(26), pt.to_string().ansi_color(202));
    println!("{}{}", "Key Type : ".ansi_color(26), keytype.to_string().ansi_color(202));
    println!("{}{}", "Nonce : ".ansi_color(26), nonce.encode_hex::<String>().to_string().ansi_color(202));
}

fn display_encrypted_archive(name: &str, directory: &str, size: usize, version: u8, entries_number: u64, keytype: &KeyType) {
    println!();
    println!("{}", "Inspection Report".ansi_color(27)); // A blue (see https://github.com/fidian/ansi for ansi color informations)
    println!();
    println!("{}{}", "Type : ".ansi_color(26), "Folder (Archive) encrypted with Enkryptit!".ansi_color(202));
    println!("{}{}", "Version : ".ansi_color(26), version.to_string().ansi_color(202));
    println!("{}{}", "Name : ".ansi_color(26), name.ansi_color(202));
    println!("{}{}", "Directory : ".ansi_color(26), directory.ansi_color(202));
    println!("{}{}", "Size : ".ansi_color(26), size.to_string().ansi_color(202));
    println!("{}{}", "Entries number : ".ansi_color(26), entries_number.to_string().ansi_color(202));
    println!("{}{}", "Key Type : ".ansi_color(26), keytype.to_string().ansi_color(202));
}

fn display_plaintext_folder(name: &str, directory: &str, size: usize, permissions: Option<u32>) -> Result<(), EnkryptitError> {
    println!();
    println!("{}", "Inspection Report".ansi_color(27)); // A blue (see https://github.com/fidian/ansi for ansi color informations)
    println!();
    println!("{}{}", "Type : ".ansi_color(26), "Plaintext File (Not encrypted with Enkryptit!)".ansi_color(202));
    println!("{}{}", "Name : ".ansi_color(26), name.ansi_color(202));
    println!("{}{}", "Directory : ".ansi_color(26), directory.ansi_color(202));
    println!("{}{}", "Size : ".ansi_color(26), size.to_string().ansi_color(202));

    if permissions.is_some() {
        let fullperms =  FullPermission::new(permissions.unwrap())?;
        println!("{}{}", "Permissions : ".ansi_color(26), fullperms.to_string().ansi_color(202));    
    }

    Ok(())
}