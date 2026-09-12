//! Functions related to and providing the ability to convert a `.spf`
//! package to a `.deb` package (and vice versa).
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{self, create_dir, remove_dir_all, remove_file, rename},
    io::stdin,
    path::Path,
    process::Command,
};

use crate::{
    fs::{FileProperty, create_tar_archive, extract_archive},
    sys::error,
};

/// Lets user know that this program has no warranty, and is not responsible
fn disclaimer() -> Result<(), std::io::Error> {
    println!(
        "I, or this program, are not responsible for any damage to your system caused by this command.\n\
        Install converted packages at your own risk.\n\n\
        Press enter to continue..."
    );

    stdin().read_line(&mut String::new())?;

    Ok(())
}

/// TODO: Make native tar functions
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

    disclaimer()?;

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

    let source_file_name = match FileProperty::name(source_package_path) {
        Ok(name) => name,
        Err(err) => error(&format!("Failed to get source name: {err}")),
    };

    let spf_metadata_path = source_file_name.replace(".spf", "/META");

    // `control.tar.xz` contains the debian package metadata, so extract if applicable.
    // Otherwise, extract assumed location of `.spf` package metadata, then collect the
    // metadata.
    let mut source_metadata: String = if source_is_debian {
        println!("        Extracting \"control.tar.xz\"...");
        //extract_archive("tar", "control.tar.xz")?;

        create_dir("control")?;

        // Manually extract to specific destination just cause
        Command::new("tar")
            .arg("-xf")
            .arg("control.tar.xz")
            .arg("-C")
            .arg("./control")
            .output()?;

        remove_file("control.tar.xz")?;

        println!("        Reading \"control\"...");

        fs::read_to_string("control/control")?
    } else {
        println!("        Reading \"{spf_metadata_path}\"...");
        fs::read_to_string(&spf_metadata_path)?
    };

    println!("        Cleaning up metadata collection...");

    if source_is_debian {
        remove_dir_all("control")?;
    } else {
        remove_file(&spf_metadata_path)?;
    }

    /* Conversion of metadata */

    println!("    Converting metadata...");

    // println!("{source_metadata}");
    // exit(0);

    let cloned_metadata = source_metadata.clone();
    let metadata_lines: Vec<&str> = cloned_metadata.lines().collect();

    for line in metadata_lines {
        if source_is_debian {
            if line.starts_with("Package:") {
                source_metadata = source_metadata.replace("Package:", "PROJECT_NAME =");
            } else if line.starts_with("Version:") {
                source_metadata = source_metadata.replace("Version:", "VERSION =");
            } else if line.starts_with("Description:") {
                source_metadata = source_metadata.replace("Description:", "DESCRIPTION =");
            } else if line.starts_with("Homepage:") {
                source_metadata = source_metadata.replace("Homepage:", "REPOSITORY =");
            } else if line.starts_with("Maintainer:") {
                source_metadata = source_metadata.replace("Maintainer:", "AUTHORS =");
            } else if line.starts_with("Architecture:") {
                source_metadata = source_metadata.replace("Architecture:", "ARCH =");
            } else {
                source_metadata =
                    source_metadata.replace(&format!("{line}\n"), &format!("#{line}\n"));
            };
        } else {
            if line.starts_with("PROJECT_NAME =") {
                source_metadata = source_metadata.replace("PROJECT_NAME =", "Package:");
            } else if line.starts_with("VERSION =") {
                source_metadata = source_metadata.replace("VERSION =", "Version:");
            } else if line.starts_with("DESCRIPTION =") {
                source_metadata = source_metadata.replace("DESCRIPTION =", "Description:");
            } else if line.starts_with("REPOSITORY =") {
                source_metadata = source_metadata.replace("REPOSITORY =", "Homepage:");
            } else if line.starts_with("LICENSE =") {
                source_metadata.retain(|_| line.starts_with("LICENSE ="));
            } else if line.starts_with("AUTHORS =") {
                source_metadata = source_metadata.replace("AUTHORS =", "Maintainer:");
            } else if line.starts_with("ARCH =") {
                source_metadata = source_metadata.replace("ARCH =", "Architecture:");
            };
        }
    }

    if source_is_debian {
        source_metadata = source_metadata
            .lines()
            .filter(|line| !line.trim().starts_with("#"))
            .map(|line| line.trim_end_matches("#"))
            .collect::<Vec<_>>()
            .join("\n");
    } else {
        source_metadata = source_metadata
            .lines()
            .filter(|line| !line.starts_with("LICENSE =") || !line.starts_with("#"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /* File copying */;

    println!("    Copying files...");

    if source_is_debian {
        create_dir("data")?;

        // Manually extract to specific destination just cause
        Command::new("tar")
            .arg("-xf")
            .arg("data.tar.xz")
            .arg("-C")
            .arg("./data")
            .output()?;

        remove_file("data.tar.xz")?;

        let new_deb_dest = source_file_name.replace(".deb", "");
        rename("data", &new_deb_dest)?;

        fs::write(format!("{new_deb_dest}/META"), source_metadata)?;

        println!("    Packaging...");

        create_tar_archive(output_package_path, &new_deb_dest)?;

        remove_dir_all(new_deb_dest)?;
    } else {
        let extracted_spf_package = spf_metadata_path.replace("/META", "");
        println!("{extracted_spf_package}");

        rename(extracted_spf_package, "data")?;

        println!("    Packaging...");

        create_tar_archive("data.tar.xz", "data")?;

        remove_dir_all("data")?;
    }

    println!("Done!");

    Ok(())
}
