//! Shared functions and variables that assist with returning filesystem
//! properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use flate2::read::GzDecoder;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
use tar::{Archive, Builder};
use xz::read::XzDecoder;

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
/// Creates a blank archive (`output`), then copies paths that are located in
/// `path`.
///
/// Finish and close the archive once done.
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
/// `archive_type` ([`str`]) determines which archive format to use.
///
/// Chooses one of the following:
///     - `gz`
///     - `xz`
///     - Leave empty for regular tar format
///
/// Using tar.gz:
/// ```
/// let archive = "archive.tar.gz";
/// let dest = ".";
/// let archive_type = "gz";
///
/// extract_archive(archive, dest, archive_type);
/// ```
///
/// Using tar.xz:
/// ```
/// let archive = "archive.tar.xz";
/// let dest = ".";
/// let archive_type = "xz";
///
/// extract_archive(archive, dest, archive_type);
/// ```
///
/// Other
/// ```
/// let archive = "archive.tar";
/// let dest = ".";
/// let archive_type = "";
///
/// extract_archive(archive, dest, archive_type);
/// ```
pub fn extract_tar_archive(
    path: &str,
    dest: &str,
    archive_type: &str,
) -> Result<(), std::io::Error> {
    let archive_path = File::open(path)?;

    if !matches!(archive_type, "gz" | "xz" | "") {
        panic!("Invalid coded archive type: {archive_type}")
    }

    if archive_type == "gz" {
        Archive::new(GzDecoder::new(archive_path)).unpack(dest)?;
    } else if archive_type == "xz" {
        Archive::new(XzDecoder::new(archive_path)).unpack(dest)?;
    } else {
        Archive::new(archive_path).unpack(dest)?;
    };

    Ok(())
}

/// Creates an archive using `tar`.
///
/// You just need to input the path of where it outputs to (`path` ([`str`])).
/// Extracts it using `archive_exec` ([`str`]) to the current working directory.
///
/// ```
/// let archive_exec = "tar";
/// let path_of_archive = "archive.ar";
/// extract_archive(archive_exec, path_of_archive);
///
/// // Extracted directory `archive` should be located in the current working
/// // directory
/// ```
pub fn extract_ar_archive(path: &str, dest: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(dest)?;

    let mut archive = ar::Archive::new(File::open(path)?);

    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result?;

        let mut file = File::create(
            str::from_utf8(entry.header().identifier()).expect("Failed to get header"),
        )
        .unwrap();

        io::copy(&mut entry, &mut file)?;
    }

    Ok(())
}
