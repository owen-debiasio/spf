use std::path::Path;

use crate::error;

/// Installs a *.spf package
pub fn spf_install(spf_package_path: String) {
    check_package_name(spf_package_path.clone());

    println!("Installing: {spf_package_path}")
}

/// Makes sure the file name is compatible. Manually
/// parses the string. Super inefficient lol
fn check_package_name(package_path: String) {
    // if file exists
    if !Path::new(&package_path).exists() {
        error(&format!("File not found: {package_path}"))
    }

    // if file ends with .spf
    if !package_path.ends_with(".spf") {
        error("Not .spf")
    }
}
