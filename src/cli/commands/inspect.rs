//! Functions related to and inspecting a .spf or installed spf
//! package.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{read_to_string, remove_dir_all},
    io::{Error, ErrorKind},
    path::Path,
};

use crate::libs::spf::{metadata::PACKAGE_INSTALL_PATH, package::Package};

/// You can inspect a .spf package you downloaded, or a package
/// you have already installed.
///
/// If the package to inspect (`package` (as [`String`])) is a .spf package,
/// send it to [`inspect_spf_package`]. Otherwise, send it to [`inspect_installed_package`].
pub fn inspect(package: &str) -> Result<(), Error> {
    if package.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Provide a .spf package or a package that is already installed",
        ));
    }

    if Path::new(&package)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("spf"))
    {
        inspect_spf_package(package)?;
    } else {
        inspect_installed_package(package)?;
    }

    Ok(())
}

/// Inspects the metadata of a .spf package (`package_path` as [`String`]).
///
/// It extracts the package, reads the `META` file, then prints the contents to the output.
///
/// ```
/// let package_to_search = "package.spf";
/// inspect_spf_package(package_to_search);
///
/// // The output is the contents of the `META` file
/// ```
fn inspect_spf_package(package_path: &str) -> Result<(), Error> {
    let package_path_as_path = Path::new(package_path);

    if !package_path_as_path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(".spf package not found: {package_path}"),
        ));
    }

    println!("Loading package \"{package_path}\"...");

    Package::from(package_path.to_string())?.extract(".")?;

    let metadata_path = package_path_as_path
        .file_name()
        .unwrap_or_default()
        .display()
        .to_string()
        .replace(".spf", "/META");

    if !Path::new(&metadata_path).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Extracted metadata file '{metadata_path}' not found"),
        ));
    }

    // Retrieve the contents of the metadata file (`metadata_path`)
    let meta_contents = read_to_string(&metadata_path)?;

    // Shows the inspected contents
    println!(
        "Metadata contents of .spf package \"{package_path}\":\n\
        --------\n\
        \n{meta_contents}\n\
        \n--------"
    );

    let extracted_contents = metadata_path.trim_end_matches("/META");

    // Clean up by removing extracted directory (`extracted_contents`)
    remove_dir_all(extracted_contents)?;

    Ok(())
}

/// Inspects the metadata of an installed spf package (`package` as [`String`]).
///
/// It reads the contents of the file to the string, then displays it to the output.
///
/// ```
/// let package_to_search = "installed_package";
/// inspect_installed_package(package_to_search);
///
/// // The output is the contents of the `META` file
/// ```
fn inspect_installed_package(package: &str) -> Result<(), Error> {
    // The package metadata file to look/inspect
    let package_meta_path = &format!("{PACKAGE_INSTALL_PATH}{package}");

    // Check if the file exists
    if !Path::new(package_meta_path).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Metadata file not found: {package_meta_path}"),
        ));
    }

    // Retrieve the contents of the metadata file (`package_meta_path`)
    let meta_contents = read_to_string(package_meta_path)?;

    // Display the contents.
    println!(
        "Metadata contents of installed package \"{package}\":\n\
        --------\n\
        \n{meta_contents}\n\
        --------"
    );

    Ok(())
}
