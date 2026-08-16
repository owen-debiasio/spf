// use std::path::Path;

// pub fn path_exists(path: &str) -> bool {
//     Path::new(path).exists()
// }

use std::{path::PathBuf, process::Command};

use crate::sys::Error;

static SOURCE_FILE: &str = "src/fs.rs";

/// Some utilities to retrieve one of the following properties from a path:
///     - File name
///     - File extension
///
/// All return to `String`
pub struct FileProperty;

impl FileProperty {
    /// Get file extension
    pub fn extension(path: &str) -> String {
        PathBuf::from(path)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_else(|| {
                Error::fatal(
                    SOURCE_FILE,
                    "FileProperty::extension()",
                    28,
                    &format!("Failed to retrieve file extension from: {path}"),
                )
            })
            .to_string()
    }

    /// Get file name
    pub fn name(path: &str) -> String {
        PathBuf::from(path)
            .file_name()
            .unwrap_or_else(|| {
                Error::fatal(
                    SOURCE_FILE,
                    "FileProperty::name()",
                    43,
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
///
/// Inputs:
///     - `parent_directories` is the directories that contain the directory to archive **(OPTIONAL)**
///     - `output` is the name of the output archive
///     - `directory` is the directory you want to archive
///
/// **Requires** `tar` executable (preferably the GNU version)
pub fn create_archive_of_dir(parent_directories: &str, output: &str, directory: &str) {
    // You gotta make this because if `parent_directories` it will prob shit itself smh
    if parent_directories.is_empty() {
        Command::new("tar")
            .arg("-cf")
            .arg(output)
            .arg(directory)
            .output()
            .unwrap_or_else(|err| {
                Error::fatal(
                    SOURCE_FILE,
                    "create_archive_of_dir()",
                    75,
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
                Error::fatal(
                    SOURCE_FILE,
                    "create_archive_of_dir()",
                    93,
                    &format!(
                        "Failed to create archive of dir \"{directory}\" to \"{output}\": {err}"
                    ),
                )
            });
    }
}

/// Creates an archive of a directory.
///
/// You just need to input the path of where it outputs to (`path`).
/// Extracts to the current working directory.
///
/// **Requires** `tar` executable (preferably the GNU version)
pub fn extract_archive(path: &str) {
    Command::new("tar")
        .arg("-xf")
        .arg(path)
        .output()
        .unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "extract_archive()",
                117,
                &format!("Failed to extract archive \"{path}\": {err}"),
            )
        });
}
