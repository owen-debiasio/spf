use std::{
    fs::{self, OpenOptions, create_dir_all},
    io::{self, Write},
    path::Path,
    process::exit,
};

use glob::glob;
use is_root::is_root;

use crate::{error, fs::extract_archive};

/// Installs a *.spf package
pub fn spf_install(spf_package_path: String) {
    if !is_root() {
        error("To execute this action, please run spf as root.")
    }

    // If file is provided
    if spf_package_path.is_empty() || spf_package_path.ends_with(".spf") {
        error("Please provide a .spf package")
    } else if !Path::new(&spf_package_path).exists() {
        error(&format!("File not found: {spf_package_path}"))
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

    let packaged_project_name = get_meta_category("PROJECT_NAME");
    let packaged_project_version = get_meta_category("VERSION");
    let packaged_project_description = get_meta_category("DESCRIPTION");
    let packaged_project_license = get_meta_category("LICENSE");
    let packaged_project_authors = get_meta_category("AUTHORS");
    let packaged_project_packaged_arch = get_meta_category("ARCH");

    // Display project info/metadata
    println!(
        "Do you want to proceed to install {packaged_project_name}-{packaged_project_version}?\n\n\
        Description: {packaged_project_description}\n\
        License(s): {packaged_project_license}\n\
        Author(s): {packaged_project_authors}\n\
        Arch: {packaged_project_packaged_arch}\n\n\
        (Y/N)"
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

    // To be safe, move the metadata file. But check if it's already installed first.
    let package_meta_path = format!("/usr/share/spf/packages/{packaged_project_name}");

    // WARNING: The code ahead is FUUUUUCKED
    if Path::new(&package_meta_path).exists() {
        let get_installed_version = {
            fs::read_to_string(&package_meta_path)
                .unwrap()
                .split('\n')
                .find(|entry| entry.contains("VERSION"))
                .unwrap_or_else(|| error("Failed to retrieve project name from metadata!"))
                .replace("VERSION = ", "")
        };

        // Check for version differences
        let installed_version = get_installed_version;
        if packaged_project_version == installed_version {
            println!(
                "{packaged_project_name}-{packaged_project_version} is already installed. Continue?"
            );
        } else {
            // Compare versions by leaving only numbers, combine them together,
            // then parsing them as `usize`. Then compare the two versions.
            // Works best if the version if formatted as "v0.1.0" or "v0.1.0-1".
            //
            // `project_version_num` is the version in the package, `installed_version_num`
            // is what's already installed.
            //
            // If my code shits and pisses itself, the versions default to `0`, which
            // will lead to spf shitting itself (intentionally, see below). It's also prone
            // to issues, so I hope it works properly.
            let special_chars = &['(', ')', ',', '\"', '.', ';', ':', '\''][..];
            let project_version_num = packaged_project_version
                .replace(special_chars, "")
                .trim_matches('v')
                .parse::<usize>()
                .unwrap_or(0);

            let installed_version_num = installed_version
                .replace(special_chars, "")
                .trim_matches('v')
                .parse::<usize>()
                .unwrap_or(0);

            // This is where spf shits itself intentionally.
            #[allow(clippy::ifs_same_cond)]
            if project_version_num == 0 {
                error(&format!(
                    "Failed to parse version in the package \"{spf_package_path}\": \
                    {packaged_project_version}->{project_version_num}"
                ))
            } else if project_version_num == 0 {
                error(&format!(
                    "Failed to parse version in the installed package {package_meta_path}: \
                    {installed_version}->{installed_version_num}"
                ))
            }

            println!(
                "Do you want to {} {packaged_project_name}-{installed_version} -> {packaged_project_name}-{packaged_project_version}?",
                // Determine action
                if project_version_num > installed_version_num {
                    "update"
                } else {
                    "downgrade"
                }
            );
        }

        println!("\n(Y/N)");

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
    }

    fs::copy(&metadata_file, &package_meta_path).unwrap();

    fs::remove_file(&metadata_file)
        .unwrap_or_else(|err| error(&format!("Failed to delete metadata file: {err}")));

    let path_to_search = &format!("./{extracted_package_path}/**/*");

    let mut project_meta_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(package_meta_path)
        .unwrap();

    project_meta_file
        .write_all(b":::PATH DEFINE START:::\n")
        .unwrap();

    // Start creating directories and copying files
    println!(
        "Installing: {packaged_project_name}-{packaged_project_version} from ./{spf_package_path}\n"
    );

    for entry in glob(path_to_search).unwrap_or_else(|err| {
        error(&format!(
            "Failed to collect directories at \"{path_to_search}\": {err}"
        ))
    }) {
        let file_from_archive = entry.unwrap().to_str().unwrap().to_string();
        let file_destination = file_from_archive.replacen(&extracted_package_path, "", 1);

        let path_to_create = Path::new(&file_destination);
        if path_to_create.is_dir() {
            if path_to_create.exists() {
                create_dir_all(&file_destination).unwrap_or_else(|err| {
                    error(&format!(
                        "Failed to create directory \"{file_destination}\": {err}"
                    ))
                });

                // Check that the directory was copied
                if !path_to_create.exists() {
                    error(&format!(
                        "Failed to create directory: \"{file_destination}\""
                    ))
                }
            }

            continue;
        }

        let final_destination = file_from_archive.replacen(&extracted_package_path, "", 1);

        fs::copy(file_from_archive, &final_destination).unwrap();

        // Write the path of the file to later be removed when uninstalled.
        // Basically shows that the program is installed.
        project_meta_file
            .write_all(format!("{final_destination}\n").as_bytes())
            .unwrap();

        // Check that the file was copied
        if !path_to_create.exists() {
            error(&format!("Failed to copy file: \"{file_destination}\""))
        }
    }

    println!("Cleaning up...");

    fs::remove_dir_all(&extracted_package_path).unwrap_or_else(|err| {
        error(&format!(
            "Failed to clean up and remove directory \"{extracted_package_path}\": {err}"
        ))
    });

    println!("Successfully installed {packaged_project_name}-{packaged_project_version}!");

    exit(0)
}
