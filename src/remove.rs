//! Shared functions and variables that assist with returning system properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    metadata::{PACKAGE_INSTALL_PATH, get_meta_value},
    sys::{error, is_root},
};
use std::{
    fs::{self, remove_dir_all, remove_file},
    io,
    path::Path,
    process::exit,
};

pub fn remove_spf_package(packages_to_list: Vec<String>) {
    if !is_root() {
        error("To execute this action, please run spf as root.")
    }

    if packages_to_list.is_empty() {
        error("Provide the package(s) you want to remove.")
    }

    // Restrict the ability to uninstall spf with spf to avoid conflicts
    // } else if packages_to_list.contains(&"spf".to_string()) && get_binary_path() == "/usr/bin/spf" {
    //     error("If you want to remove spf using spf, please use the standalone binary.")
    // }

    // Keeps track of when the ability to list packages
    // is allowed.
    //
    // Once the packages finish listing, disable.
    let mut enable_list_packages = true;

    // Cycle through the packages
    for package in &packages_to_list {
        // The marker of where the package is installed. Also removed.
        let package_meta_path = format!("{PACKAGE_INSTALL_PATH}{package}");

        // Check if packages are installed
        if !Path::new(&package_meta_path).exists() {
            error(&format!("Package not installed: {package}"))
        }

        // Retrieves the package version
        let package_version = get_meta_value(package_meta_path.clone(), "VERSION");

        // The pretty-print of the package to be removed. Shows the package name
        // and the version.
        let package_formatted = format!("{package}-{package_version}");

        // Check if the ability to list packages is enabled, and do such
        // if so.
        if enable_list_packages {
            // Handles listing packages.
            list_packages(packages_to_list.clone(), package_meta_path.clone());

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

            // Disable package listing
            enable_list_packages = false
        }

        // Finally remove the package
        remove_package(package_formatted, package_meta_path)
    }

    println!("\nSuccessfully removed package(s)!");

    exit(0);
}

/// Removes an installed spf package (entries and metadata file).
///
/// Requires:
///     - `package_formatted`: To list that the package has been removed
///     - `package_meta_path`: for retrieving paths to delete and to have itself
///     deleted.
fn remove_package(package_formatted: String, package_meta_path: String) {
    println!("\nRemoving: {package_formatted}");

    // Cycle through the lines of the metadata file
    for entry in fs::read_to_string(package_meta_path.clone())
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
            remove_file(entry).unwrap_or_else(|_| panic!("Failed to remove file \"{entry}\""))
        } else {
            remove_dir_all(entry).unwrap_or_else(|_| panic!("Failed to remove \"{entry}\""))
        }

        // Recreate the status of the path to properly detect if the file
        // is deleted.
        let path_to_remove_check = Path::new(entry);

        // Check if the path has been removed
        if path_to_remove_check.exists() {
            panic!(
                "Failed to remove \"{}\": Path still remains",
                path_to_remove_check.to_str().unwrap()
            )
        }
    }

    println!("    Removing package entry...");

    // Removes the metadata file
    remove_file(&package_meta_path)
        .unwrap_or_else(|_| panic!("Failed to remove \"{package_meta_path}\""));

    if Path::new(&package_meta_path).exists() {
        panic!("Failed to remove \"{package_meta_path}\": File still remains")
    }

    println!("Successfully removed {package_formatted}!");
}

/// Lists the packages that are going to be removed (`packages`).
///
/// `package_meta_path` is used to retrieve the version of those
/// packages.
///
/// It also lists them dynamically (see below).
fn list_packages(packages: Vec<String>, package_meta_path: String) {
    println!("You are about to remove the following package(s):\n");

    // If there are less than 5 or less packages, list packages like this:
    //
    // package1-v0.0.1 package2-v0.0.2 package3-v0.0.3 ...
    //
    // Otherwise list them like this:
    //
    // package1-v0.0.1
    // package2-v0.0.2
    // package3-v0.0.3
    // ...
    if packages.len() > 6 {
        // Cycle through packages
        for package in &packages {
            // Retrieve package version
            let listed_package_version = get_meta_value(package_meta_path.clone(), "VERSION");

            // List package
            print!("{package}-{listed_package_version}")
        }
        println!();
    } else {
        // Cycle through packages
        for package in &packages {
            // Retrieve package version
            let listed_package_version = get_meta_value(package_meta_path.clone(), "VERSION");

            // List package
            println!("{package}-{listed_package_version}")
        }
    }
}
