//! Functions related to and providing the ability to convert a `.spf`
//! package to a `.deb` package (and vice versa).
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{File, remove_dir_all, remove_file},
    io::{Error, Write, stdin, stdout},
    path::Path,
    process::exit,
};

use glob::glob;

use deb_rust::{DebArchitecture, DebFile, binary::DebPackage};
use rpm::{FileOptions, PackageBuilder, PackageMetadata};

use crate::{
    fs::{FileProperty, extract_tar_archive},
    metadata::{Categories, Meta},
    sys::error,
};

/// Lets user know that this program has no warranty, and is not responsible
fn disclaimer() -> Result<(), Error> {
    println!(
        "I, or this program, are not responsible for any damage to your system caused by this command.\n\
        Install converted packages at your own risk.\n\n\
        Press enter to continue..."
    );

    stdin().read_line(&mut String::new())?;

    Ok(())
}

/// Checks if the input/output packages are supported.
///
/// It specifically checks if:
/// - Package formats are supported
/// - User tries to convert a package to the same format
/// - User tries to convert `.deb` packages to `rpm` packages (and vice versa)
fn verify_input_paths(source_package_path: &str, output_package_path: &str) -> String {
    let source_file_ext = match FileProperty::extension(source_package_path) {
        Ok(ext) => ext,
        Err(err) => error(&format!("Failed to get source file extension: {err}")),
    };

    let output_file_ext = match FileProperty::extension(output_package_path) {
        Ok(ext) => ext,
        Err(err) => error(&format!("Failed to get output file extension: {err}")),
    };

    // Verify supported package formats
    if !matches!(source_file_ext.as_str(), "spf" | "deb" | "rpm") {
        error(&format!(
            "Unsupported input package format: .{source_file_ext}"
        ))
    } else if !matches!(output_file_ext.as_str(), "spf" | "deb" | "rpm") {
        error(&format!(
            "Unsupported output package format: {output_file_ext}"
        ))
    }

    if output_package_path.is_empty() {
        error("Please provide an output package path!")
    }

    // Makes sure the use doesn't try to convert 2 of the same package types
    if source_file_ext == output_file_ext {
        error("Source and output packages must not be the same!")
    }

    // Make sure user doesn't attempt to convert `.deb` -> `.rpm` (and vice versa)
    if (source_file_ext == "rpm" && output_file_ext == "deb")
        || (source_file_ext == "deb" && output_file_ext == "rpm")
    {
        error("\".deb\" and \".rpm\" files can not be converted back and forth!")
    }

    output_file_ext
}

pub fn convert(source_package_path: &str, output_package_path: &str) -> Result<(), Error> {
    disclaimer()?;

    if source_package_path.is_empty() {
        error("Please provide an input package path!")
    }

    let source_file = Path::new(source_package_path);

    if !source_file.exists() {
        error(&format!("File \"{source_package_path}\" does not exist!"))
    }

    let output_extension = verify_input_paths(source_package_path, output_package_path);

    println!("Converting \"{source_package_path}\" -> \"{output_package_path}\"...");

    let output_package_type = match output_extension.as_str() {
        "spf" => PackageType::Spf,
        "deb" => PackageType::Deb,
        "rpm" => PackageType::Rpm,
        _ => {
            error("Failed to determine output package type? idk bro this probably shouldn't happen")
        }
    };

    Package::from(source_package_path.to_string())?
        .convert(output_package_type, output_package_path)?;

    println!("\nDone! Converted \"{source_package_path}\" -> \"{output_package_path}\"!");

    exit(0)
}

// /// Follows a series of steps in order to convert a `.deb` file to `.spf`.
// fn convert_to_spf(output_package_path: &str) -> Result<(), Error> {
//     println!("        Extracting \"control.tar.xz\"...");

//     extract_tar_archive("./control.tar.xz", "./control", "xz")?;

//     remove_file("control.tar.xz")?;

//     println!("        Reading \"control\"...");

//     // `control.tar.xz` contains the debian package metadata, so extract if applicable.
//     // Otherwise, extract assumed location of `.spf` package metadata, then collect the
//     // metadata.
//     let mut source_metadata: String = fs::read_to_string("control/control")?;

//     println!("        Cleaning up metadata collection...");

//     remove_dir_all("control")?;

//     /* Conversion of metadata */
//     println!("    Converting metadata...");

//     let cloned_metadata = source_metadata.clone();
//     let metadata_lines: Vec<&str> = cloned_metadata.lines().collect();

//     // Go through and replace the category headers with the .spf META file
//     // counterparts.
//     for line in metadata_lines {
//         if line.starts_with("Package:") {
//             source_metadata = source_metadata.replace("Package:", "PROJECT_NAME =");
//         } else if line.starts_with("Version:") {
//             source_metadata = source_metadata.replace("Version:", "VERSION =");
//         } else if line.starts_with("Description:") {
//             source_metadata = source_metadata.replace("Description:", "DESCRIPTION =");
//         } else if line.starts_with("Homepage:") {
//             source_metadata = source_metadata.replace("Homepage:", "REPOSITORY =");
//         } else if line.starts_with("Maintainer:") {
//             source_metadata = source_metadata.replace("Maintainer:", "AUTHORS =");
//         } else if line.starts_with("Architecture:") {
//             let deb_arch = line.replace("Architecture: ", "");
//             let spf_arch = convert_arch(&deb_arch, true)?;

//             println!("        Converted architecture \"{deb_arch}\" -> \"{spf_arch}\"");

//             source_metadata = source_metadata.replace(line, &format!("ARCH = {spf_arch}"));
//         } else {
//             // If nothing matches, comment the lines. Will be removed after.
//             source_metadata = source_metadata.replace(&format!("{line}\n"), &format!("#{line}\n"));
//         };
//     }

//     // Removes the lines that have been commented.
//     source_metadata = source_metadata
//         .lines()
//         .filter(|line| !line.starts_with("#"))
//         .map(|line| line.trim_end_matches("#"))
//         .collect::<Vec<_>>()
//         .join("\n");

//     println!("    Extracting data (this might take a while)...");

//     extract_tar_archive("data.tar.xz", "./data", "xz")?;

//     remove_file("data.tar.xz")?;

//     let new_spf_dest = output_package_path.replace(".spf", "");
//     rename("data", &new_spf_dest)?;

//     fs::write(format!("{new_spf_dest}/META"), source_metadata)?;

//     println!("    Packaging...");

//     create_tar_archive(output_package_path, &new_spf_dest, "")?;

//     remove_dir_all(new_spf_dest)?;

//     Ok(())
// }

#[derive(Clone, Debug)]
enum PackageType {
    Spf,
    Deb,
    Rpm,
}

struct Package {
    source_package_path: String,
}

impl Package {
    pub fn from(package_path: String) -> Result<Package, Error> {
        Ok(Package {
            source_package_path: package_path,
        })
    }

    fn load_package_metadata(
        package_type: PackageType,
        package_path: &str,
    ) -> Result<Categories, Error> {
        let returned_meta = match package_type {
            // Get spf package metadata
            PackageType::Spf => {
                if !package_path.ends_with(".spf") {
                    panic!("must be .spf file")
                }

                extract_tar_archive(package_path, ".", "")?;

                let metadata_path = &format!(
                    "{}/META",
                    FileProperty::name(package_path)?.trim_end_matches(".spf")
                );

                let package = Meta::from(metadata_path)?;

                Categories {
                    name: package.clone().load_value("PROJECT_NAME")?,
                    version: package.clone().load_value("VERSION")?,
                    description: package.clone().load_value("DESCRIPTION")?,
                    source: package.clone().load_value("REPOSITORY")?,
                    license: package.clone().load_value("LICENSE")?,
                    authors: package.clone().load_value("AUTHORS")?,
                    arch: package.load_value("ARCH")?,
                }
            }

            // Get Debian package metadata
            PackageType::Deb => {
                let package = DebPackage::from(File::open(package_path)?)?;

                Categories {
                    name: package.name().to_string(),
                    version: package.version().to_string(),
                    description: package.description().to_string(),
                    source: package.homepage().to_string(),
                    license: String::new(), // There is no license field for Debian packages, so return an empty string.
                    authors: package.maintainer().to_string(),
                    arch: package.architecture().as_str().to_string(),
                }
            }

            PackageType::Rpm => {
                let package = PackageMetadata::open(package_path)
                    .unwrap_or_else(|err| error(&format!("Failed to open rpm package: {err}")));

                Categories {
                    name: package.get_name().unwrap_or_default().to_string(),
                    version: package.get_version().unwrap_or_default().to_string(),
                    description: package.get_description().unwrap_or_default().to_string(),
                    source: package.get_vendor().unwrap_or_default().to_string(),
                    license: package.get_license().unwrap_or_default().to_string(),
                    authors: package.get_packager().unwrap_or_default().to_string(),
                    arch: package.get_arch().unwrap_or_default().to_string(),
                }
            }
        };

        Ok(returned_meta)
    }

    pub fn convert(self, package_type: PackageType, output_location: &str) -> Result<(), Error> {
        let source_package = &self.source_package_path;

        let source_package_type = match FileProperty::extension(source_package)?.as_ref() {
            "spf" => PackageType::Spf,
            "deb" => PackageType::Deb,
            "rpm" => PackageType::Rpm,
            _ => error("no"),
        };

        println!("    Collecting source package metadata...");
        let metadata = Self::load_package_metadata(source_package_type.clone(), source_package)?;

        match source_package_type {
            PackageType::Spf => {
                println!("    Extracting...");
                extract_tar_archive(source_package, ".", "")?;

                let source_name = FileProperty::name(source_package)?;

                let extracted_source = source_name.trim_end_matches(".spf");

                remove_file(format!("{extracted_source}/META"))?;

                match package_type {
                    PackageType::Spf => {
                        error("You cannot convert a .spf package to a .spf package!")
                    }
                    // spf -> deb
                    PackageType::Deb => {
                        let mut package = DebPackage::new(&metadata.name);

                        let package_arch = metadata.arch;

                        // Convert the architectures to the `.deb` counterparts
                        let arch_to_use = match package_arch.as_ref() {
                            "universal" => DebArchitecture::All,
                            "x86_64" => DebArchitecture::Amd64,
                            "x86" => DebArchitecture::I386,
                            "aarch64" => DebArchitecture::Arm64,
                            "arm" => DebArchitecture::Armhf,
                            _ => error("Failed converting arch to .deb equivalent"),
                        };

                        println!("        Applying...");

                        // Set the metadata
                        package = package
                            .set_name(&metadata.name)
                            .set_version(&metadata.version)
                            .set_description(&metadata.description)
                            .set_maintainer(&metadata.authors)
                            .set_homepage(&metadata.source)
                            .set_architecture(arch_to_use);

                        println!("    Collecting paths...");

                        // Collect paths and add them to the package
                        for path in
                            glob(&format!("{extracted_source}/**/*")).expect("Failed to get paths")
                        {
                            let current_path = path?.display().to_string();

                            print!("\r\x1B[K        Writing path: \"{current_path}\"");
                            stdout().flush()?;

                            let destination =
                                &current_path.trim_start_matches(source_package).to_string();

                            // Adds the paths. Varies depending on if the path is a file or directory.
                            package = if Path::new(&current_path).is_file() {
                                package.with_file(DebFile::from_path(&current_path, destination)?)
                            } else {
                                package.with_dir(&current_path, destination)?
                            }
                        }

                        println!("\n    Building...");

                        package.build()?.write(File::create(output_location)?)?;

                        remove_dir_all(extracted_source)?;
                    }
                    // spf -> rpm
                    PackageType::Rpm => todo!(),
                }
            }
            PackageType::Deb => match source_package_type {
                // deb -> spf
                PackageType::Spf => todo!(),
                // deb -> deb
                PackageType::Deb => error("You cannot convert a .deb package to a .deb package!"),
                // deb -> rpm
                PackageType::Rpm => error("You cannot convert a .deb package to a .rpm package!"),
            },
            PackageType::Rpm => match source_package_type {
                // rpm -> spf
                PackageType::Spf => todo!(),
                // rpm -> deb
                PackageType::Deb => error("You cannot convert a .rpm package to a .deb package!"),
                // rpm -> rpm
                PackageType::Rpm => error("You cannot convert a .rpm package to a .rpm package!"),
            },
        }

        Ok(())
    }
}
