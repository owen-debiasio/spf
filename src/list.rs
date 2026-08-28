//! Functions related to and providing the ability list any packages installed
//! by spf.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::process::exit;

use glob::glob;

use crate::{
    metadata::{Meta, PACKAGE_INSTALL_PATH},
    sys::error,
};

/// Lists packages that are be installed. You can optionally provide
/// a string of text to match (`optional_string` ([`String`])).
///
/// Example with no optional string:
/// ```
/// list_packages(String::new());
///
/// // Every package installed is listed
/// ```
/// Example with an optional string:
/// ```
/// let optional = String::from("sp");
/// list_packages(optional);
///
/// // Every package installed that contains `spf` is listed
/// ```
pub fn list_packages(optional_string: &str) {
    // where to search for packages
    let path_to_search = &format!("{PACKAGE_INSTALL_PATH}/*");

    // packages that are to be listed.
    //
    // First though, it is temporarily used to verify that
    // packages are installed
    let mut packages_to_list = vec![];

    // Collect paths and add them to the vec
    for dir in glob(path_to_search).unwrap() {
        packages_to_list.push(dir.unwrap().to_str().unwrap().to_string());
    }

    // If there are no paths found, that means no packages are installed.
    // Throw an error.
    if packages_to_list.is_empty() {
        error("No packages are installed.")
    }

    // Clear the vec to be used for its main purpose
    packages_to_list.clear();

    // Go through the package metadata install directory
    for package_path_raw in glob(path_to_search)
        .unwrap_or_else(|_| panic!("Failed to collect directories at \"{PACKAGE_INSTALL_PATH}\""))
    {
        // The path of the package metadata. The name of the metadata file is the name
        // of the package.
        let package_metadata_path = package_path_raw.unwrap().to_str().unwrap().to_string();

        // Name of the package to be listed
        let package_meta = Meta::from(&package_metadata_path);

        let package_name = package_meta.clone().load_value("PROJECT_NAME").clone();

        // Checks if the entered optional string is in the package name.
        // If not, move to next package.
        if !package_name.contains(optional_string) {
            continue;
        }

        // Retrieve the package version and description
        let package_version = package_meta.clone().load_value("VERSION");
        let package_desc = package_meta.load_value("DESCRIPTION").clone();

        // Add the package name, version, and description
        packages_to_list.push(format!(
            "> {package_name}-{package_version}\n    {package_desc}"
        ));
    }

    // Dynamically choose the message to show
    println!(
        "{} packages:\n",
        if optional_string.is_empty() {
            "Installed"
        } else if packages_to_list.is_empty() {
            error(&format!("No packages match or contain: {optional_string}"))
        } else {
            "Matching installed"
        }
    );

    // List the packages collected
    for package in packages_to_list {
        println!("{package}");
    }

    exit(0)
}
