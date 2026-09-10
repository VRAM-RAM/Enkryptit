use crate::frontend::treat_output::treat_output;
use crate::log_error;
use crate::treatment::inspect::inspect_object;


/// Helper for inspecting one or many paths.
/// \
/// It just iterates over all the paths and inspects each one calling `inspect_object`.
pub fn inspect_one_or_more_objects(
    paths: &Vec<String>,
) -> () {
    // We iterate over the paths
    for path in paths {
        match inspect_object(path) {
            Ok(output) => treat_output(output),
            Err(e) => log_error!(e),
        }
    }
}

