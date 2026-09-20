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

    let source_file_name = match FileProperty::name(source_package_path) {
        Ok(name) => name,
        Err(err) => error(&format!("Failed to get source name: {err}")),
    };

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

    if source_file_ext == output_file_ext {
        error("Source and output packages may not be the same!")
    }

    disclaimer()?;

    println!("Converting \"{source_package_path}\" -> \"{output_package_path}\"...");
    println!("    Extracting \"{source_package_path}\"...");

    let source_is_debian = source_file_ext == "deb";

    // `debian-binary` isn't used, so delete it
    if source_is_debian {
        extract_ar_archive(source_package_path, ".")?;
        remove_file("debian-binary")?;
    } else {
        extract_tar_archive(source_package_path, ".", "")?;
    }

    let spf_metadata_path = source_file_name.replace(".spf", "/META");

    if source_is_debian {
        convert_to_spf(output_package_path)?;
    } else {
        convert_to_deb(spf_metadata_path, source_file_name, output_package_path)?;
    }

    println!("\nDone! Converted \"{source_package_path}\" -> \"{output_package_path}\"!");

    Ok(())
}

/// Follows a series of steps in order to convert a `.deb` file to `.spf`.
fn convert_to_spf(output_package_path: &str) -> Result<(), std::io::Error> {
    println!("        Extracting \"control.tar.xz\"...");

    extract_tar_archive("./control.tar.xz", "./control", "xz")?;

    remove_file("control.tar.xz")?;

    println!("        Reading \"control\"...");

    // `control.tar.xz` contains the debian package metadata, so extract if applicable.
    // Otherwise, extract assumed location of `.spf` package metadata, then collect the
    // metadata.
    let mut source_metadata: String = fs::read_to_string("control/control")?;

    println!("        Cleaning up metadata collection...");

    remove_dir_all("control")?;

    /* Conversion of metadata */

    println!("    Converting metadata...");

    let cloned_metadata = source_metadata.clone();
    let metadata_lines: Vec<&str> = cloned_metadata.lines().collect();

    // Go through and replace the category headers with the .spf META file
    // counterparts.
    for line in metadata_lines {
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
            let deb_arch = line.replace("Architecture: ", "");
            let spf_arch = convert_arch(&deb_arch, true)?;

            println!("        Converted architecture \"{deb_arch}\" -> \"{spf_arch}\"");

            source_metadata = source_metadata.replace(line, &format!("ARCH = {spf_arch}"));
        } else {
            // If nothing matches, comment the lines. Will be removed after.
            source_metadata = source_metadata.replace(&format!("{line}\n"), &format!("#{line}\n"));
        };
    }

    // Removes the lines that have been commented.
    source_metadata = source_metadata
        .lines()
        .filter(|line| !line.starts_with("#"))
        .map(|line| line.trim_end_matches("#"))
        .collect::<Vec<_>>()
        .join("\n");

    println!("    Extracting data (this might take a while)...");

    extract_tar_archive("data.tar.xz", "./data", "xz")?;

    remove_file("data.tar.xz")?;

    let new_spf_dest = output_package_path.replace(".spf", "");
    rename("data", &new_spf_dest)?;

    fs::write(format!("{new_spf_dest}/META"), source_metadata)?;

    println!("    Packaging...");

    create_tar_archive(output_package_path, &new_spf_dest, "")?;

    remove_dir_all(new_spf_dest)?;

    Ok(())
}

/// Follows a series of steps in order to convert a `.spf` file to `.deb`.
///
/// Collects the metadata, copies paths, then add them to the output .deb
/// file.
fn convert_to_deb(
    spf_metadata_path: String,
    source_file_name: String,
    output_package_path: &str,
) -> Result<(), std::io::Error> {
    let extracted_source = source_file_name.replace(".spf", "");

    println!("    Packaging...");
    println!("        Loading package metadata...");

    let package_metadata = Meta::from(&spf_metadata_path)?;

    // Name
    let name = package_metadata.clone().load_value("PROJECT_NAME")?;
    println!("            Collected package name: \"{name}\"");

    // Version
    let version = package_metadata.clone().load_value("VERSION")?;
    println!("            Collected package version: \"{version}\"");

    // Description
    let desc = package_metadata.clone().load_value("DESCRIPTION")?;
    println!("            Collected package description: \"{desc}\"");

    // Maintainer
    let maintainer = package_metadata.clone().load_value("AUTHORS")?;
    println!("            Collected package maintainer(s): \"{maintainer}\"");

    // Homepage
    let homepage = package_metadata.clone().load_value("REPOSITORY")?;
    println!("            Collected package homepage: \"{homepage}\"");

    // Architecture
    let arch = package_metadata.load_value("ARCH")?;
    println!("            Collected package architecture: \"{arch}\"");

    remove_file(spf_metadata_path)?;

    let mut package = DebPackage::new(&name);

    // Convert the architectures to the `.deb` counterparts
    let arch_to_use = match arch.as_str() {
        "universal" => DebArchitecture::All,
        "x86_64" => DebArchitecture::Amd64,
        "x86" => DebArchitecture::I386,
        "aarch64" => DebArchitecture::Arm64,
        "arm" => DebArchitecture::Armhf,
        _ => error(&format!(
            "Failed converting arch \"{arch}\" to .deb equivalent"
        )),
    };

    // Set the metadata
    package = package
        .set_name(&name)
        .set_version(&version)
        .set_description(&desc)
        .set_maintainer(&maintainer)
        .set_homepage(&homepage)
        .set_architecture(arch_to_use);

    println!("        Writing paths...");

    // Goes through and adds all the paths to add
    for path in glob(&format!("{extracted_source}/**/*")).expect("Failed to get paths") {
        let current_path = path?.display().to_string();

        print!("\r\x1B[K            Writing path: \"{current_path}\"");
        stdout().flush()?;

        // Adds the paths. Varies depending on if the path is a file or directory.
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

    Ok(())
}

/// Converts a `.spf` arch to `.deb` arch, and vice versa.
///
/// Determined with `is_debian` ([bool]), and converts `arch` ([`str`])
///
/// Convert to `.spf`:
/// ```
/// let is_debian = true;
/// let deb_arch = "amd64";
///
/// let arch = convert_arch(deb_arch, is_debian)
///
/// assert_eq!("x86_64", arch)
/// ```
/// Convert to `.deb`:
/// ```
/// let is_debian = false;
/// let deb_arch = "x86_64";
///
/// let arch = convert_arch(deb_arch, is_debian)
///
/// assert_eq!("arm64", arch)
/// ```
pub fn convert_arch(arch: &str, is_debian: bool) -> Result<&'static str, std::io::Error> {
    let arch = if is_debian {
        // Convert to .spf
        match arch {
            "all" => "universal",
            "amd64" => "x86_64",
            "i386" => "x86",
            "arm64" => "aarch64",
            "armhf" => "arm",
            _ => error(&format!(
                "Failed to convert arch \"{arch}\" to .spf equivalent"
            )),
        }
    } else {
        // Convert to .deb
        match arch {
            "universal" => "all",
            "x86_64" => "amd64",
            "x86" => "i386",
            "aarch64" => "arm64",
            "arm" => "armhf",
            _ => error(&format!(
                "Failed to convert arch \"{arch}\" to .deb equivalent"
            )),
        }
    };

    Ok(arch)
}
