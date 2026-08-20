//! Functions related to and inspecting a .spf or installed spf
//! package.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{self, remove_dir_all},
    path::Path,
    process::exit,
};

use crate::{
    fs::{FileProperty, extract_archive},
    metadata::PACKAGE_INSTALL_PATH,
    sys::Error,
};

static SOURCE_FILE: &str = "src/inspect.rs";

/// You can inspect a .spf package you downloaded, or a package
/// you have already installed.
pub fn inspect(package: String) {
    if package.is_empty() {
        Error::normal("Please provide a .spf package or a package that is already installed.")
    }

    if package.ends_with(".spf") {
        inspect_spf_package(package);
    } else {
        inspect_installed_package(package);
    }
}

/// Inspects the metadata of a .spf package.
///
/// It extracts the package, reads the META file, then prints
/// the contents to the output.
fn inspect_spf_package(package_path: String) {
    if !Path::new(&package_path).exists() {
        Error::normal(&format!(".spf package not found: {package_path}"))
    }

    extract_archive(&package_path);

    let metadata_path = FileProperty::name(&package_path).replace(".spf", "/META");

    if !Path::new(&metadata_path).exists() {
        Error::fatal(
            SOURCE_FILE,
            "inspect_spf_package()",
            49,
            &format!("Metadata file not found: {metadata_path}"),
        )
    }

    // Retrieve the contents of the metadata file (`metadata_path`)
    let meta_contents = fs::read_to_string(&metadata_path).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "inspect_spf_package()",
            59,
            &format!("Failed to retrieve contents of \"{metadata_path}\": {err}"),
        )
    });

    // Shows the inspected contents
    println!(
        "Metadata contents of .spf package \"{package_path}\":\n\
        --------\n\
        \n{meta_contents}\n\
        \n--------"
    );

    let extracted_contents = metadata_path.trim_end_matches("/META");

    // Clean up by removing extracted directory (`extracted_contents`)
    remove_dir_all(extracted_contents).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "inspect_spf_package()",
            79,
            &format!("Failed to remove \"{extracted_contents}\": {err}"),
        )
    });

    exit(0)
}

/// Inspects the metadata of an installed spf package.
///
/// It reads the contents of the file to the string, then
/// displays it to the output.
fn inspect_installed_package(package: String) {
    // The package metadata file to look/inspect
    let package_meta_path = &format!("{PACKAGE_INSTALL_PATH}{package}");

    // Check if the file exists
    if !Path::new(package_meta_path).exists() {
        Error::fatal(
            SOURCE_FILE,
            "inspect_installed_package()",
            100,
            &format!("Metadata file not found: {package_meta_path}"),
        )
    }

    // Retrieve the contents of the metadata file (`package_meta_path`)
    let meta_contents = fs::read_to_string(package_meta_path).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "inspect_installed_package()",
            110,
            &format!("Failed to retrieve contents of \"{package_meta_path}\": {err}"),
        )
    });

    // Display the contents.
    println!(
        "Metadata contents of installed package \"{package}\":\n\
        --------\n\
        \n{meta_contents}\n\
        --------"
    );

    exit(0)
}
