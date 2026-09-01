use crate::errors::EnkryptitError;
use crate::frontend::cli::show_params;
use crate::frontend::tui::input::TuiInput;
use crate::log_error;
use crate::parameters::params::{EnkryptitParams, load_params, save_params};
use crate::success;
use crate::types::CompressionType::{self, Lz4, NoComp, Xz};
use crate::types::KeyParams::{File, Os, PassWord};
use crate::types::ParallelismType;
use colored::*;

/// Parameters UI

/// Launch the params UI
pub fn launch_params(input: &impl TuiInput) -> Result<(), EnkryptitError> {
    println!("\n{}", "Parameters Panel".cyan().bold());

    loop {
        let choices = vec![
            "Change compression type",
            "Change key type",
            "Change parallelism type",
            "Show current parameters",
            "Back to main menu",
        ];

        match input.select("What do you want to configure?", &choices) {
            Ok(choice) if choice == "Change compression type" => change_compression(input)?,
            Ok(choice) if choice == "Change key type" => change_key_type(input)?,
            Ok(choice) if choice == "Change parallelism type" => change_parallelism(input)?,
            Ok(choice) if choice == "Show current parameters" => show_current_params()?,
            Ok(choice) if choice == "Back to main menu" => break,
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
fn change_compression(input: &impl TuiInput) -> Result<(), EnkryptitError> {
    let old_params = load_params()?;

    let choices = vec![
        "Auto (automatically choosed by Enkryptit!)",
        "Zstd (balanced)",
        "Lz4 (fastest)",
        "Xz (best compression)",
        "No compression",
    ];

    let choice = input.select("Select compression type:", &choices)?;

    let compression = match choice.as_str() {
        "Auto (automatically choosed by Enkryptit!)" => CompressionType::Auto,
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
fn change_key_type(input: &impl TuiInput) -> Result<(), EnkryptitError> {
    let old_params = load_params()?;

    let choices = vec!["Password (Argon2id)", "OS Keyring", "Key File"];

    let choice = input.select("Select key type:", &choices)?;

    let kt = match choice.as_str() {
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
fn change_parallelism(input: &impl TuiInput) -> Result<(), EnkryptitError> {
    let old_params = load_params()?;

    let choices = vec!["Single", "MultiThread"];

    let choice = input.select("Select parallelism type:", &choices)?;

    let pt = match choice.as_str() {
        "Single" => ParallelismType::Single,
        "MultiThread" => ParallelismType::MultiThread(choose_threads(input)?),
        _ => ParallelismType::Single,
    };

    let params = EnkryptitParams::new(old_params.key_params, old_params.compression, pt.clone());
    save_params(&params)?;

    success!(format!("Key type changed to {:?}", pt));
    Ok(())
}

/// Private helper for choosing number of threads
fn choose_threads(input: &impl TuiInput) -> Result<u8, EnkryptitError> {
    let choosed =
        input.custom_counter("Enter the number of threads you want (recommend : 4-8) :")?;

    Ok(choosed)
}

/// Private helper for printing current parameters
fn show_current_params() -> Result<(), EnkryptitError> {
    let params = load_params()?;
    show_params(&params.key_params, &params.compression, &params.parallelism);
    Ok(())
}
