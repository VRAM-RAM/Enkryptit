use crate::context::EnkryptitContext;
use crate::errors::EnkryptitError;
use crate::frontend::cli::Output;
use crate::frontend::treat_output::treat_output;
use crate::log_error;
use crate::parameters::params::load_params;
use crate::treatment::object_treatment::treat_object;
use crate::types::Interface;

/// Helper for treating a path.
/// First, we load the parameters, before converting the path from `&str` to `&Path`.
/// Then, if the path doesn't exist, we return an error.
/// If it exist, we continue, by getting the `key` and the `keytype`.
/// Finally, we delegate the object treatment to `treat_object()`.
pub fn treat_object_with_path(
    path_str: &str,
    cli_password: Option<String>,
) -> Result<Output, EnkryptitError> {
    let parameters = load_params()?;
    let mut context = EnkryptitContext::new(Interface::Cli, cli_password);
    treat_object(&parameters, path_str, &mut context)
}

/// Function that treat the objects, when the args contain many paths
pub fn treat_objects_with_multiple_paths(
    paths: &Vec<String>,
    cli_password: Option<String>,
) -> Result<(), EnkryptitError> {
    // We load the parameters
    let parameters = load_params()?;
    // Create the global context
    let mut context = EnkryptitContext::new(Interface::Cli, cli_password);
    // And iterate to treat every path
    for path in paths {
        match treat_object(&parameters, path, &mut context) {
            // We don't return. Instead, we log / treat the output.
            // Why ? Because we want to continue treating the other objects, even if one fails treating.
            Ok(output) => treat_output(output),
            Err(e) => log_error!(e),
        }
    }
    Ok(())
}
