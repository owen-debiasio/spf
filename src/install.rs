use std::{
    fs::{self, create_dir_all},
    io,
    path::Path,
    process::exit,
};

use glob::glob;

use crate::{error, fs::extract_archive};

/// Installs a *.spf package
pub fn spf_install(spf_package_path: String) {
    // If file is provided
    if spf_package_path.is_empty() {
        error("Please provide a .spf package")
    }

    // if file exists
    if !Path::new(&spf_package_path).exists() {
        error(&format!("File not found: {spf_package_path}"))
    }

    // if file ends with .spf
    if !spf_package_path.ends_with(".spf") {
        error("Not .spf")
    }

    println!("Loading package: {spf_package_path}\n");

    extract_archive(&spf_package_path);

    let metadata_file = spf_package_path.replace(".spf", "/META");

    // Retrieve the metadata
    let stored_metadata = fs::read_to_string(&metadata_file).unwrap_or_else(|err| {
        error(&format!(
            "Failed to retrieve metadata from \"{spf_package_path}\": {err}"
        ))
    });

    let get_meta_category = |category: &str| -> String {
        stored_metadata
            .split('\n')
            .find(|entry| entry.contains(category))
            .unwrap_or_else(|| error("Failed to retrieve project name from metadata!"))
            // Because the compiler suck ass, it won't let me use .next_back()
            // so I have to use .replace() and pray that it works smh
            .replace(&format!("{category} = "), "")
    };

    let project_name = get_meta_category("PROJECT_NAME");
    let project_license = get_meta_category("LICENSE");
    let project_authors = get_meta_category("AUTHORS");
    let project_packaged_arch = get_meta_category("ARCH");

    // Display project info/metadata
    println!(
        "Do you want to proceed to install \"{project_name}\"?\n\
        License(s): {project_license}\n\
        Author(s): {project_authors}\n\
        Arch: {project_packaged_arch}\n\
        \n(Y/N)"
    );

    let mut proceed_to_install = String::new();
    io::stdin()
        .read_line(&mut proceed_to_install)
        .expect("failed to readline");

    println!();

    let extracted_package_path = spf_package_path.replace(".spf", "");

    if proceed_to_install.trim().to_lowercase() != "y" {
        fs::remove_dir_all(&extracted_package_path).unwrap_or_else(|err| {
            error(&format!(
                "Failed to clean up and remove directory \"{extracted_package_path}\": {err}"
            ))
        });

        error("Aborted.")
    }

    println!("Installing: {spf_package_path}\n");

    // To be safe, just delete the metadata file.
    fs::remove_file(&metadata_file)
        .unwrap_or_else(|err| error(&format!("Failed to delete metadata file: {err}")));

    let path_to_search = &format!("./{extracted_package_path}/**/*");

    // Start creating directories and copying files
    for entry in glob(path_to_search).unwrap_or_else(|err| {
        error(&format!(
            "Failed to collect directories at \"{path_to_search}\": {err}"
        ))
    }) {
        let file_from_archive = entry.unwrap().to_str().unwrap().to_string();
        let file_destination = file_from_archive.replacen(&extracted_package_path, "", 1);

        let folder_to_create = Path::new(&file_destination);
        if folder_to_create.is_dir() {
            if folder_to_create.exists() {
                create_dir_all(&file_destination).unwrap_or_else(|err| {
                    error(&format!(
                        "Failed to create directory \"{file_destination}\": {err}"
                    ))
                });
            }

            continue;
        }

        let final_destination = file_from_archive.replacen(&extracted_package_path, "", 1);

        fs::copy(file_from_archive, final_destination).unwrap();

        // Check that the file was copied
        if !Path::new(&file_destination).exists() {
            error(&format!("Failed to copy file: \"{file_destination}\""))
        }
    }

    println!("Successfully installed \"{project_name}\"!");

    exit(0)
}
