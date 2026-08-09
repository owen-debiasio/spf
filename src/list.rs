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

        let get_meta_category = |category: &str| -> String {
            fs::read_to_string(&package_path)
                .unwrap()
                .split('\n')
                .find(|entry| entry.contains(category))
                .unwrap_or_else(|| error("Failed to retrieve project name from metadata!"))
                .replace(&format!("{category} = "), "")
        };

        let package_version = get_meta_category("VERSION");

        let package_desc = get_meta_category("DESCRIPTION");

        println!("> {package_name} {package_version}\n    {package_desc}")
    }

    exit(0)
}
