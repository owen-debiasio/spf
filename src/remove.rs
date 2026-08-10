use crate::sys::{error, is_root};
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

    // Some bullshit of a "function"?? idkk
    let get_meta_category = |meta_path: String, category: &str| -> String {
        fs::read_to_string(meta_path)
            .unwrap()
            .split('\n')
            .find(|entry| entry.contains(category))
            .unwrap_or_else(|| error("Failed to retrieve project name from metadata!"))
            .replace(&format!("{category} = "), "")
    };

    // Variables kept to make sure things only one a certain amount
    let mut enable_list_packages = true;
    for package in &packages_to_list {
        let package_meta_path = format!("/usr/share/spf/packages/{package}");

        // Check if packages are installed
        if !Path::new(&package_meta_path).exists() {
            error(&format!("Package not installed: {package}"))
        }

        let package_version = get_meta_category(package_meta_path.clone(), "VERSION");

        let package_formatted = format!("{package}-{package_version}");

        // List packages, if able to
        if enable_list_packages {
            println!("You are about to remove the following package(s):\n");

            // the code here is absolute dogshit so you can try to
            // fix it if you want but I won't cause it works
            if packages_to_list.len() > 6 {
                for package in &packages_to_list {
                    let listed_package_meta_path = format!("/usr/share/spf/packages/{package}");
                    let listed_package_version =
                        get_meta_category(listed_package_meta_path, "VERSION");

                    print!("{package}-{listed_package_version}")
                }
                println!();
            } else {
                for package in &packages_to_list {
                    let listed_package_meta_path = format!("/usr/share/spf/packages/{package}");
                    let listed_package_version =
                        get_meta_category(listed_package_meta_path, "VERSION");

                    println!("{package}-{listed_package_version}")
                }
            }

            println!("\nProceed?\n(Y/N)");

            let mut proceed_to_remove = String::new();
            io::stdin()
                .read_line(&mut proceed_to_remove)
                .expect("failed to readline");

            if proceed_to_remove.trim().to_lowercase() != "y" {
                error("Aborted.")
            }

            enable_list_packages = false
        }

        println!("\nRemoving: {package_formatted}");

        let mut enable_path_deletion = false;
        for entry in fs::read_to_string(package_meta_path.clone())
            .unwrap_or_else(|err| {
                error(&format!(
                    "Failed to retrieve contents of \"{package_meta_path}\": {err}"
                ))
            })
            .lines()
        {
            if entry == ":::PATH DEFINE START:::" {
                enable_path_deletion = true;
                continue;
            }

            if !enable_path_deletion {
                continue;
            }

            println!("    Removing \"{entry}\"...");

            // Delete the paths
            if Path::new(entry).is_file() {
                remove_file(entry)
                    .unwrap_or_else(|err| error(&format!("Failed to remove \"{entry}\": {err}")))
            } else {
                remove_dir_all(entry)
                    .unwrap_or_else(|err| error(&format!("Failed to remove \"{entry}\": {err}")))
            }
        }

        println!("    Removing package entry...");
        remove_file(&package_meta_path).unwrap_or_else(|err| {
            error(&format!("Failed to remove \"{package_meta_path}\": {err}"))
        });

        println!("Successfully removed {package_formatted}!");
    }

    println!("\nSuccessfully removed package(s)!");

    exit(0);
}
