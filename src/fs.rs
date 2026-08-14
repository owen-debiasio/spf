// use std::path::Path;

// pub fn path_exists(path: &str) -> bool {
//     Path::new(path).exists()
// }

use std::{path::PathBuf, process::Command};

use crate::error;

static SOURCE_FILE: &str = "src/fs.rs";

pub struct FileProperty;

impl FileProperty {
    pub fn extension(path: &str) -> String {
        PathBuf::from(path)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_else(|| {
                error(
                    SOURCE_FILE,
                    "FileProperty::extension()",
                    22,
                    &format!("Failed to retrieve file extension from: {path}"),
                )
            })
            .to_string()
    }

    pub fn name(path: &str) -> String {
        PathBuf::from(path)
            .file_name()
            .unwrap_or_else(|| {
                error(
                    SOURCE_FILE,
                    "FileProperty::name()",
                    36,
                    &format!("Failed to get name of file: {path}"),
                )
            })
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// Creates an archive of a directory.
/// Why am I using `std::process::command` instead of a crate? F*ck you, that's why
/// I'm using `tar` because it's basically on every distro and supported well on Linux.
pub fn create_archive_of_dir(parent_directories: &str, output: &str, directory: &str) {
    // You gotta make this because if `parent_directories` it will prob shit itself smh
    if parent_directories.is_empty() {
        Command::new("tar")
            .arg("-cf")
            .arg(output)
            .arg(directory)
            .output()
            .unwrap_or_else(|err| {
                error(
                    SOURCE_FILE,
                    "create_archive_of_dir()",
                    54,
                    &format!(
                        "Failed to create archive of dir \"{directory}\" to \"{output}\": {err}"
                    ),
                )
            });
    } else {
        Command::new("tar")
            .arg("-C")
            .arg(parent_directories)
            .arg("-cf")
            .arg(output)
            .arg(directory)
            .output()
            .unwrap_or_else(|err| {
                error(
                    SOURCE_FILE,
                    "create_archive_of_dir()",
                    72,
                    &format!(
                        "Failed to create archive of dir \"{directory}\" to \"{output}\": {err}"
                    ),
                )
            });
    }
}

/// Creates an archive of a directory.
pub fn extract_archive(path: &str) {
    Command::new("tar")
        .arg("-xf")
        .arg(path)
        .output()
        .unwrap_or_else(|err| {
            error(
                SOURCE_FILE,
                "extract_archive()",
                98,
                &format!("Failed to extract archive \"{path}\": {err}"),
            )
        });
}
