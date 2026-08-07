use std::{
    fs::{self, File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
    process::exit,
};

use crate::{error, fs::create_archive_of_dir};

/// # Create .spf package
///
/// See the example config in spf/samples/example_config
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

    fs::create_dir(output_location).unwrap_or_else(|err| {
        error(&format!(
            "Failed to create directory \"{output_location}\": {err}"
        ))
    });

    println!("Compiling files and directories...\n");

    let dir_list = &*fs::read_to_string(dir_list)
        .unwrap_or_else(|err| error(&format!("Failed to open file: {err}")));

    let (mut meta_has_parsed, mut paths_have_parsed) = (false, false);
    let mut project_meta_file = File::create(format!("{output_location}/META")).unwrap();

    for entry in dir_list.lines() {
        // Skip comments or empty lines
        if entry.starts_with('#') || entry.is_empty() {
            continue;
        }

        if entry == "=== PROJECT_META_BEGIN ===" {
            meta_has_parsed = true;
            continue;
        } else if entry == "===  PROJECT_META_END  ===" {
            meta_has_parsed = false;
            continue;
        }

        let meta_category = entry.split(" = ").next().unwrap();

        // Verify the meta category

        if meta_has_parsed {
            if matches!(
                meta_category,
                "PROJECT_NAME" | "LICENSE" | "AUTHORS" | "ARCH"
            ) {
                // Write the metadata to the metadata file
                println!("Writing metadata: {entry}");
                project_meta_file
                    .write_all(format!("{entry}\n").as_bytes())
                    .unwrap();
            } else {
                error(&format!(
                    "Failed to parse entry \"{entry}\": Invalid metadata category"
                ))
            }
        }

        if meta_has_parsed {
            continue;
        }

        if entry == "=== DEFINE PATHS BEGIN ===" {
            paths_have_parsed = true;
            continue;
        } else if entry == "===  DEFINE PATHS END  ===" {
            paths_have_parsed = false;
            continue;
        }

        let original_file = entry.split(':').next().unwrap().to_string();
        let file_destination = entry.split(':').next_back().unwrap().to_string();

        let destination_of_file =
            format!("{output_location}/{file_destination}").replace("//", "/");

        if !meta_has_parsed && paths_have_parsed {
            // check if the entry is formatted correctly
            check_path_entry(entry, original_file.clone(), file_destination.clone());
        }

        let mut dirs_to_create = vec![];

        for dir in destination_of_file.split('/').skip(1) {
            dirs_to_create.push(dir.to_string())
        }

        let destination_directory = &format!("{output_location}/{}", dirs_to_create.join("/"));
        println!("Copying: {destination_directory}");
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
    println!("\nPackaging...");
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
fn check_path_entry(entry: &str, original: String, destination: String) {
    if !(!original.is_empty()
        && !destination.is_empty()
        && format!("{original}:{destination}") == entry
        && entry.contains(':'))
    {
        error(&format!("Failed to parse entry: {entry}"))
    }
}
