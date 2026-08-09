use crate::error;
use is_root::is_root;
use std::{fs, io, path::Path};

pub fn remove_spf_package(packages: Vec<String>) {
    if !is_root() {
        error("To execute this action, please run spf as root.")
    }

    if packages.is_empty() {
        error("Provide the package you want to remove.")
    }

    // Check if packages are installed
    for package in &packages {
        let package_meta_path = format!("/usr/share/spf/packages/{package}");

        if !Path::new(&package_meta_path).exists() {
            error(&format!("Package not installed: {package}"))
        }
    }

    println!("You are about to remove the following package(s):\n");

    // Some bullshit of a "function"?? idkk
    let get_meta_category = |meta_path: String, category: &str| -> String {
        fs::read_to_string(meta_path)
            .unwrap()
            .split('\n')
            .find(|entry| entry.contains(category))
            .unwrap_or_else(|| error("Failed to retrieve project name from metadata!"))
            .replace(&format!("{category} = "), "")
    };

    // List packages to be removed idk
    for package in &packages {
        let package_meta_path = format!("/usr/share/spf/packages/{package}");

        let package_version = get_meta_category(package_meta_path, "VERSION");

        if packages.len() > 6 {
            print!("{package}-{package_version}")
        } else {
            println!("{package}-{package_version}")
        }
    }

    println!("\nProceed?\n(Y/N)");

    let mut proceed_to_remove = String::new();
    io::stdin()
        .read_line(&mut proceed_to_remove)
        .expect("failed to readline");

    if proceed_to_remove.trim().to_lowercase() != "y" {
        error("Aborted.")
    } else {
        println!()
    }

    for package in packages {
        let package_meta_path = format!("/usr/share/spf/packages/{package}");

        let package_version = get_meta_category(package_meta_path, "VERSION");

        println!("Removing: {package}-{package_version}");
    }

    error("package removal still has yet to be implemented! I'll deal with it soon!")
}
