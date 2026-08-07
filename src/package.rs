use std::{
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
    process::exit,
};

use crate::{error, fs::create_archive_of_dir};

/// # Create .spf package
///
/// dir list format:
/// path/of/original/file:/path/of/install/location
///
/// Example:
/// target/debug/spf:/usr/bin/spf
///
/// Note: `output_location` defaults to cwd if empty
pub fn create_spf_package(dir_list: &str, output_location: &str) {
    if !Path::new(dir_list).exists() {
        error("Directory list not found!")
    }

    // Check file extension
    let dir_list_has_extension = Path::new(dir_list)
        .extension()
        .unwrap_or_default()
        .is_empty();

    if !dir_list_has_extension || dir_list.is_empty() {
        error("Please provide a text file with no file extension!")
    }

    // init the output directory
    if output_location.contains('/') {
        error(
            "Please provide the output directory name rather than the destination of the output file.",
        )
    }

    if PathBuf::from(output_location)
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        != String::new()
    {
        error("Your output location must not contain a file extension.")
    }

    fs::create_dir_all(output_location).unwrap_or_else(|err| {
        error(&format!(
            "Failed to create directory \"{output_location}\": {err}"
        ))
    });

    println!("Compiling files and directories...");

    let dir_list = &*fs::read_to_string(dir_list)
        .unwrap_or_else(|err| error(&format!("Failed to open file: {err}")));

    for entry in dir_list.lines() {
        let original_file = entry.split(':').next().unwrap().to_string();
        let file_destination = entry.split(':').next_back().unwrap().to_string();

        // check if the entry is formatted correctly
        check_entry(entry, original_file.clone(), file_destination.clone());

        let path_str = original_file.replace("//", "/");
        let destination_of_file =
            format!("{output_location}/{file_destination}").replace("//", "/");

        let mut dirs_to_create = vec![];

        let separated_dirs: Vec<&str> = path_str.split('/').collect();
        let amount_of_dirs = separated_dirs.len();

        for (i, dir) in destination_of_file.split('/').skip(1).enumerate() {
            if i < amount_of_dirs - 1 {
                dirs_to_create.push(dir.to_string());
            } else {
                break;
            }
        }

        let destination_directory = &format!("{output_location}/{}", dirs_to_create.join("/"));
        create_dir_all(destination_directory).unwrap();

        let file_name = PathBuf::from(&original_file)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // Here to avoid copying files that don't exist to directories
        // that are supposed to be empty
        if file_name.is_empty() {
            continue;
        }

        fs::copy(
            &original_file,
            format!("{destination_directory}/{file_name}"),
        )
        .unwrap_or_else(|err| error(&format!("Failed to copy file \"{original_file}\": {err}")));
    }

    let archive_name = &format!(
        "{output_location}{}",
        // skip appending extension if already included
        if output_location.ends_with(".spf") {
            ""
        } else {
            ".spf"
        }
    );

    // Zip the output folder into a .spf package
    println!("Packaging...");
    create_archive_of_dir(archive_name, output_location);

    println!("Cleaning up...");
    fs::remove_dir_all(output_location)
        .unwrap_or_else(|err| error(&format!("Failed to clean up packaging: {err}")));

    println!("\nDone! Packaged to: \"{archive_name}\"");
    exit(0)
}

/// Check if:
/// 1. original file is mentioned
/// 2. file destination is mentioned
/// 3. entry is formatted correctly
fn check_entry(entry: &str, original: String, destination: String) {
    if !(!original.is_empty()
        && !destination.is_empty()
        && format!("{original}:{destination}") == entry
        && entry.contains(':'))
    {
        error(&format!("Failed to parse entry: {entry}"))
    }
}
