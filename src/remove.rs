use std::path::Path;

use crate::error;

pub fn remove_spf_package(packages: Vec<String>) {
    if packages.is_empty() {
        error("Provide the package you want to remove.")
    }

    for package in packages {
        let package_meta_path = format!("/usr/share/spf/packages/{package}");

        if !Path::new(&package_meta_path).exists() {
            error(&format!("Package not installed: {package}"))
        }
    }
}
