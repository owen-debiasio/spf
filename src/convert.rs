//! Functions related to and providing the ability to convert a `.spf`
//! package to a `.deb` package (and vice versa).
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{self, remove_file},
    path::Path,
};

use crate::{
    fs::{FileProperty, extract_archive},
    sys::error,
};

pub fn convert(source_package_path: &str, output_package_path: &str) -> Result<(), std::io::Error> {
    /* Input file checks */

    if source_package_path.is_empty() {
        error("Please provide an input package path!")
    }

    let source_file = Path::new(source_package_path);

    if !source_file.exists() {
        error(&format!("File \"{source_package_path}\" does not exist!"))
    }

    let source_file_ext = match FileProperty::extension(source_package_path) {
        Ok(ext) => ext,
        Err(err) => error(&format!("Failed to get source file extension: {err}")),
    };

    if !matches!(source_file_ext.as_str(), "spf" | "deb") {
        error(&format!(
            "Unsupported input package format: .{source_file_ext}"
        ))
    }

    /* Output file checks */

    if output_package_path.is_empty() {
        error("Please provide an output package path!")
    }

    let output_file_ext = match FileProperty::extension(output_package_path) {
        Ok(ext) => ext,
        Err(err) => error(&format!("Failed to get output file extension: {err}")),
    };

    if !matches!(output_file_ext.as_str(), "spf" | "deb") {
        error(&format!(
            "Unsupported output package format: {output_file_ext}"
        ))
    }

    println!("Converting \"{source_package_path}\" -> \"{output_package_path}\"...");

    let source_is_debian = source_file_ext == "deb";

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

    // `debian-binary` isn't used, so delete it
    if source_is_debian {
        remove_file("debian-binary")?;
    }

    /* Start reading metadata */

    println!("    Reading metadata...");

    let spf_metadata_path = &FileProperty::name(source_package_path)?.replace(".spf", "/META");

    // `control.tar.xz` contains the debian package metadata, so extract if applicable.
    // Otherwise, extract assumed location of `.spf` package metadata, then collect the
    // metadata.
    let mut source_metadata: String = if source_is_debian {
        println!("        Extracting \"control.tar.xz\"...");
        extract_archive("tar", "control.tar.xz")?;

        remove_file("control.tar.xz")?;

        println!("        Reading \"control\"...");

        fs::read_to_string("control")?
    } else {
        println!("        Reading \"{spf_metadata_path}\"...");
        fs::read_to_string(spf_metadata_path)?
    };

    println!("        Cleaning up metadata collection...");

    if source_is_debian {
        remove_file("control")?;
    } else {
        remove_file(spf_metadata_path)?;
    }

    /* Conversion of metadata */

    println!("        Converting metadata...");

    Ok(())
}
