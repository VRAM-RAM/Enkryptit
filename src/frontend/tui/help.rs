use colored::*;

/// helper for printing the help in TUI mode
pub fn show_help() {
    println!("\n{}", "Available Commands".cyan().bold());
    println!("   Encrypt/Decrypt  -> Process a file");
    println!("   Parameters       -> Configure settings");
    println!("   Help             -> Show this help");
    println!("   Browse           -> Browse files, folders or both to encrypt / decrypt");
    println!("   Exit             -> Quit application");
}
