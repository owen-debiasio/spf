//! Shared functions and variables that assist with returning system properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    metadata::{Meta, PACKAGE_INSTALL_PATH},
    sys::{error, is_root},
};
use std::{
    collections::HashSet,
    fs::{self, remove_dir_all, remove_file},
    io,
    path::Path,
    process::exit,
};

/// Starts the process of removing `.spf` packages.
///
/// The list of packages to be removed is stored in `packages_to_list` as [`Vec<String>`].
///
/// Cycles through the packages in `packages_to_list` and removes the metadata file, and
/// the installed files/directories found within its package metadata (found in [`PACKAGE_INSTALL_PATH`]).
///
/// ```
/// let packages_to_remove = vec![
///     String::from("package_a"),
///     String::from("package_b"),
///     String::from("package_c")
/// ];
///
/// remove_spf_package(packages_to_remove)
/// ```
pub fn remove_spf_package(mut packages_to_remove: Vec<String>) {
    if !is_root() {
        error("To execute this action, please run spf as root.")
    }

    if packages_to_remove.is_empty() {
        error("Provide the package(s) you want to remove.")
    }

    // Remove duplicate package entries
    let package_set: HashSet<_> = packages_to_remove.drain(..).collect();
    packages_to_remove.extend(package_set);

    // Check if packages are installed
    for package in &packages_to_remove {
        let package_meta_path = format!("{PACKAGE_INSTALL_PATH}{package}");

        if !Path::new(&package_meta_path).exists() {
            error(&format!("Package not installed: {package}"))
        }
    }

    list_packages(&packages_to_remove);

    // Everything below here is to ask user for confirmation
    // to remove their selected packages
    println!("\nProceed?\n(Y/N)");

    let mut proceed_to_remove = String::new();
    io::stdin()
        .read_line(&mut proceed_to_remove)
        .expect("Failed to readline");

    if proceed_to_remove.trim().to_lowercase() != "y" {
        println!("Aborted");

        exit(0)
    }

    // Cycle through the packages, and remove them
    for package in packages_to_remove {
        // The marker of where the package is installed. Also removed.
        let package_meta_path = format!("{PACKAGE_INSTALL_PATH}{package}");

        // Retrieves the package version
        let package_version = Meta::from(&package_meta_path).load_value("VERSION");

        // The pretty-print of the package to be removed. Shows the package name
        // and the version.
        let package_formatted = format!("{package}-{package_version}");

        // Finally remove the package
        remove_package(&package_formatted, &package_meta_path);
    }

    println!("\nSuccessfully removed package(s)!");

    exit(0);
}

/// Removes an installed spf package (entries and metadata file).
///
/// Requires:
///     - `package_formatted` ([`String`]): To list that the package has been removed
///     - `package_meta_path`([`String`]): for retrieving paths to delete and to have itself
///     deleted.
///
/// ```
/// // The formatted package name includes the version at the end, such as "-v0.2.0".
/// let package_formatted = "package-v0.2.0";
/// let package_meta_path = "/usr/share/spf/packages/package";
///
/// remove_package(package_formatted, package_meta_path)
/// ```
fn remove_package(package_formatted: &str, package_meta_path: &str) {
    println!("\nRemoving: {package_formatted}");

    // Cycle through the lines of the metadata file
    for entry in fs::read_to_string(package_meta_path)
        .unwrap_or_else(|_| panic!("Failed to retrieve contents of \"{package_meta_path}\""))
        .lines()
    {
        // If the entry isn't an obvious path, skip to next line.
        if !entry.starts_with('/') {
            continue;
        }

        println!("    Removing \"{entry}\"...");

        // Delete the paths.
        //
        // Whether the path is a file or directory is detected through
        // `.is_file()`
        if Path::new(entry).is_file() {
            remove_file(entry).unwrap_or_else(|_| panic!("Failed to remove file \"{entry}\""));
        } else {
            remove_dir_all(entry).unwrap_or_else(|_| panic!("Failed to remove \"{entry}\""));
        }

        // Recreate the status of the path to properly detect if the file
        // is deleted.
        let path_to_remove_check = Path::new(entry);

        // Check if the path has been removed
        if path_to_remove_check.exists() {
            error(&format!(
                "Failed to remove \"{}\": Path still remains",
                path_to_remove_check.display()
            ))
        }
    }

    println!("    Removing package entry...");

    // Removes the metadata file
    remove_file(package_meta_path)
        .unwrap_or_else(|_| panic!("Failed to remove \"{package_meta_path}\""));

    if Path::new(&package_meta_path).exists() {
        error(&format!(
            "Failed to remove \"{package_meta_path}\": File still remains"
        ))
    }

    println!("Successfully removed {package_formatted}!");
}

/// Lists the packages that are going to be removed (`packages`).
///
/// `package_meta_path` is used to retrieve the version of those
/// packages.
///
/// ```
/// // This is if 5 or less packages are being listed
///
/// let packages = vec![
///     String::from("package1"),
///     String::from("package2"),
///     String::from("package3")
/// ];
///
/// list_packages(packages);
///
/// // Output:
/// // package1-v0.0.1 package2-v0.0.2 package3-v0.0.3 ...
/// ```
///
/// This is how the packages are listed if there are 5 or more
/// packages.
///
/// ```
/// let packages = vec![
///     String::from("package1"),
///     String::from("package2"),
///     String::from("package3"),
///     String::from("package4"),
///     String::from("package5"),
///     String::from("package6"),
/// ];
///
/// list_packages(packages);
///
/// // Output:
///
/// // package1-v0.0.1
/// // package2-v0.0.2
/// // package3-v0.0.3
/// // ...
/// ```
fn list_packages(packages_to_list: &[String]) {
    println!("You are about to remove the following package(s):\n");

    let mut package_list: Vec<String> = Vec::new();

    // Goes through and collect the package names and versions from their metadata
    // file located in `PACKAGE_INSTALL_PATH`. Then it writes the formatted name:
    // `package-version` to the buffer.
    for package in packages_to_list {
        let package_meta_path = format!("{PACKAGE_INSTALL_PATH}{package}");
        let package_version = Meta::from(&package_meta_path).load_value("VERSION");

        package_list.push(format!("{package}-{package_version}"));
    }

    // If there are more than 6 packages, separate them with `\n`. Otherwise,
    // keep them aligned next to each other with ` `.
    let package_list_buffer: String = if packages_to_list.len() > 5 {
        package_list.join("\n")
    } else {
        package_list.join(" ")
    };

    // Display the collected packages and their versions
    println!("{package_list_buffer}");
}
