//! Functions related to and installing a .spf package.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env::consts::ARCH,
    fs::{self, OpenOptions, create_dir_all, remove_dir_all},
    io::{self, Write},
    path::Path,
    process::exit,
};

use glob::glob;

use crate::{
    fs::{FileProperty, extract_archive},
    metadata::{Meta, PACKAGE_INSTALL_PATH},
    sys::{args_contains, error, get_binary_path, is_root},
};

/// Installs a .spf package.
///
/// The path of the .spf package (`spf_package_path` (as [`String`]))
/// must be provided
///
/// To install, the .spf package is extracted to the cwd using [`extract_archive`].
/// The metadata file stored inside is read for the various details.
///
/// If architecture in the metadata file doesn't match the current system arch, exit.
///
/// The versions are compared using [`check_version`] and
/// [`parse_installed_and_packaged_versions`] before installation.
///
/// When it actually gets to installing the files using [`install_files`],
/// the tree inside the extracted package contains the directories and files
/// that are written to their new metadata file located in [`PACKAGE_INSTALL_PATH`],
/// and are copied to their respective locations.
pub fn spf_install(mut spf_package_path: String) {
    // Root is required for this command
    if !is_root() {
        error("To execute this action, please run spf as root.")
    }

    let package_path_check = Path::new(&spf_package_path);

    // If a package isn't provided, or provided package isn't a `.spf`
    // package, prompt user to do so.
    if spf_package_path.is_empty()
        || !package_path_check
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("spf"))
        || !package_path_check.exists()
    {
        error("Please provide a .spf package")
    }

    println!("Loading package: {spf_package_path}\n");

    // Extract the provided package
    extract_archive(&spf_package_path);

    spf_package_path = FileProperty::name(&spf_package_path);

    // Get location of the package path (`spf_package_path`)
    let packaged_metadata_file = spf_package_path.replace(".spf", "/META");

    let package_metadata = Meta::from(&packaged_metadata_file);

    // Retrieve the metadata. Wish there was a better way to do this
    let package_name = package_metadata.clone().load_value("PROJECT_NAME");
    let package_version = package_metadata.clone().load_value("VERSION");
    let package_desc = package_metadata.clone().load_value("DESCRIPTION");
    let package_license = package_metadata.clone().load_value("LICENSE");
    let package_authors = package_metadata.clone().load_value("AUTHORS");
    let package_arch = package_metadata.load_value("ARCH");

    // The extracted path has no extension, so remove it
    let extracted_package_path = spf_package_path.replace(".spf", "");

    if package_arch != ARCH && !args_contains("--ignore-arch") {
        remove_dir_all(extracted_package_path).expect("Failed to cleanup");

        error(&format!(
            "Package architecture: {package_arch} doesn't match current system architecture ({ARCH}).\n\
            Bypass by passing: `--ignore-arch`"
        ))
    }

    // The formatted package name looks something like:
    //
    // spf-v0.1.0, or my_project-v0.0.1
    //
    // Formatted by <project name>-<project_ version>
    let package_name_formatted = &format!("{package_name}-{package_version}");

    ask_user_to_install(
        package_name_formatted,
        &package_desc,
        &package_license,
        &package_authors,
        &package_arch,
        &extracted_package_path,
    );

    // To be safe, move the metadata file. But check if it's already installed first.
    let package_meta_path_install_location = format!("{PACKAGE_INSTALL_PATH}{package_name}");

    // Check if the package is already installed. If so, proceed to check version conflicts.
    // Otherwise, skip and proceed to copying files.
    if Path::new(&package_meta_path_install_location).exists() {
        check_version(
            &package_meta_path_install_location,
            &package_name,
            &package_version,
            &spf_package_path,
        );
    }

    // Start creating directories and copying files
    println!("Installing: {package_name}-{package_version} from ./{spf_package_path}");

    // Install all the necessary paths, including the metadata file.
    install_files(
        &packaged_metadata_file,
        package_meta_path_install_location,
        &extracted_package_path,
    );

    println!("Cleaning up...");

    // Clean up by removing the extracted package
    remove_dir_all(&extracted_package_path).unwrap_or_else(|_| {
        panic!("Failed to clean up and remove directory \"{extracted_package_path}\"")
    });

    println!("\nSuccessfully installed {package_name}-{package_version}!");

    exit(0)
}

/// Take loaded metadata and display it as package information, before asking the
/// user if they want to install the package.
///
/// The following are used as identifying information for the package:
///     - `package_name_formatted` ([`str`])
///     - `packaged_project_description` ([`String`])
///     - `packaged_project_license` ([`String`])
///     - `packaged_project_authors` ([`String`])
///     - `packaged_project_packaged_arch` ([`String`])
///     - `extracted_package_path` ([`String`])
///
/// If user aborts, clean up by deleting `extracted_package_path` (stored as [`String`])
fn ask_user_to_install(
    package_name_formatted: &str,
    packaged_project_description: &str,
    packaged_project_license: &str,
    packaged_project_authors: &str,
    packaged_project_packaged_arch: &str,
    extracted_package_path: &str,
) {
    // Display project info/metadata
    println!(
        "Do you want to proceed to install {package_name_formatted}?\n\n\
        Description: {packaged_project_description}\n\
        License(s): {packaged_project_license}\n\
        Author(s): {packaged_project_authors}\n\
        Arch: {packaged_project_packaged_arch}\n\n\
        (Y/N)"
    );

    let mut proceed_to_install = String::new();
    io::stdin()
        .read_line(&mut proceed_to_install)
        .expect("Failed to readline");

    println!();

    if proceed_to_install.trim().to_lowercase() != "y" {
        remove_dir_all(extracted_package_path).unwrap_or_else(|_| {
            panic!("Failed to clean up and remove directory \"{extracted_package_path}\"")
        });

        println!("Aborted.");
        exit(0)
    }
}

/// Compares versions if the package is already installed. User can
/// abort/proceed as they wish.
///
/// Unfortunately, you need to provide:
///     - A metadata path (`package_meta_path`),
///     - The packaged project name (`packaged_project_name`)
///     - The packaged project version (`packaged_project_version`)
///     - The path of the `.spf` package (`spf_package_path`)
///
/// All are provided as [`str`].
///
/// [`check_version`] checks the version by comparing the version of
/// the installed package (if installed), and the version of the package.
/// The versions themselves are actually parsed in [`parse_installed_and_packaged_versions`].
///
/// The two versions retrieved look like normal numbers, and their sizes are compared to see
/// what version is the newest.
/// Examples: `v0.2.0` is retrieved as `020` (`20`), and `v1.4.2` is retrieved as `142`. `142`
/// is larger than `20` which shows that `v1.4.2` is the newest version
///
/// It then asks you if you want to update or downgrade the package, where
/// you have to enter `y` in user input.
fn check_version(
    package_meta_path: &str,
    packaged_project_name: &str,
    packaged_project_version: &str,
    spf_package_path: &str,
) {
    // Loads package version
    let installed_version = Meta::from(package_meta_path).load_value("VERSION");

    // Check for version differences
    if packaged_project_version == installed_version {
        // If no version differences, let the user know that the package is already installed.
        println!(
            "{packaged_project_name}-{packaged_project_version} is already installed. Continue?"
        );
    } else {
        // Compare versions by leaving only numbers, combine them together,
        // then parsing them as `usize`.
        //
        // `project_version_num` is the version in the package, `installed_version_num`
        // is what's already installed.
        let (project_version_num, installed_version_num) =
            parse_installed_and_packaged_versions(packaged_project_version, &installed_version);

        // Make sure that the versions were parsed correctly.
        if project_version_num == 0 || installed_version_num == 0 {
            error(&format!(
                "Failed to parse version in the installed package \"{package_meta_path}\""
            ))
        }

        // Prompt the user if they actually want to update
        println!(
            "Do you want to {} {packaged_project_name}-{installed_version} -> {packaged_project_name}-{packaged_project_version}?",
            // Determine action the user is taking.
            //
            // if `project_version_num` > `installed_version_num`, it means that the
            // user is updating.
            //
            // Otherwise, it likely means the user is downgrading.
            if project_version_num > installed_version_num {
                "update"
            } else {
                "downgrade"
            }
        );
    }

    println!("(Y/N)");

    // Take in the user input
    let mut proceed_to_install = String::new();
    io::stdin()
        .read_line(&mut proceed_to_install)
        .expect("Failed to readline");

    println!();

    let extracted_package_path = spf_package_path.replace(".spf", "");

    // If user declines, clean up and exit. Otherwise, proceed and end function
    if proceed_to_install.trim().to_lowercase() != "y" {
        fs::remove_dir_all(&extracted_package_path).unwrap_or_else(|_| {
            panic!("Failed to clean up and remove directory \"{extracted_package_path}\"")
        });

        println!("Aborted");
        exit(0)
    }
}

/// Takes the installed and packaged versions, then formats them to
/// remove special characters and `v`. It also converts them to
/// [`usize`].
///
/// Returns modified versions of `packaged_project_version` and `installed_version`
///
/// ```
/// let (project_version_num, installed_version_num) = parse_installed_and_packaged_versions(packaged_project_version, &installed_version);
/// ```
fn parse_installed_and_packaged_versions(
    packaged_project_version: &str,
    installed_version: &str,
) -> (usize, usize) {
    // Removes `v`, any special characters, and converts to usize.
    // Returns `0` is something fails.
    let remove_chars_and_to_usize = |version: &str| -> usize {
        let special_chars = &['(', ')', ',', '\"', '.', ';', ':', '\''];

        version
            // Remove special chars
            .replace(special_chars, "")
            // Remove the `v` if found
            .trim_matches('v')
            // Convert to `usize`
            .parse::<usize>()
            // Make it `0` if something fails for whatever reason
            .unwrap_or(0)
    };

    // Convert the 2 versions
    let project_version_num = remove_chars_and_to_usize(packaged_project_version);
    let installed_version_num = remove_chars_and_to_usize(installed_version);

    (project_version_num, installed_version_num)
}

/// Copy files or create directories needed to install a program.
///
/// Start with the metadata file, then move on to the packaged paths
/// to also be installed.
///
/// The files are copied from files from `path_to_search`
/// (part of `extracted_package_path` (as [`String`])).
///
/// The paths that are modified are then marked/inscribed into
/// the project metadata file (`project_meta_file` (as [`String`]))
fn install_files(
    packaged_metadata_file: &str,
    package_meta_path_install_location: String,
    extracted_package_path: &str,
) {
    // Clean up by removing the extracted package (only used on errors)
    let cleanup = || -> () {
        remove_dir_all(extracted_package_path).expect("Failed to clean up extracted package");
    };

    // Copy the packaged metadata file to its install location
    fs::copy(packaged_metadata_file, &package_meta_path_install_location).
        unwrap_or_else(|err| panic!("Failed to copy file \"{packaged_metadata_file}\" -> \"{package_meta_path_install_location}\": {err}"));

    // Remove the metadata file that was packaged
    fs::remove_file(packaged_metadata_file).expect("Failed to delete metadata file");

    // Get the location to look for the paths to copy
    let path_to_search = &format!("./{extracted_package_path}/**/*");

    // Init the new metadata file.
    //
    // Allows appending, it creates it, and opens it
    let mut project_meta_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(package_meta_path_install_location)
        .unwrap();

    // Write the header for defining installed paths
    project_meta_file
        .write_all(b"\n:::PATH DEFINE START:::\n")
        .unwrap_or_else(|_| {
            panic!("Failed to write path define header to \"{project_meta_file:#?}\"")
        });

    // Go through and install packaged paths
    for found_path in glob(path_to_search)
        .unwrap_or_else(|_| panic!("Failed to collect directories at \"{path_to_search}\""))
    {
        // File/folder to be copied
        let file_from_archive = found_path.unwrap().to_str().unwrap().to_string().clone();

        // Path where `file_from_archive` will be copied to
        // `extracted_package_path` is removed to prevent conflicts
        let file_destination = file_from_archive.replacen(extracted_package_path, "", 1);

        // `file_destination` as `Path`
        let path_to_create = Path::new(&file_destination);

        // If the path to be copied is a directory, simply create it instead of copying it.
        if Path::new(&file_from_archive).is_dir() {
            create_dir_all(path_to_create).unwrap_or_else(|err| {
                panic!(
                    "Failed to create directory \"{}\": {err}",
                    path_to_create.display()
                )
            });

            continue;
        }

        println!("    Copying \"{file_from_archive}\" -> \"{file_destination}\"");

        // Check if the file being copied `file_destination` is spf itself. If so,
        // replace the old binary (current binary path) with the new binary
        // (`file_from_archive`/`file_destination`)
        if file_destination == get_binary_path() {
            self_replace::self_replace(&file_from_archive).unwrap();
        }

        fs::copy(&file_from_archive, &file_destination).unwrap_or_else(|err| {
            cleanup();
            panic!("Failed to copy file \"{file_from_archive}\" -> \"{file_destination}\": {err}")
        });

        // Write the path of the file to later be removed when uninstalled.
        // Basically shows that the program is installed.
        project_meta_file
            .write_all(format!("{file_destination}\n").as_bytes())
            .unwrap();

        // Check that the path was copied/created correctly
        if !path_to_create.exists() {
            cleanup();
            panic!("Failed to copy file: \"{file_destination}\"")
        }
    }
}
