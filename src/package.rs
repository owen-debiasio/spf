use std::{
    fs::{self, File, create_dir_all},
    io::Write,
    path::Path,
    process::exit,
};

use crate::{
    fs::{FileProperty, create_archive_of_dir},
    sys::error,
};

/// # Create .spf package
///
/// See the example config in spf/samples/example_config or
/// the example package output in spf/samples/example_package
pub fn create_spf_package(package_config: &str, mut output_location: &str) {
    // Check if the package config file name is valid, whether it
    // has extension or if it's empty.
    // If the file has an extension or is not provided / is empty,
    // exit.
    if !FileProperty::extension(package_config).is_empty() || package_config.is_empty() {
        error("Please provide a text file with no file extension!")

    // Check if package config exists
    } else if !Path::new(package_config).exists() {
        error("Package config not found!")
    }

    // Make sure the output file is a .spf file
    if !FileProperty::extension(output_location).ends_with("spf") {
        error(&format!(
            "Your provided output location (\"{output_location}\") must be a .spf file."
        ))
    }

    // The output archive is initially created as a directory so strip the
    // file extension.
    output_location = output_location.trim_end_matches(".spf");

    fs::create_dir_all(output_location).unwrap_or_else(|err| {
        error(&format!(
            "Failed to create directory \"{output_location}\": {err}"
        ))
    });

    println!("Compiling files and directories...\n");

    let package_config_contents = &*fs::read_to_string(package_config)
        .unwrap_or_else(|err| error(&format!("Failed to open file: {err}")));

    let (mut meta_has_parsed, mut paths_have_parsed) = (false, false);
    let mut project_meta_file = File::create(format!("{output_location}/META")).unwrap();

    for entry in package_config_contents.lines() {
        // Skip comments or empty lines
        if entry.starts_with('#') || entry.is_empty() {
            continue;
        }

        // Verify the meta category

        if entry == ":::META DEFINE START:::" {
            meta_has_parsed = true;
            continue;
        } else if entry == ":::META DEFINE END:::" {
            meta_has_parsed = false;
            continue;
        }

        let meta_category = entry
            .split(" = ")
            .next()
            .unwrap_or_else(|| error(&format!("Failed to parse entry: {entry}")));

        if meta_has_parsed {
            if matches!(
                meta_category,
                "PROJECT_NAME" | "VERSION" | "DESCRIPTION" | "LICENSE" | "AUTHORS" | "ARCH"
            ) {
                // Write the metadata to the metadata file
                println!("Writing metadata: {entry}");
                project_meta_file
                    .write_all(format!("{entry}\n").as_bytes())
                    .unwrap();
            } else {
                error(&format!(
                    "Failed to parse entry \"{entry}\": {}",
                    if !entry.contains(" = ") {
                        "Invalid entry formatting"
                    } else {
                        "Invalid metadata category"
                    }
                ));
            }
        }

        if meta_has_parsed {
            continue;
        }

        if entry == ":::PATH DEFINE START:::" {
            paths_have_parsed = true;
            println!();
            continue;
        } else if entry == ":::PATH DEFINE END:::" {
            paths_have_parsed = false;
            println!();
            continue;
        }

        let original_file_path = entry.split(':').next().unwrap().to_string();
        let file_destination = entry.split(':').next_back().unwrap().to_string();

        if !file_destination.starts_with('/') {
            error(&format!(
                "Entered destination must start from the root: {file_destination}"
            ))
        }

        if !meta_has_parsed && paths_have_parsed {
            // check if the entry is formatted correctly
            check_path_entry(entry, original_file_path.clone(), file_destination.clone());
        }

        let original_file_file_name = FileProperty::name(&original_file_path);

        // Cleans up duplicate "/"'s.
        let destination_of_file =
            format!("{output_location}/{file_destination}").replace("//", "/");

        let mut dirs_to_create = vec![];
        let extra_dirs: Vec<&str> = output_location.split('/').collect();

        for dir in destination_of_file.split('/').skip(extra_dirs.len()) {
            // Avoid creating a directory named the same as the file that contains it.
            // I still don't understand why that was happening
            if dir == original_file_file_name {
                break;
            }
            dirs_to_create.push(dir)
        }

        let destination_directory = &format!("{output_location}/{}", dirs_to_create.join("/"));

        println!("Copying: {destination_directory}/{original_file_file_name}");

        create_dir_all(destination_directory).unwrap();

        // Here to avoid copying files that don't exist to directories
        // that are supposed to be empty
        if original_file_file_name.is_empty() {
            continue;
        }

        fs::copy(
            &original_file_path,
            format!("{destination_directory}/{original_file_file_name}"),
        )
        .unwrap_or_else(|err| {
            error(&format!(
                "Failed to copy file \"{original_file_path}\": {err}"
            ))
        });
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

    // Get parent directory
    let parent_directories = &Path::new(output_location)
        .parent()
        .unwrap()
        .to_string_lossy();

    create_archive_of_dir(
        parent_directories,
        archive_name,
        &FileProperty::name(archive_name).replace(".spf", ""),
    );

    if !Path::new(archive_name).exists() {
        error(&format!(
            "Failed to package to \"{output_location}\": File not found"
        ))
    }

    fs::remove_dir_all(output_location)
        .unwrap_or_else(|err| error(&format!("Failed to clean up packaging: {err}")));

    println!("\nDone! Packaged to: \"{archive_name}\"");

    exit(0)
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
        error(&format!("Failed to parse entry: {entry}"))
    }
}
