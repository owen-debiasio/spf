//! Shared functions and variables that assist with returning filesystem
//! properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{path::PathBuf, process::Command};

/// Some utilities to retrieve one of the following properties from a path:
///     - File extension (using [`FileProperty::extension`])
///     - File name (using [`FileProperty::name`])
///
/// All return to [`String`].
///
/// Examples:
///
/// File extension (using [`FileProperty::extension`]):
///
/// ```
/// let file_path = "file.extension"
/// let file_ext = FileProperty::extension(file_path)?;
///
/// // Output should be `extension`
/// println!("{file_ext}");
/// ```
///
/// File name (using [`FileProperty::name`]):
///
/// ```
/// let file_path = "file.extension"
/// let file_name = FileProperty::name(file_path)?;
///
/// // Output should be `file`
/// println!("{file_name}");
/// ```
pub struct FileProperty;

impl FileProperty {
    /// Get file extension
    ///
    /// Retrieves it from `path` (as [`str`]), then returns it as [`String`]
    ///
    /// ```
    /// let file_path = "file.extension"
    /// let file_ext = FileProperty::extension(file_path)?;
    ///
    /// // Output should be `extension`
    /// println!("{file_ext}");
    /// ```
    pub fn extension(path: &str) -> Result<String, std::io::Error> {
        Ok(PathBuf::from(path)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string())
    }

    /// Get file name.
    ///
    /// Retrieves it from `path` (as [`str`]), then returns it as [`String`]
    ///
    /// ```
    /// let file_path = "file.extension"
    /// let file_name = FileProperty::name(file_path)?;
    ///
    /// // Output should be `file`
    /// println!("{file_name}");
    /// ```
    pub fn name(path: &str) -> Result<String, std::io::Error> {
        Ok(PathBuf::from(path)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .to_string())
    }
}

/// Creates an archive of a directory.
/// Why am I using [`std::process::Command`] instead of a crate? F*ck you, that's why
/// I'm using `tar` because it's basically on every distro and supported well on Linux.
///
/// Inputs:
///     - `parent_directories` ([`str`]) is the directories that contain the directory to archive **(OPTIONAL)**
///     - `output` ([`str`]) is the name of the output archive
///     - `directory` ([`str`]) is the directory you want to archive
///
/// **Requires** `tar` executable (preferably the GNU version)
///
/// ```
/// let parent_directories = "parent/dir/";
/// let archive = "archive.spf";
/// let folder_to_compress = "archive";
///
/// create_archive_of_dir(parent_directories, archive, folder_to_compress)
/// ```
pub fn create_archive_of_dir(
    parent_directories: &str,
    output: &str,
    directory: &str,
) -> Result<(), std::io::Error> {
    // You gotta make this because if `parent_directories` it will prob shit itself smh
    if parent_directories.is_empty() {
        Command::new("tar")
            .arg("-cf")
            .arg(output)
            .arg(directory)
            .output()?;

        Ok(())
    } else {
        Command::new("tar")
            .arg("-C")
            .arg(parent_directories)
            .arg("-cf")
            .arg(output)
            .arg(directory)
            .output()?;

        Ok(())
    }
}

/// Creates an archive of a directory.
///
/// You just need to input the path of where it outputs to (`path` ([`str`])).
/// Extracts to the current working directory.
///
/// **Requires** `tar` executable (preferably the GNU version)
///
/// ```
/// let path_of_archive = "archive.spf";
/// extract_archive(path_of_archive);
///
/// // Extracted directory `archive` should be located in the current working
/// // directory
/// ```
pub fn extract_archive(path: &str) -> Result<(), std::io::Error> {
    Command::new("tar").arg("-xf").arg(path).output()?;

    Ok(())
}
