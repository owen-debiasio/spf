//! Functions related to and providing the ability to convert a `.spf`
//! package to a `.deb` package (and vice versa).
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{self, File, remove_dir_all, remove_file, rename},
    io::{Write, stdin, stdout},
    path::Path,
};

use glob::glob;

use deb_rust::{DebArchitecture, DebFile, binary::DebPackage};

use crate::{
    fs::{FileProperty, create_tar_archive, extract_ar_archive, extract_tar_archive},
    metadata::Meta,
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
    println!("    Extracting \"{source_package_path}\"...");

    let source_is_debian = source_file_ext == "deb";

    // Dynamically extract the package.
    //
    // If the package is a `.deb` file, use `ar`.
    // Otherwise, use `tar.`
    if source_file_ext == "deb" {
        extract_ar_archive(source_package_path, ".")?;
    } else {
        extract_tar_archive(source_package_path, ".", "")?;
    };

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

        extract_tar_archive("./control.tar.xz", "./control", "xz")?;

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
        //remove_file(&spf_metadata_path)?;
    }

    /* Conversion of metadata */

    println!("    Converting metadata...");

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
            } else if line.starts_with("AUTHORS =") {
                source_metadata = source_metadata.replace("AUTHORS =", "Maintainer:");
            } else if line.starts_with("LICENSE =") {
                source_metadata = source_metadata.replace("LICENSE = ", "#");
            } else if line.starts_with("ARCH =") {
                source_metadata = source_metadata.replace("ARCH =", "Architecture:");
            } else {
                source_metadata =
                    source_metadata.replace(&format!("{line}\n"), &format!("#{line}\n"));
            };
        }
    }

    source_metadata = source_metadata
        .lines()
        .filter(|line| !line.starts_with("#"))
        .map(|line| line.trim_end_matches("#"))
        .collect::<Vec<_>>()
        .join("\n")

    /* File copying */;

    println!("    Copying files...");

    if source_is_debian {
        extract_tar_archive("data.tar.xz", "./data", "xz")?;

        remove_file("data.tar.xz")?;

        let new_deb_dest = source_file_name.replace(".deb", "");
        rename("data", &new_deb_dest)?;

        fs::write(format!("{new_deb_dest}/META"), source_metadata)?;

        println!("    Packaging...");

        create_tar_archive(output_package_path, &new_deb_dest, "xz")?;

        remove_dir_all(new_deb_dest)?;
    } else {
        println!("    Packaging...");
        println!("        Setting metadata...");

        let package_metadata = Meta::from(&spf_metadata_path)?;
        let name = package_metadata.clone().load_value("PROJECT_NAME")?;
        let version = package_metadata.clone().load_value("VERSION")?;
        let desc = package_metadata.load_value("DESCRIPTION")?;

        remove_file(spf_metadata_path)?;

        let mut package = DebPackage::new(&name);

        package = package
            .set_version(&version)
            .set_description(&desc)
            .set_architecture(DebArchitecture::All);

        println!("        Writing paths...");

        let extracted_source = source_file_name.replace(".spf", "");

        for path in glob(&format!("{extracted_source}/**/*")).expect("Failed to get paths") {
            let current_path = path?.display().to_string();

            print!("\r\x1B[K            Writing path: \"{current_path}\"");
            stdout().flush()?;

            package = if Path::new(&current_path).is_file() {
                package.with_file(DebFile::from_path(
                    &current_path,
                    current_path.replace(&extracted_source, ""),
                )?)
            } else {
                package.with_dir(&current_path, &current_path.replace(&extracted_source, ""))?
            }
        }

        remove_dir_all(extracted_source)?;

        println!("\n    Building...");

        package.build()?.write(File::create(output_package_path)?)?;
    }

    println!("\nDone! Converted \"{source_package_path}\" -> \"{output_package_path}!\"");

    Ok(())
}
