use std::{fs, process::exit};

use crate::error;
use glob::glob;

pub fn list_packages(optional_string: String) {
    println!("Installed packages:\n");

    static META_INSTALL: &str = "/usr/share/spf/packages/";

    for package_path_raw in glob("/usr/share/spf/packages/*").unwrap_or_else(|err| {
        error(&format!(
            "Failed to collect directories at \"{META_INSTALL}\": {err}"
        ))
    }) {
        let package_path = package_path_raw.unwrap().to_str().unwrap().to_string();
        let package_name = package_path.replace(META_INSTALL, "");

        if !package_name.contains(&optional_string) {
            continue;
        }

        let package_version = fs::read_to_string(package_path)
            .unwrap()
            .split('\n')
            .find(|entry| entry.contains("VERSION = "))
            .unwrap()
            .replace("VERSION = ", "");

        println!("{package_name} {package_version}")
    }

    exit(0)
}
