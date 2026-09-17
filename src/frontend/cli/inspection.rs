use crate::treatment::inspect::inspect_object;


/// Helper for inspecting one or many paths.
/// \
/// It just iterates over all the paths and inspects each one calling `inspect_object`.
pub fn inspect_one_or_more_objects(
    paths: &Vec<String>,
) -> () {
    // We iterate over the paths
    for path in paths {
        inspect_object(path).display();
    }
}

