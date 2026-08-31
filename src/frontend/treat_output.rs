use crate::frontend::cli::Output;

/// Helper for treating an `Output`. Given an `Output`, it prints a text in the terminal
pub fn treat_output(output: Output) {
    match output {
        Output::Success { message } => println!("Success : {}", message),
        Output::Error { error } => eprintln!("[ERROR] {}", error),
        Output::CorruptedFile => eprintln!("[ERROR] File is corrupted, or doesn't exist."),
    }
}
