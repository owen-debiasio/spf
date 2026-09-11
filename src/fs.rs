//! Shared functions and variables that assist with returning filesystem
//! properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};
use tar::Builder;

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
        let file_extension = PathBuf::from(path)
            .extension()
            .unwrap_or(OsStr::new(&String::new()))
            .display()
            .to_string();

        Ok(file_extension)
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
        let file_name = PathBuf::from(path)
            .file_name()
            .expect("Failed to retrieve file name")
            .display()
            .to_string();

        Ok(file_name)
    }
}

/// Creates an archive of a directory.
///
/// Inputs:
///     - `output` ([`str`]) is the name of the output archive
///     - `path` ([`str`]) is the path you want to archive
///
/// ```
/// let output = "archive.spf";
/// let path = "archive";
///
/// create_tar_archive(output, path)
/// ```
pub fn create_tar_archive(output: &str, path: &str) -> Result<(), std::io::Error> {
    let mut archive = Builder::new(File::create(output)?);

    if Path::new(path).is_dir() {
        archive.append_dir_all(FileProperty::name(path)?, path)?;
    } else {
        archive.append_path(path)?
    }

    archive.finish()?;

    Ok(())
}

/// Creates an archive of a directory.
///
/// You just need to input the path of where it outputs to (`path` ([`str`])).
/// Extracts it using `archive_exec` ([`str`]) to the current working directory.
///
/// ```
/// let archive_exec = "tar";
/// let path_of_archive = "archive.spf";
/// extract_archive(archive_exec, path_of_archive);
///
/// // Extracted directory `archive` should be located in the current working
/// // directory
/// ```
pub fn extract_archive(exec: &str, path: &str) -> Result<(), std::io::Error> {
    Command::new(exec).arg("-xf").arg(path).output()?;

    Ok(())
}
