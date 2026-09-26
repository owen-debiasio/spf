//! Shared functions and variables that assist with returning filesystem
//! properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use flate2::read::GzDecoder;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufReader, Error, copy},
    path::{Path, PathBuf},
    str::from_utf8,
};
use tar::{Archive, Builder};
use xz2::{read::XzDecoder, write::XzEncoder};

use crate::sys::error;

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
    pub fn extension(path: &str) -> Result<String, Error> {
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
    pub fn name(path: &str) -> Result<String, Error> {
        let file_name = PathBuf::from(path)
            .file_name()
            .expect("Failed to retrieve file name")
            .display()
            .to_string();

        Ok(file_name)
    }
}

/// Determines what type of archive to create/extract
pub enum ArchiveType {
    Tar,
    Xz,
    Gz,
    Ar,
}

/// Creates an archive using `tar`.
///
/// Creates a blank archive (`output`), then copies paths that are located in
/// `path`.
///
/// Finish and close the archive once done.
///
/// Inputs:
///     - `output` ([`str`]) is the name of the output archive
///     - `path` ([`str`]) is the path you want to archive
///     - `archive_type` ([`ArchiveType`]) is the type of tar type
///
/// Example: Making tar.xz
/// ```
/// let output = "archive.tar.xz";
/// let path = "archive";
/// let tar_type = ArchiveType::Xz;
///
/// create_tar_archive(output, path, tar_type)
/// ```
pub fn create_archive(
    output_file: &str,
    path_to_archive: &str,
    archive_type: ArchiveType,
) -> Result<(), Error> {
    let archive_file = File::create(output_file)?;

    match archive_type {
        ArchiveType::Tar => {
            let mut archive = Builder::new(archive_file);

            if Path::new(path_to_archive).is_dir() {
                archive.append_dir_all(FileProperty::name(path_to_archive)?, path_to_archive)?;
            } else {
                archive.append_path(path_to_archive)?
            }

            archive.finish()?;
        }
        ArchiveType::Xz => {
            let mut archive = Builder::new(XzEncoder::new(&archive_file, 6));

            if Path::new(path_to_archive).is_dir() {
                archive.append_dir_all(FileProperty::name(path_to_archive)?, path_to_archive)?;
            } else {
                archive.append_path(path_to_archive)?
            }

            archive.finish()?;
        }
        ArchiveType::Gz => todo!(),
        ArchiveType::Ar => todo!(),
    }

    Ok(())
}

/// Extracts a `tar` archive.
///
/// Inputs:
///     - `path` ([`str`]) is the path you want to archive
///     - `dest` ([`str`]) is the output of the archive
///     - `archive_type` ([`ArchiveType`]) is the type of `tar` archive
///
/// Example: Extracting tar.gz:
/// ```
/// let archive_path = "example.tar.gz";
/// let output = "dir/file.tar.gz";
/// let type = ArchiveType::Gz;
///
/// extract_archive(archive_path, output, type)?;
/// ```
pub fn extract_archive(path: &str, dest: &str, archive_type: ArchiveType) -> Result<(), Error> {
    let archive_path = File::open(path)?;

    match archive_type {
        ArchiveType::Tar => Archive::new(archive_path).unpack(dest)?,
        ArchiveType::Xz => Archive::new(XzDecoder::new(archive_path)).unpack(dest)?,
        ArchiveType::Gz => {
            Archive::new(GzDecoder::new(BufReader::new(archive_path))).unpack(dest)?
        }
        ArchiveType::Ar => {
            fs::create_dir_all(dest)?;

            let mut archive = ar::Archive::new(archive_path);

            while let Some(entry_result) = archive.next_entry() {
                let mut entry = entry_result?;

                let mut file = File::create(
                    from_utf8(entry.header().identifier())
                        .unwrap_or_else(|err| error(&format!("Failed to get utf8 header: {err}"))),
                )?;

                copy(&mut entry, &mut file)?;
            }
        }
    }

    Ok(())
}
