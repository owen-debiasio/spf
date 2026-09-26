//! Functions related to and providing the ability to convert supported packages.
//! - `.spf` -> `.deb`
//! - `.deb` -> `.spf`
//! - `.rpm` -> `.spf`
//! - `.spf` -> `.rpm`
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    ffi::OsStr,
    fs::{File, remove_dir_all, remove_file, rename},
    io::{Error, Write, stdin, stdout},
    os::unix::ffi::OsStrExt,
    path::Path,
    process::{Command, exit},
};

use cmd_exists::cmd_exists;
use deb_rs2::file::Deb;
use glob::glob;

use deb_rust::{DebArchitecture, DebFile, binary::DebPackage};
use rpm::{PackageBuilder, PackageMetadata};

use crate::{
    VERSION,
    fs::{ArchiveType, FileProperty, create_archive, extract_archive},
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

#[derive(Clone, Debug)]
pub enum PackageType {
    Spf,
    Deb,
    Rpm,
}

#[derive(Clone, Debug)]
struct Package {
    source_package_path: String,
    package_type: PackageType,
}

impl Package {
    pub fn from(package_path: String) -> Result<Package, Error> {
        let source_package_ext = FileProperty::extension(&package_path)?;

        let source_package_type = match source_package_ext.as_ref() {
            "spf" => PackageType::Spf,
            "deb" => PackageType::Deb,
            "rpm" => PackageType::Rpm,
            _ => error(&format!(
                "Invalid source package type: {source_package_ext:?}"
            )),
        };

        Ok(Package {
            source_package_path: package_path,
            package_type: source_package_type,
        })
    }

    fn load_metadata(&self) -> Result<Categories, Error> {
        let package_path = self.source_package_path.clone();

        let returned_meta = match self.package_type {
            // Get spf package metadata
            PackageType::Spf => {
                if !package_path.ends_with(".spf") {
                    panic!("must be .spf file")
                }

                extract_archive(&package_path, ".", ArchiveType::Tar)?;

                let metadata_path = &format!(
                    "{}/META",
                    FileProperty::name(&package_path)?.trim_end_matches(".spf")
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
                let loaded_metadata = Deb::new(package_path).extract()?.retrieve_control()?;

                let unknown = String::from("Unknown (converted from Debian package)");

                Categories {
                    name: loaded_metadata.package,
                    version: loaded_metadata.version,
                    description: loaded_metadata
                        .description
                        .split(" #")
                        .next()
                        .unwrap_or(&unknown)
                        .to_string(),
                    source: loaded_metadata.homepage.unwrap_or(unknown.clone()),
                    license: unknown, // There is no license field for Debian packages, so return an empty string.
                    authors: loaded_metadata.maintainer,
                    arch: loaded_metadata.architecture,
                }
            }

            PackageType::Rpm => {
                let package = PackageMetadata::open(package_path)
                    .unwrap_or_else(|err| error(&format!("Failed to open rpm package: {err}")));

                Categories {
                    name: package
                        .get_name()
                        .unwrap_or("Unknown (likely lost in conversion)")
                        .to_string(),
                    version: package
                        .get_version()
                        .unwrap_or("Unknown (likely lost in conversion)")
                        .to_string(),
                    description: package
                        .get_description()
                        .unwrap_or("Unknown (likely lost in conversion)")
                        .to_string(),
                    source: package
                        .get_vendor()
                        .unwrap_or("Unknown (likely lost in conversion)")
                        .to_string(),
                    license: package
                        .get_license()
                        .unwrap_or("Unknown (likely lost in conversion)")
                        .to_string(),
                    authors: package
                        .get_packager()
                        .unwrap_or("Unknown (likely lost in conversion)")
                        .to_string(),
                    arch: package.get_arch().unwrap_or("noarch").to_string(),
                }
            }
        };

        Ok(returned_meta)
    }

    pub fn convert(
        self,
        output_package_type: PackageType,
        output_location: &str,
    ) -> Result<(), Error> {
        println!("    Loading package...");

        let metadata = self.clone().load_metadata()?;
        let source_package_path = &self.source_package_path;

        match self.package_type {
            PackageType::Spf => {
                println!("        Extracting...");
                extract_archive(source_package_path, ".", ArchiveType::Tar)?;

                let source_name = FileProperty::name(source_package_path)?;

                let extracted_source = source_name.trim_end_matches(".spf");

                remove_file(format!("{extracted_source}/META"))?;

                match output_package_type {
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

                            let destination = &current_path
                                .trim_start_matches(source_package_path)
                                .to_string();

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
                    PackageType::Rpm => {
                        println!("    Setting metadata...");
                        let hostname = hostname::get()
                            .unwrap_or(OsStr::from_bytes("Unknown".as_bytes()).to_owned())
                            .display()
                            .to_string();

                        let build_host = &format!("SPF package converter @ {hostname}");

                        let mut package_details = PackageBuilder::new(
                            &metadata.name,
                            &metadata.version,
                            &metadata.license,
                            convert_arch(&metadata.arch, output_package_type)?,
                            &metadata.description,
                        );

                        let package = package_details.build_host(build_host);

                        let direct_source_path = FileProperty::name(source_package_path)?;
                        let extracted_spf = direct_source_path.trim_end_matches(".spf");

                        for path in
                            glob(&format!("{extracted_spf}/**/*")).expect("Failed to find paths")
                        {
                            let path = path?.display().to_string();
                            let destination = path.trim_start_matches(extracted_spf);

                            print!("\r\x1B[K    Writing path: \"{path}\"");
                            stdout().flush()?;

                            if Path::new(&path).is_file() {
                                package
                                    .with_file(
                                        &path,
                                        rpm::FileOptions::new(
                                            path.trim_start_matches(extracted_spf),
                                        )
                                        .config()
                                        .noreplace(),
                                    )
                                    .unwrap_or_else(|_| error("no"));
                            } else {
                                package
                                    .with_dir(path.clone(), destination, |path| path.config())
                                    .unwrap_or_else(|err| {
                                        error(&format!("Failed to copy directory: {err}"))
                                    });
                            }
                        }

                        println!("\n    Building...");

                        package
                            .build()
                            .unwrap_or_else(|err| {
                                error(&format!("Failed to convert from rpm: {err}"))
                            })
                            .write_to(output_location)
                            .unwrap_or_else(|err| {
                                error(&format!("Failed to write output spf file from rpm: {err}"))
                            });

                        remove_dir_all(extracted_spf)?;
                    }
                }
            }
            PackageType::Deb => match output_package_type {
                // deb -> spf
                PackageType::Spf => {
                    println!("        Extracting...");

                    extract_archive(source_package_path, ".", ArchiveType::Ar)?;

                    println!(
                        "    Converting package files...\n        Removing unused data files..."
                    );

                    println!("            Removing \"./debian-binary\"...");
                    remove_file("debian-binary")?;

                    println!("            Removing \"./control.tar.xz\"...");
                    remove_file("control.tar.xz")?;

                    println!("        Extracting \"./data.tar.xz\"...");
                    extract_archive("data.tar.xz", "./data", ArchiveType::Xz)?;

                    println!("            Removing \"./data.tar.xz\"...");
                    remove_file("data.tar.xz")?;

                    let output_location_name = FileProperty::name(output_location)?;
                    let new_package_dir = output_location_name
                        .split('.')
                        .next()
                        .expect("Expected file name");

                    rename("data", new_package_dir)?;

                    println!("    Converting metadata...");

                    println!("        Generating...");

                    // Convert arch to spf equivelent

                    let metadata_file_contents: Vec<String> = vec![
                        format!("### CONVERTED & PACKAGED WITH SPF {VERSION} ###\n"),
                        format!("PROJECT_NAME = {}", metadata.name),
                        format!("VERSION = {}", metadata.version),
                        format!("DESCRIPTION = {}", metadata.description),
                        format!("REPOSITORY = {}", metadata.source),
                        format!("LICENSE = {}", metadata.license),
                        format!("AUTHORS = {}", metadata.authors),
                        format!(
                            "ARCH = {}",
                            convert_arch(&metadata.arch, output_package_type)?
                        ),
                    ];

                    println!("        Writing...");

                    let mut new_meta_file = File::create(format!("{new_package_dir}/META"))?;
                    new_meta_file.write_all(metadata_file_contents.join("\n").as_bytes())?;

                    println!("    Packaging...");

                    create_archive(output_location, new_package_dir, ArchiveType::Tar)?;

                    remove_dir_all(new_package_dir)?;
                }
                // deb -> deb
                PackageType::Deb => error("You cannot convert a .deb package to a .deb package!"),
                // deb -> rpm
                PackageType::Rpm => error("You cannot convert a .deb package to a .rpm package!"),
            },
            PackageType::Rpm => match output_package_type {
                // rpm -> spf
                PackageType::Spf => {
                    cmd_exists("rpm2archive")
                        .unwrap_or_else(|err| error(&format!("Cannot run \"rpm2cpio\": {err}")));

                    println!("        Extracting...");

                    Command::new("rpm2archive")
                        .arg(source_package_path)
                        .status()?;

                    let tgz_file = &format!("{source_package_path}.tgz");

                    let extract_dest = &format!("./{}", FileProperty::name(tgz_file)?);
                    let extract_dest_clean = &extract_dest.trim_end_matches(".rpm.tgz").to_string();

                    extract_archive(tgz_file, extract_dest_clean, ArchiveType::Gz)?;

                    remove_file(tgz_file)?;

                    println!("    Converting metadata...");

                    println!("        Collecting...");

                    let metadata_file_contents: Vec<String> = vec![
                        format!("### CONVERTED & PACKAGED WITH SPF {VERSION} ###\n"),
                        format!("PROJECT_NAME = {}", metadata.name),
                        format!("VERSION = {}", metadata.version),
                        format!("DESCRIPTION = {}", metadata.description),
                        format!("REPOSITORY = {}", metadata.source),
                        format!("LICENSE = {}", metadata.license),
                        format!("AUTHORS = {}", metadata.authors),
                        format!(
                            "ARCH = {}",
                            convert_arch(&metadata.arch, output_package_type)?
                        ),
                    ];

                    println!("        Writing...");

                    let mut new_meta_file = File::create(format!("{extract_dest_clean}/META"))?;
                    new_meta_file.write_all(metadata_file_contents.join("\n").as_bytes())?;

                    println!("    Packaging...");

                    let new_extracted_folder_name = output_location.trim_end_matches(".spf");

                    rename(extract_dest_clean, new_extracted_folder_name)?;

                    create_archive(output_location, new_extracted_folder_name, ArchiveType::Tar)?;

                    remove_dir_all(new_extracted_folder_name)?;
                }
                // rpm -> deb
                PackageType::Deb => error("You cannot convert a .rpm package to a .deb package!"),
                // rpm -> rpm
                PackageType::Rpm => error("You cannot convert a .rpm package to a .rpm package!"),
            },
        }

        Ok(())
    }
}

pub fn convert_arch(
    input_arch: &str,
    output_package_type: PackageType,
) -> Result<&'static str, Error> {
    let arch_error = || {
        error(&format!(
            "Failed to convert arch \"{input_arch}\" to \"{output_package_type:?}\" equivalent"
        ))
    };

    let arch = match output_package_type {
        PackageType::Spf => match input_arch {
            "all" | "noarch" | "src" | "nosrc" => "universal",
            "amd64" | "x86_64" => "x86_64",
            "i386" | "i686" => "x86",
            "arm64" | "aarch64" => "aarch64",
            "armhf" | "armv7hl" | "armvhl" => "arm",
            _ => arch_error(),
        },
        PackageType::Deb => match input_arch {
            "universal" | "noarch" | "src" | "nosrc" => "all",
            "x86_64" => "amd64",
            "x86" | "i386" | "i686" => "i386",
            "aarch64" => "arm64",
            "arm" | "armv7hl" | "armvhl" => "armhf",
            _ => arch_error(),
        },
        PackageType::Rpm => match input_arch {
            "universal" | "all" => "noarch",
            "x86_64" => "amd64",
            "x86" | "i386" => "i686",
            "aarch64" | "arm64" => "aarch64",
            "arm" | "armhf" => "armv7hl",
            _ => arch_error(),
        },
    };

    Ok(arch)
}
