// use std::path::Path;

// pub fn path_exists(path: &str) -> bool {
//     Path::new(path).exists()
// }

use std::process::Command;

use crate::error;

/// Creates an archive of a directory.
/// Why am I using `std::process::command` instead of a crate? F*ck you, that's why
/// I'm using `tar` because it's basically on every distro and supported well on Linux.
pub fn create_archive_of_dir(output: &str, directory: &str) {
    Command::new("tar")
        .arg("-cf")
        .arg(output)
        .arg(directory)
        .output()
        .unwrap_or_else(|err| {
            error(&format!(
                "Failed to create archive of dir \"{directory}\" to \"{output}\": {err}"
            ))
        });
}
