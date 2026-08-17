//! Functions related to and installing a .spf package.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{self, OpenOptions, create_dir_all},
    io::{self, Write},
    path::Path,
    process::exit,
};

use glob::glob;

use crate::{
    fs::{FileProperty, extract_archive},
    metadata::get_meta_value,
    sys::{Error, is_root},
};

static SOURCE_FILE: &str = "src/install.rs";

/// Installs a .spf package.
/// The path of the .spf package (`spf_package_path`) must be provided
///
/// Goes through checking the version of the package or a potential
/// already-installed version
pub fn spf_install(mut spf_package_path: String) {
    // Root is required for this command
    if !is_root() {
        Error::normal("To execute this action, please run spf as root.")
    }

    // If a package isn't provided, or provided package isn't a `.spf`
    // package, prompt user to do so.
    if spf_package_path.is_empty() || !spf_package_path.ends_with(".spf") {
        Error::normal("Please provide a .spf package")

    // If the file straight up doesn't exist, let them know
    } else if !Path::new(&spf_package_path).exists() {
        Error::normal(&format!("File not found: {spf_package_path}"))
    }

    println!("Loading package: {spf_package_path}\n");

    // Extract the provided package
    extract_archive(&spf_package_path);

    spf_package_path = FileProperty::name(&spf_package_path);

    // Get location of the package path (`spf_package_path`)
    let packaged_metadata_file = spf_package_path.replace(".spf", "/META");

    // Retrieve the metadata. Wish there was a better way to do this
    let packaged_project_name = get_meta_value(packaged_metadata_file.clone(), "PROJECT_NAME");
    let packaged_project_version = get_meta_value(packaged_metadata_file.clone(), "VERSION");
    let packaged_project_description =
        get_meta_value(packaged_metadata_file.clone(), "DESCRIPTION");
    let packaged_project_license = get_meta_value(packaged_metadata_file.clone(), "LICENSE");
    let packaged_project_authors = get_meta_value(packaged_metadata_file.clone(), "AUTHORS");
    let packaged_project_packaged_arch = get_meta_value(packaged_metadata_file.clone(), "ARCH");

    // The formatted package name looks something like:
    //
    // spf-v0.1.0, or my_project-v0.0.1
    //
    // Formatted by <project name>-<project_ version>
    let package_name_formatted = &format!("{packaged_project_name}-{packaged_project_version}");

    // The extracted path has no extension, so remove it
    let extracted_package_path = spf_package_path.replace(".spf", "");

    ask_user_to_install(
        package_name_formatted,
        packaged_project_description,
        packaged_project_license,
        packaged_project_authors,
        packaged_project_packaged_arch,
        extracted_package_path.clone(),
    );

    // To be safe, move the metadata file. But check if it's already installed first.
    let package_meta_path_install_location =
        format!("/usr/share/spf/packages/{packaged_project_name}");

    // Check if the package is already installed. If so, proceed to check version conflicts.
    // Otherwise, skip and proceed to copying files.
    if Path::new(&package_meta_path_install_location).exists() {
        check_version(
            &package_meta_path_install_location,
            &packaged_project_name,
            &packaged_project_version,
            &spf_package_path,
        );
    }

    // Start creating directories and copying files
    println!(
        "Installing: {packaged_project_name}-{packaged_project_version} from ./{spf_package_path}"
    );

    // Install all the necessary paths, including the metadata file.
    install_files(
        packaged_metadata_file,
        package_meta_path_install_location,
        extracted_package_path.clone(),
    );

    println!("Cleaning up...");

    // Clean up by removing the extracted package
    fs::remove_dir_all(&extracted_package_path).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "spf_install()",
            113,
            &format!("Failed to clean up and remove directory \"{extracted_package_path}\": {err}"),
        )
    });

    println!("\nSuccessfully installed {packaged_project_name}-{packaged_project_version}!");

    exit(0)
}

/// Take loaded metadata and display it as package information, before asking the
/// user if they want to install the package.
///
/// If user aborts, clean up by deleting `extracted_package_path`
fn ask_user_to_install(
    package_name_formatted: &str,
    packaged_project_description: String,
    packaged_project_license: String,
    packaged_project_authors: String,
    packaged_project_packaged_arch: String,
    extracted_package_path: String,
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
        .unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "ask_user_to_install()",
                152,
                &format!("Failed to readline: {err}"),
            )
        });

    println!();

    if proceed_to_install.trim().to_lowercase() != "y" {
        fs::remove_dir_all(&extracted_package_path).unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "ask_user_to_install()",
                164,
                &format!(
                    "Failed to clean up and remove directory \"{extracted_package_path}\": {err}"
                ),
            )
        });

        println!("Aborted.");
        exit(0)
    }
}

/// Compares versions if the package is already installed. User can
/// abort/proceed as they wish.
fn check_version(
    package_meta_path: &str,
    packaged_project_name: &str,
    packaged_project_version: &str,
    spf_package_path: &str,
) {
    // Reads the package meta, then rips the value from `VERSION`.
    let installed_version = fs::read_to_string(package_meta_path)
        .unwrap()
        // Separate the lines
        .split('\n')
        // Find the line that contains the `VERSION` metadata tag
        .find(|entry| entry.contains("VERSION"))
        .unwrap_or_else(|| {
            Error::fatal(
                SOURCE_FILE,
                "check_version()",
                195,
                "Failed to retrieve project name from metadata!",
            )
        })
        // Retrieve the version
        .replace("VERSION = ", "");

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
            Error::fatal(
                SOURCE_FILE,
                "check_version()",
                222,
                &format!("Failed to parse version in the installed package {package_meta_path}"),
            )
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
        .unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "check_version()",
                254,
                &format!("Failed to readline: {err}"),
            )
        });

    println!();

    let extracted_package_path = spf_package_path.replace(".spf", "");

    // If user declines, clean up and exit. Otherwise, proceed and end function
    if proceed_to_install.trim().to_lowercase() != "y" {
        fs::remove_dir_all(&extracted_package_path).unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "check_version()",
                269,
                &format!(
                    "Failed to clean up and remove directory \"{extracted_package_path}\": {err}"
                ),
            )
        });

        Error::normal("Aborted");
    }
}

/// Takes the installed and packaged versions, then formats them to
/// remove special characters and `v`. It also converts them to
/// `usize`.
///
/// Returns modified versions of `packaged_project_version` and `installed_version`
fn parse_installed_and_packaged_versions(
    packaged_project_version: &str,
    installed_version: &str,
) -> (usize, usize) {
    // Removes `v`, any special characters, and converts to usize.
    // Returns `0` is something fails.
    let remove_chars_and_to_usize = |version: &str| -> usize {
        let special_chars = &['(', ')', ',', '\"', '.', ';', ':', '\''][..];

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
/// (part of `extracted_package_path`).
///
/// The paths that are modified are then marked/inscribed into
/// the project metadata file (`project_meta_file`)
fn install_files(
    packaged_metadata_file: String,
    package_meta_path_install_location: String,
    extracted_package_path: String,
) {
    // Copy the packaged metadata file to its install location
    fs::copy(&packaged_metadata_file, &package_meta_path_install_location).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "install_files()",
            332,
            &format!("Failed to copy file \"{packaged_metadata_file}\" -> \"{package_meta_path_install_location}\": {err}"),
        )
    });

    // Remove the metadata file that was packaged
    fs::remove_file(&packaged_metadata_file).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "install_files()",
            342,
            &format!("Failed to delete metadata file: {err}"),
        )
    });

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
        .unwrap();

    // Go through and install packaged paths
    for found_path in glob(path_to_search).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "install_files()",
            369,
            &format!("Failed to collect directories at \"{path_to_search}\": {err}"),
        )
    }) {
        // File/folder to be copied
        let file_from_archive = found_path.unwrap().to_str().unwrap().to_string().clone();

        // Path where `file_from_archive` will be copied to
        // `extracted_package_path` is removed to prevent conflicts
        let file_destination = file_from_archive.replacen(&extracted_package_path, "", 1);

        // `file_destination` as `Path`
        let path_to_create = Path::new(&file_destination);

        // If the path to be copied is a directory, simply create it instead of copying it.
        if Path::new(&file_from_archive).is_dir() {
            create_dir_all(path_to_create).unwrap();
            continue;
        }

        println!("    Copying \"{file_from_archive}\" -> \"{file_destination}\"");

        fs::copy(&file_from_archive, &file_destination).unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "install_files()",
                395,
                &format!(
                    "Failed to copy file \"{file_from_archive}\" -> \"{file_destination}\": {err}"
                ),
            )
        });

        // Write the path of the file to later be removed when uninstalled.
        // Basically shows that the program is installed.
        project_meta_file
            .write_all(format!("{file_destination}\n").as_bytes())
            .unwrap();

        // Check that the path was copied/created correctly
        if !path_to_create.exists() {
            Error::fatal(
                SOURCE_FILE,
                "install_files()",
                413,
                &format!("Failed to copy file: \"{file_destination}\""),
            )
        }
    }
}
