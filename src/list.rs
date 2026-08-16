//! Functions related to and providing the ability list any packages installed
//! by spf.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{fs, process::exit};

use glob::glob;

use crate::sys::Error;

static SOURCE_FILE: &str = "src/list.rs";

/// Lists packages that are be installed. You can optionally provide
/// a string of text to match (`optional_string`).
pub fn list_packages(optional_string: String) {
    println!("Installed packages:\n");

    // Package metadata install directory
    static META_INSTALL: &str = "/usr/share/spf/packages/";

    // Go through the package metadata install directory
    for package_path_raw in glob("/usr/share/spf/packages/*").unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "list_packages()",
            15,
            &format!("Failed to collect directories at \"{META_INSTALL}\": {err}"),
        )
    }) {
        // The path of the package metadata. The name of the metadata file is the name
        // of the package.
        let package_path = package_path_raw.unwrap().to_str().unwrap().to_string();

        // Name of the package to be listed
        let package_name = package_path.replace(META_INSTALL, "");

        if !package_name.contains(&optional_string) {
            continue;
        }

        // Parses the package metadata
        let get_meta_category = |category: &str| -> String {
            fs::read_to_string(&package_path)
                .unwrap()
                // Separate lines
                .split('\n')
                // Finds the category to search
                .find(|entry| entry.contains(category))
                .unwrap_or_else(|| {
                    Error::fatal(
                        SOURCE_FILE,
                        "list_packages()",
                        46,
                        "Failed to retrieve project name from metadata!",
                    )
                })
                // Retrieve the value found in the category
                .replace(&format!("{category} = "), "")
        };

        // Retrieve the package name and version
        let package_version = get_meta_category("VERSION");
        let package_desc = get_meta_category("DESCRIPTION");

        println!("> {package_name}-{package_version}\n    {package_desc}")
    }

    exit(0)
}
