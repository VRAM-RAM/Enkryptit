use crate::errors::EnkryptitError;
use crate::parameters::params::{EnkryptitParams, load_params, save_params};
use crate::types::CompressionType::{self, Lz4, NoComp, Xz};
use crate::types::KeyParams::{File, Os, PassWord};
use crate::types::ParallelismType;
use colored::*;
use inquire::ui::RenderConfig;
use inquire::{CustomType, Select};
use crate::log_error;
use crate::success;
use crate::show_params;

/// Parameters UI

/// Launch the params UI
pub fn launch_params() -> Result<(), EnkryptitError> {
    println!("\n{}", "Parameters Panel".cyan().bold());

    loop {
        let choices = vec![
            "Change compression type",
            "Change key type",
            "Change parallelism type",
            "Show current parameters",
            "Back to main menu",
        ];

        let choice = Select::new("What do you want to configure?", choices).prompt();

        match choice {
            Ok("Change compression type") => change_compression()?,
            Ok("Change key type") => change_key_type()?,
            Ok("Change parallelism type") => change_parallelism()?,
            Ok("Show current parameters") => show_current_params()?,
            Ok("Back to main menu") => break,
            Err(_) => {
                log_error!("Selection cancelled");
                continue;
            }
            _ => continue,
        }
    }

    success!("Parameters updated successfully!");
    Ok(())
}

/// Private helper for changing compression
fn change_compression() -> Result<(), EnkryptitError> {
    let old_params = load_params()?;

    let choices = vec![
        "Zstd (balanced)",
        "Lz4 (fastest)",
        "Xz (best compression)",
        "No compression",
    ];

    let choice = Select::new("Select compression type:", choices)
        .prompt()
        .map_err(|_| EnkryptitError::CommandNotFound)?;

    let compression = match choice {
        "Zstd (balanced)" => CompressionType::Zstd,
        "Lz4 (fastest)" => Lz4,
        "Xz (best compression)" => Xz,
        "No compression" => NoComp,
        _ => CompressionType::Zstd,
    };

    let params = EnkryptitParams::new(old_params.key_params, compression, old_params.parallelism);
    save_params(&params)?;

    success!(format!("Compression changed to {:?}", compression));
    Ok(())
}

/// Private helper for changing key type
fn change_key_type() -> Result<(), EnkryptitError> {
    let old_params = load_params()?;

    let choices = vec!["Password (Argon2id)", "OS Keyring", "Key File"];

    let choice = Select::new("Select key type:", choices)
        .prompt()
        .map_err(|_| EnkryptitError::CommandNotFound)?;

    let kt = match choice {
        "Password (Argon2id)" => PassWord,
        "OS Keyring" => Os,
        "Key File" => File,
        _ => PassWord,
    };

    let params = EnkryptitParams::new(kt.clone(), old_params.compression, old_params.parallelism);
    save_params(&params)?;

    success!(format!("Key type changed to {:?}", kt));
    Ok(())
}

/// Private helper for changing parallelism type
fn change_parallelism() -> Result<(), EnkryptitError> {
    let old_params = load_params()?;

    let choices = vec!["Single", "MultiThread"];

    let choice = Select::new("Select parallelism type:", choices)
        .prompt()
        .map_err(|_| EnkryptitError::CommandNotFound)?;

    let pt = match choice {
        "Single" => ParallelismType::Single,
        "MultiThread" => ParallelismType::MultiThread(choose_threads()?),
        _ => ParallelismType::Single,
    };

    let params = EnkryptitParams::new(old_params.key_params, old_params.compression, pt.clone());
    save_params(&params)?;

    success!(format!("Key type changed to {:?}", pt));
    Ok(())
}

/// Private helper for choosing number of threads
fn choose_threads() -> Result<u8, EnkryptitError> {
    let threads: CustomType<u8> = CustomType::new("Enter the number of threads you want (recommend : 4-8) :")
        .with_default(4)
        .with_error_message("Please type a valid number, greater than 0.")
        .with_help_message("The number of threads you want to use. Recommanded : 4-8 (must be greater than 0).")
        .with_parser(&|u| match u.parse::<u8>() {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        })
        .with_render_config(RenderConfig::default());

    let choosed = threads.prompt().map_err(|_| EnkryptitError::CommandNotFound)?;

    Ok(choosed)
}

/// Private helper for printing current parameters
fn show_current_params() -> Result<(), EnkryptitError> {
    let params = load_params()?;
    show_params!(params.key_params, params.compression);
    Ok(())
}

