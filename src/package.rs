//! Functions related to and providing the ability to create a .spf package.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{self, create_dir_all, File},
    io::Write,
    path::Path,
    process::exit,
};

use crate::{
    fs::{create_archive_of_dir, FileProperty},
    sys::Error,
};

static SOURCE_FILE: &str = "src/package.rs";

/// Create .spf package
///
/// See the example config in spf/samples/example_config
pub fn create_spf_package(package_config: &str, mut output_location: &str) {
    // Check if the package config file name is valid, whether it
    // has extension or if it's empty.
    // If the file has an extension or is not provided / is empty,
    // exit.
    if !FileProperty::extension(package_config).is_empty() || package_config.is_empty() {
        Error::normal("Please provide a text file with no file extension!")

    // Check if package config exists
    } else if !Path::new(package_config).exists() {
        Error::normal("Package config not found!")
    }

    // Make sure the output file is a .spf file
    if !FileProperty::extension(output_location).ends_with("spf") {
        Error::normal(&format!(
            "Your provided output location (\"{output_location}\") must be a .spf file."
        ))
    }

    // The output archive is initially created as a directory so strip the
    // file extension.
    output_location = output_location.trim_end_matches(".spf");

    fs::create_dir_all(output_location).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "create_spf_package()",
            48,
            &format!("Failed to create directory \"{output_location}\": {err}"),
        )
    });

    println!("Compiling files and directories...\n");

    let package_config_contents = &*fs::read_to_string(package_config).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "create_spf_package()",
            59,
            &format!("Failed to open file: {err}"),
        )
    });

    let project_meta_file_path = File::create(format!("{output_location}/META")).unwrap();

    // Collect the package metadata, and write them to the packages metadata file (`project_meta_file_path`)
    write_project_meta_config(package_config_contents, &project_meta_file_path);

    copy_package_paths(package_config_contents, output_location);

    // Zip the output folder into a .spf package
    println!("Packaging...");

    // If the provided output name already ends with `.spf`, do nothing (return empty &str).
    // Otherwise, append extension.
    let archive_name = &format!(
        "{output_location}{}",
        if output_location.ends_with(".spf") {
            ""
        } else {
            ".spf"
        }
    );

    package_to_spf(output_location, archive_name);

    // Cleanup directory that was compressed
    fs::remove_dir_all(output_location).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "create_spf_package()",
            92,
            &format!("Failed to clean up packaging: {err}"),
        )
    });

    println!("\nDone! Packaged to: \"./{archive_name}\"");

    exit(0)
}

/// Write to `project_meta_file`. The metadata written could be one of the following:
/// - PROJECT_NAME (name of the package/project that is being packaged)
/// - VERSION     (version of release)
/// - DESCRIPTION (description of the contents, package or project)
/// - LICENSE     (license of the package)
/// - AUTHORS     (authors behind the project/package)
/// - ARCH        (packaged architecture)
fn write_project_meta_config(package_config_contents: &str, mut project_meta_file: &File) {
    let mut project_meta_buffer: Vec<&str> = vec![];

    // Go through the metadata file and collect needed package as necessary.
    for entry in package_config_contents.lines() {
        // If the read entry is the following:
        // - Meta define start header
        // - empty line
        // - Comment ('#')
        //
        // Skip that entry. Otherwise, if the entry is the meta define closer,
        // stop parsing the metadata and skip to writing the buffer.
        if entry == ":::META DEFINE START:::" || entry.is_empty() || entry.starts_with('#') {
            continue;
        } else if entry == ":::META DEFINE END:::" {
            break;
        }

        // Get the category of metadata. Could be "PROJECT_NAME", "VERSION", "LICENSE", and others.
        let meta_category = entry.split(" = ").next().unwrap_or_else(|| {
            Error::fatal(
                SOURCE_FILE,
                "write_project_meta_config()",
                132,
                &format!("Failed to parse entry: {entry}"),
            )
        });

        // Verify that the detected metadata category is valid. You can see the
        // available categories in the `matches!` block.
        //
        // If the category is valid, write it to the buffer (`project_meta_buffer`).
        // Otherwise, throw an error.
        if matches!(
            meta_category,
            "PROJECT_NAME" | "VERSION" | "DESCRIPTION" | "LICENSE" | "AUTHORS" | "ARCH"
        ) {
            // Collect the metadata to `project_meta_buffer` if the category is valid
            println!("Collecting metadata: {entry}");

            // Push the metadata to the buffer
            project_meta_buffer.push(entry);

            continue;
        }

        // The error could either be:
        // - Invalid entry formatting, where the meta category and value are not
        // separated correctly
        // - The metadata category found is not a valid category.
        Error::fatal(
            SOURCE_FILE,
            "write_project_meta_config()",
            162,
            &format!(
                "Failed to parse entry \"{entry}\": {}",
                if !entry.contains(" = ") {
                    "Invalid entry formatting"
                } else {
                    "Invalid metadata category"
                }
            ),
        );
    }

    println!("Writing package metadata...");

    // Convert the vectorized buffer to an easily writable string
    let processed_meta_buffer = project_meta_buffer.join("\n");

    // Finally write the buffer to `project_meta_file`
    project_meta_file
        .write_all(processed_meta_buffer.as_bytes())
        .unwrap_or_else(|err|
            Error::fatal(
                SOURCE_FILE,
                "write_project_meta_config()",
                186,
                &format!("Failed to write package config metadata buffer to \"{project_meta_file:#?}\": {err}")));
}

/// Parse defined paths in `package_config_contents` that are located under
/// ":::PATH DEFINE START:::".
///
/// The defined paths are formatted as such:
/// `original/file/path:/location/to/install`
///
/// These paths are copied to their respective locations. `output_location`
/// is treated as the "root" of the filesystem, such as "/".
fn copy_package_paths(package_config_contents: &str, output_location: &str) {
    // This easily allows the parser to skip the project metadata, which
    // has already been parsed thanks to `write_project_meta_config()`.
    let mut enable_skipping_project_meta = true;

    // Go through and copy paths as needed
    for entry in package_config_contents.lines() {
        // The header that helps initialized the parsing of paths
        let path_start_header_is_read = entry == ":::PATH DEFINE START:::";

        // If `enable_skipping_project_meta` hasn't been disabled, skip the entry
        if entry != ":::PATH DEFINE START:::" && enable_skipping_project_meta {
            continue;
        }

        // If the path define header has been encountered, disable the the ability
        // to skip entries.
        if path_start_header_is_read {
            enable_skipping_project_meta = false;
        }

        // If the read entry is the following:
        // - Path define start header (`path_start_header`)
        // - empty line
        // - Comment ('#')
        //
        // Skip that entry. Otherwise, if the entry is the meta define closer,
        // stop copying the paths then later proceed to packaging the copied files.
        if path_start_header_is_read || entry.is_empty() || entry.starts_with('#') {
            continue;
        } else if entry == ":::PATH DEFINE END:::" {
            break;
        }

        // Split the parsed path configs into their respective values.
        // `original_file_path` will be copied to `file_destination`
        let original_file_path = entry.split(':').next().unwrap().to_string();
        let file_destination = entry.split(':').next_back().unwrap().to_string();

        // `file_destination` needs to start with '/' so the file destination has
        // a clearly defined root path.
        if !file_destination.starts_with('/') {
            Error::fatal(
                SOURCE_FILE,
                "copy_package_paths()",
                243,
                &format!("Path destination must start from the root: {file_destination}"),
            )
        }

        // Double check that the entry is formatted correctly
        check_path_entry(entry, original_file_path.clone(), file_destination.clone());

        // Get the file name of the path
        let original_file_name = FileProperty::name(&original_file_path);

        // Here to avoid copying files that don't exist to directories
        // that are supposed to be empty
        if original_file_name.is_empty() {
            continue;
        }

        // Cleans up duplicate "/"'s to clean up the path string
        let destination_of_file =
            format!("{output_location}/{file_destination}").replace("//", "/");

        // Get the directories that will be created
        let dirs_to_create =
            get_dirs_to_create(output_location, destination_of_file, &original_file_name);

        // The missing directories that need to be created
        let destination_directories_to_create =
            &format!("{output_location}/{}", dirs_to_create.join("/"));

        // The finalized destination of the path
        let final_file_destination =
            format!("{destination_directories_to_create}/{original_file_name}");

        println!("Copying: {original_file_path} -> {final_file_destination}");

        // Create the directories listed in `destination_directory`
        create_dir_all(destination_directories_to_create).unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "copy_package_paths()",
                283,
                &format!("Failed to create \"{destination_directories_to_create}\": {err}"),
            )
        });

        // Copy `original_file_path` to `final_file_destination`
        fs::copy(&original_file_path, final_file_destination).unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "copy_package_paths()",
                293,
                &format!("Failed to copy file \"{original_file_path}\": {err}"),
            )
        });
    }
}

/// Take the copied directories located in `output_location` and compress
/// them to what was provided as `archive_name`
fn package_to_spf(output_location: &str, archive_name: &str) {
    // Get parent directory
    let parent_directories = &Path::new(output_location)
        .parent()
        .unwrap()
        .to_string_lossy();

    let directory_to_compress = &FileProperty::name(archive_name).replace(".spf", "");

    // Take the directories (`parent_directories`) inside `directory_to_compress`,
    // then package them to whatever `archive_name` is.
    create_archive_of_dir(parent_directories, archive_name, directory_to_compress);

    // Check if the package actually exists.
    if !Path::new(archive_name).exists() {
        Error::fatal(
            SOURCE_FILE,
            "package_to_spf()",
            320,
            &format!("Failed to package to \"{output_location}\": File not found"),
        )
    }
}

/// Check if:
/// 1. original file is mentioned
/// 2. file destination is mentioned
/// 3. entry is formatted correctly
fn check_path_entry(entry: &str, original: String, destination: String) {
    let paths_are_included = !original.is_empty() && !destination.is_empty();

    let entry_formatting_is_preserved = format!("{original}:{destination}") == entry;

    let paths_are_split = entry.contains(':');

    if !(paths_are_included && entry_formatting_is_preserved && paths_are_split) {
        Error::fatal(
            SOURCE_FILE,
            "check_path_entry()",
            341,
            &format!("Failed to parse entry: {entry}"),
        )
    }
}

/// Get a list of the directories that are needed to be created inside
/// the `.spf` package.
///
/// Requires:
/// - Output location of package
/// - Destination of the path to be copied
/// - The name of the original file name
fn get_dirs_to_create(
    output_location: &str,
    destination_of_file: String,
    original_file_name: &str,
) -> Vec<String> {
    let mut dirs_to_create = vec![];

    // Help avoid creating duplicate directories. Things break otherwise.
    let extra_dirs: Vec<&str> = output_location.split('/').collect();

    for dir in destination_of_file.split('/').skip(extra_dirs.len()) {
        // Avoid creating a directory named the same as the file that contains it.
        // I still don't understand why that was happening
        if dir == original_file_name {
            break;
        }
        dirs_to_create.push(dir.to_string())
    }

    dirs_to_create
}
