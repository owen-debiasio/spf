//! Functions related to and providing the ability list any packages installed
//! by spf.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::process::exit;

use glob::glob;

use crate::{
    Error,
    metadata::{PACKAGE_INSTALL_PATH, get_meta_value},
};

static SOURCE_FILE: &str = "src/list.rs";

/// Lists packages that are be installed. You can optionally provide
/// a string of text to match (`optional_string`).
pub fn list_packages(optional_string: String) {
    println!("Installed packages:\n");

    // Go through the package metadata install directory
    for package_path_raw in glob(&format!("{PACKAGE_INSTALL_PATH}/*")).unwrap_or_else(|err| {
        Error::fatal(
            SOURCE_FILE,
            "list_packages()",
            25,
            &format!("Failed to collect directories at \"{PACKAGE_INSTALL_PATH}\": {err}"),
        )
    }) {
        // The path of the package metadata. The name of the metadata file is the name
        // of the package.
        let package_path = package_path_raw.unwrap().to_str().unwrap().to_string();

        // Name of the package to be listed
        let package_name = package_path.replace(PACKAGE_INSTALL_PATH, "");

        if !package_name.contains(&optional_string) {
            continue;
        }

        // Retrieve the package name and version
        let package_version = get_meta_value(package_path.clone(), "VERSION");
        let package_desc = get_meta_value(package_path, "DESCRIPTION");

        println!("> {package_name}-{package_version}\n    {package_desc}")
    }

    exit(0)
}
