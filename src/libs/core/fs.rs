//! Shared functions and variables that assist with returning filesystem
//! properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use flate2::read::GzDecoder;
use std::{
    fs::{self, File},
    io::{BufReader, Error, ErrorKind, Write, copy},
    path::Path,
};
use tar::{Archive, Builder};
use xz2::{read::XzDecoder, write::XzEncoder};

/// Determines what type of archive to create/extract
#[derive(Debug, PartialEq)]
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
/// NOTE: Only supports standard `tar` or `tar.xz`. Other variations are
/// disregarded.
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

    let writer: Box<dyn Write> = if archive_type == ArchiveType::Tar {
        Box::new(archive_file)
    } else if archive_type == ArchiveType::Xz {
        Box::new(XzEncoder::new(archive_file, 6))
    } else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid archive type: {archive_type:?}"),
        ));
    };

    let mut archive = Builder::new(writer);

    if Path::new(path_to_archive).is_dir() {
        if let Some(path) = Path::new(path_to_archive).file_name() {
            archive.append_dir_all(path, path_to_archive)?;
        } else {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Failed to add to archive: {path_to_archive}"),
            ));
        }
    } else {
        archive.append_path(path_to_archive)?
    }

    archive.finish()?;

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

                let mut file =
                    File::create(String::from_utf8_lossy(entry.header().identifier()).as_ref())?;

                copy(&mut entry, &mut file)?;
            }
        }
    }

    Ok(())
}
