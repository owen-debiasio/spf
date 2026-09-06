//! Functions related to and providing the ability to convert a `.spf`
//! package to a `.deb` package (and vice versa).
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;

use crate::{fs::extract_archive, sys::error};

pub fn convert(source_package_path: &str, output_package_path: &str) -> Result<(), std::io::Error> {
    /* Input file checks */

    if source_package_path.is_empty() {
        error("Please provide an input package path!")
    }

    let source_file = Path::new(source_package_path);

    if !source_file.exists() {
        error(&format!("File \"{source_package_path}\" does not exist!"))
    }

    let source_file_ext = source_file
        .extension()
        .unwrap_or_default()
        .display()
        .to_string();

    if !matches!(source_file_ext.as_str(), "spf" | "deb") {
        error(&format!(
            "Unsupported input package format: .{source_file_ext}"
        ))
    }

    /* Output file checks */

    if output_package_path.is_empty() {
        error("Please provide an output package path!")
    }

    let output_file = Path::new(output_package_path);

    let output_file_ext = output_file
        .extension()
        .unwrap_or_default()
        .display()
        .to_string();

    if !matches!(output_file_ext.as_str(), "spf" | "deb") {
        error(&format!(
            "Unsupported output package format: {output_file_ext}"
        ))
    }

    println!("Converting \"{source_package_path}\" -> \"{output_package_path}\"...");

    // Dynamically choose the extractor for the package.
    //
    // If the package is a `.deb` file, use `ar`.
    // Otherwise, use `tar.`
    let extracter = if source_file_ext == "spf" {
        "tar"
    } else if source_file_ext == "deb" {
        "ar"
    } else {
        // Use `tar` for fallback (however it's likely to fail on unsupported binaries)
        "tar"
    };

    println!("    Extracting \"{source_package_path}\"...");

    extract_archive(extracter, source_package_path)?;

    Ok(())
}
