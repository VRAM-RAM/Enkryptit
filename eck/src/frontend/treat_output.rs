use crate::diagnostic::EnkryptitOutput;

/// Helper for treating an `EnkryptitOutput`.
/// \
/// Given an `EnkryptitOutput`, it renders it in the terminal.
pub fn treat_output(output: EnkryptitOutput) {
    output.display();
}