//! Functions related to and providing tools for retrieving spf package
//! metadata.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;

use crate::sys::Error;

static SOURCE_FILE: &str = "src/metadata.rs";

/// Where spf package metadata is installed to.
pub static PACKAGE_INSTALL_PATH: &str = "/usr/share/spf/packages/";

/// Tool for retrieving file metadata.
///
/// Reads the metadata file from `meta_path`, then searches for
/// the set value from the `category`.
///
/// Returns to `String`.
pub fn get_meta_value(meta_path: String, category: &str) -> String {
    if !matches!(
        category,
        "PROJECT_NAME" | "VERSION" | "DESCRIPTION" | "LICENSE" | "AUTHORS" | "ARCH"
    ) {
        Error::fatal(
            SOURCE_FILE,
            "get_meta_category()",
            27,
            &format!("Invalid metadata category being retrieved: {category}"),
        )
    }

    // Parse the metadata and retrieve the value that is needed.
    fs::read_to_string(&meta_path)
        .unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "get_meta_category()",
                38,
                &format!("Failed to retrieve project metadata: {err}"),
            )
        })
        // Seperate the lines
        .split('\n')
        // Find the line that contains the category
        .find(|entry| entry.contains(category))
        .unwrap_or_else(|| {
            Error::fatal(
                SOURCE_FILE,
                "get_meta_category()",
                50,
                &format!(
                    "Failed to retrieve metadata value from category \
                    \"{category}\" in file \"{meta_path}\""
                ),
            )
        })
        // Return the value of the category by stripping out the category name
        .replace(&format!("{category} = "), "")
}
