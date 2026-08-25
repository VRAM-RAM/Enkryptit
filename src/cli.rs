use crate::errors::EnkryptitError;

/// Enum for Outputs, also used in Modern Ui
pub enum Output {
    Help,
    Error { error: EnkryptitError },
    Success { message: String },
    CorruptedFile,
    SomethingWentWrong,
    ParamsChanged,
}

#[macro_export]
/// Macro that prints the help
macro_rules! print_help {
    () => {
      println!("\n Available commands : \n\n eck path/to/file   | To encrypt or decrypt a file \n params | To open parameters menu")
    };
}

#[macro_export]
/// Macro that prints the help for parameters
macro_rules! print_params_help {
    () => {
      println!("\n Available commands : \n\n comp | To change compression type \n kt | To change key type \n switch | To switch Ui")
    };
}

#[macro_export]
/// Macro that prints a success
macro_rules! success {
    ($succes: expr) => {
        println!("Success : {}", $succes)
    };
}

#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {
        eprintln!("[ERROR] {}", $msg)
    };
}

#[macro_export]
macro_rules! exit {
    () => {
        println!("\n \n Exiting... \n \n")
    };
}

#[macro_export]
macro_rules! enter_password {
    () => {
        println!("\n Please enter a password for encrypting your file : \n")
    };
}

#[macro_export]
macro_rules! params_changed {
    () => {
        println!("\n Params were changed ! \n")
    };
}

#[macro_export]
macro_rules! show_params {
    ($kt: expr, $c: expr) => {
        println!(
            "\n Actual parameters : \n Key Type : {:?} \n Compression : {:?} \n",
            $kt, $c
        )
    };
}
