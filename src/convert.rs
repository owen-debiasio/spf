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
use rpm::{FileOptions, PackageBuilder, PackageMetadata};

use crate::{
    fs::{ArchiveType, FileProperty, create_archive, extract_archive},
    metadata::{Categories, Meta},
    sys::{convert_arch, error},
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
fn verify_input_paths(source_package_path: &str, output_package_path: &str) -> Result<(), Error> {
    let source_file_ext = match FileProperty::extension(source_package_path) {
        Ok(ext) => ext,
        Err(err) => error(&format!("Failed to get source file extension: {err}")),
    };

    let output_file_ext = match FileProperty::extension(output_package_path) {
        Ok(ext) => ext,
        Err(err) => error(&format!("Failed to get output file extension: {err}")),
    };

    // Verify supported package formats
    if !matches!(
        source_file_ext.as_str(),
        PackageType::SPF | PackageType::DEB | PackageType::RPM
    ) {
        error(&format!(
            "Unsupported input package format: .{source_file_ext}"
        ))
    } else if !matches!(
        output_file_ext.as_str(),
        PackageType::SPF | PackageType::DEB | PackageType::RPM
    ) {
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
    if (source_file_ext == PackageType::RPM && output_file_ext == PackageType::DEB)
        || (source_file_ext == PackageType::DEB && output_file_ext == PackageType::RPM)
    {
        error("\".deb\" and \".rpm\" files can not be converted back and forth!")
    }

    Ok(())
}

/// Convert `.spf`, `.deb`, and `.rpm` packages to and from each other.
///
/// Example: Converting `.spf` to `.deb`:
/// ```
/// let package_to_convert = "sample.spf";
/// let output_package = "output.deb";
///
/// convert(package_to_convert, output_package)?;
/// ```
pub fn convert(source_package_path: &str, output_package_path: &str) -> Result<(), Error> {
    if source_package_path.is_empty() {
        error("Please provide an input package path!")
    }

    let source_file = Path::new(source_package_path);

    if !source_file.exists() {
        error(&format!("File \"{source_package_path}\" does not exist!"))
    }

    verify_input_paths(source_package_path, output_package_path)?;

    disclaimer()?;

    println!("Converting \"{source_package_path}\" -> \"{output_package_path}\"...");

    let output_package_type = PackageType::get_from_extension(output_package_path)?;

    Converter::from(source_package_path.to_string())?
        .convert(output_package_type, output_package_path)?;

    println!("\nDone! Converted \"{source_package_path}\" -> \"{output_package_path}\"!");

    exit(0)
}

/// Identifies packages
///
/// Entries correspond with what they mean
///
/// - [`PackageType::get_from_extension`]:
///   Retrieves the package type from the package extension.
///
///   Returns as [`PackageType`]
///
///   ```
///   let package_path = "test.spf";
///   let package_type_from_package = PackageType::get_from_extension(package_path)?;
///
///   assert_eq!(package_type_from_package, PackageType::Spf);
///   ```
#[derive(Clone, Debug)]
pub enum PackageType {
    /// `.spf` package
    Spf,

    /// `.deb` package
    Deb,

    /// `.rpm` package
    Rpm,
}

impl PackageType {
    /// Package type `spf` as [`str`]
    pub const SPF: &str = "spf";

    /// Package type `deb` as [`str`]
    pub const DEB: &str = "deb";

    /// Package type `rpm` as [`str`]
    pub const RPM: &str = "rpm";

    /// Retrieves the package type from the package extension.
    ///
    /// Returns as [`PackageType`]
    ///
    /// ```
    /// let package_path = "test.spf";
    /// let package_type_from_package = PackageType::get_from_extension(package_path)?;
    ///
    /// assert_eq!(package_type_from_package, PackageType::Spf);
    /// ```
    pub fn get_from_extension(path_of_package: &str) -> Result<PackageType, Error> {
        let source_package_ext = FileProperty::extension(path_of_package)?;

        let package_type = match source_package_ext.as_ref() {
            Self::SPF => Self::Spf,
            Self::DEB => Self::Deb,
            Self::RPM => Self::Rpm,
            _ => error(&format!("Invalid package type: {source_package_ext}")),
        };

        Ok(package_type)
    }
}

/// The core converter functionality.
///
/// - [Converter::from]:
///
///   Loads the package that you want to convert.
///
///   The package to load (`package_path`) is an input as [String].
///
///   ```
///   let package_path = String::from("sample_package.spf");
///   let package_to_convert = Converter::from(package_path)?;
///   ```
///
/// - [Converter::load_source_metadata]:
///
///   Loads the metadata from the loaded package. The metadata is loaded using the struct
///   [Categories].
///
///   Example: Loading metadata from `.deb` package:
///   ```
///   let package_path = String::from("sample_package.deb");
///   let loaded_package = Converter::from(package_path)?;
///
///   let loaded_metadata = loaded_package.load_source_metadata()?;
///   ```
///   Example: Pulling individual values from loaded metadata:
///   ```
///   let package_name = loaded_metadata.name;
///   assert_eq(package_name, "sample_package");
///   ```
/// - [Converter::convert]
///   The actual conversion process.
///
///   Example: Converting `.rpm` -> `.spf`
///   ```
///   let package_path = String::from("sample_package.rpm");
///   let loaded_package = Converter::from(package_path)?;
///
///   let output_package_path = String::from("sample_package.spf");
///   let output_package_type = PackageType::Spf;
///
///   loaded_package.convert(output_package_type, output_package_path)?;
///   ```
#[derive(Clone, Debug)]
struct Converter {
    /// The path of the package to be converted
    source_package_path: String,

    /// The type of package to convert to or from
    package_type: PackageType,
}

impl Converter {
    /// Loads the package that you want to convert.
    ///
    /// The package to load (`package_path`) is an input as [String].
    ///
    /// ```
    /// let package_path = String::from("sample_package.spf");
    /// let package_to_convert = Converter::from(package_path)?;
    /// ```
    pub fn from(package_path: String) -> Result<Converter, Error> {
        let source_package_type = PackageType::get_from_extension(&package_path)?;

        Ok(Converter {
            source_package_path: package_path,
            package_type: source_package_type,
        })
    }

    /// Loads the metadata from the loaded package. The metadata is loaded using the struct
    /// [Categories].
    ///
    /// Example: Loading metadata from `.deb` package:
    /// ```
    /// let package_path = String::from("sample_package.deb");
    /// let loaded_package = Converter::from(package_path)?;
    ///
    /// let loaded_metadata = loaded_package.load_source_metadata()?;
    /// ```
    /// Example: Pulling individual values from loaded metadata:
    /// ```
    /// let package_name = loaded_metadata.name;
    /// assert_eq(package_name, "sample_package");
    /// ```
    fn load_source_metadata(&self) -> Result<Categories, Error> {
        let package_path = self.source_package_path.clone();

        let returned_meta = match self.package_type {
            // Get spf package metadata
            PackageType::Spf => {
                if !package_path.ends_with(".spf") {
                    error("must be .spf file")
                }

                extract_archive(&package_path, ".", ArchiveType::Tar)?;

                let metadata_path = &format!(
                    "{}/META",
                    FileProperty::name(&package_path)?.trim_end_matches(".spf")
                );

                let package = Meta::from(metadata_path)?;

                Categories {
                    name: package.load_value("PROJECT_NAME")?,
                    version: package.load_value("VERSION")?,
                    description: package.load_value("DESCRIPTION")?,
                    source: package.load_value("REPOSITORY")?,
                    license: package.load_value("LICENSE")?,
                    authors: package.load_value("AUTHORS")?,
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
                    source: loaded_metadata.homepage.unwrap_or(String::from("Unknown")),
                    license: String::from("Not Applicable (converted using spf)"),
                    authors: loaded_metadata.maintainer,
                    arch: loaded_metadata.architecture,
                }
            }

            PackageType::Rpm => {
                let package = PackageMetadata::open(package_path)
                    .unwrap_or_else(|err| error(&format!("Failed to open rpm package: {err}")));
                const UNKNOWN: &str = "Unknown (likely lost in conversion)";

                Categories {
                    name: package.get_name().unwrap_or(UNKNOWN).to_string(),
                    version: package.get_version().unwrap_or(UNKNOWN).to_string(),
                    description: package.get_description().unwrap_or(UNKNOWN).to_string(),
                    source: package.get_url().unwrap_or(UNKNOWN).to_string(),
                    license: package.get_license().unwrap_or(UNKNOWN).to_string(),
                    authors: package
                        .get_packager()
                        .unwrap_or(package.get_vendor().unwrap_or(UNKNOWN))
                        .to_string(),
                    arch: package.get_arch().unwrap_or("noarch").to_string(),
                }
            }
        };

        Ok(returned_meta)
    }

    /// The actual conversion process.
    ///
    /// Example: Converting `.rpm` -> `.spf`
    /// ```
    /// let package_path = String::from("sample_package.rpm");
    /// let loaded_package = Converter::from(package_path)?;
    ///
    /// let output_package_path = String::from("sample_package.spf");
    /// let output_package_type = PackageType::Spf;
    ///
    /// loaded_package.convert(output_package_type, output_package_path)?;
    /// ```
    pub fn convert(
        self,
        output_package_type: PackageType,
        output_location: &str,
    ) -> Result<(), Error> {
        println!("    Loading package...");

        let metadata = self.load_source_metadata()?;
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
                        let collected_paths = match glob(&format!("{extracted_source}/**/*")) {
                            Ok(paths) => paths,
                            Err(err) => error(&format!("Failed to load paths: {err}")),
                        };

                        for path in collected_paths {
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
                            .unwrap_or(OsStr::from_bytes(b"Unknown").to_owned())
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

                        package_details.url(metadata.source);
                        package_details.packager(metadata.authors);

                        let package = package_details.build_host(build_host);

                        let direct_source_path = FileProperty::name(source_package_path)?;
                        let extracted_spf = direct_source_path.trim_end_matches(".spf");

                        // Collect paths and add them to the package
                        let collected_paths = match glob(&format!("{extracted_spf}/**/*")) {
                            Ok(paths) => paths,
                            Err(err) => error(&format!("Failed to load paths: {err}")),
                        };

                        for path in collected_paths {
                            let path = path?.display().to_string();
                            let destination = path.trim_start_matches(extracted_spf);

                            print!("\r\x1B[K    Writing path: \"{path}\"");
                            stdout().flush()?;

                            if Path::new(&path).is_file() {
                                package
                                    .with_file(
                                        &path,
                                        FileOptions::new(path.trim_start_matches(extracted_spf))
                                            .config()
                                            .noreplace(),
                                    )
                                    .unwrap_or_else(|err| {
                                        error(&format!("Failed to copy path: {err}"))
                                    });
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

                    println!("            Extracting \"./data.tar.xz\"...");
                    extract_archive("data.tar.xz", "./data", ArchiveType::Xz)?;

                    println!(
                        "    Converting package files...\n        Removing unused data files..."
                    );

                    let deb_files = ["debian-binary", "control.tar.xz", "data.tar.xz"];

                    for file in deb_files {
                        println!("            Removing \"./{file}\"...");
                        remove_file(file)?;
                    }

                    let output_location_name = FileProperty::name(output_location)?;
                    let new_package_dir = output_location_name
                        .split('.')
                        .next()
                        .unwrap_or_else(|| error("Failed to get new package directory"));

                    rename("data", new_package_dir)?;

                    println!("    Converting metadata...");

                    println!("        Generating...");

                    let spf_metadata_constructed = Meta::construct_contents(metadata)?;

                    println!("        Writing...");

                    let mut new_meta_file = File::create(format!("{new_package_dir}/META"))?;
                    new_meta_file.write_all(spf_metadata_constructed.join("\n").as_bytes())?;

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
                        .unwrap_or_else(|err| error(&format!("Cannot run \"rpm2archive\": {err}")));

                    println!("        Extracting...");

                    Command::new("rpm2archive")
                        .arg(source_package_path)
                        .status()?;

                    let tgz_file = &format!("{source_package_path}.tgz");

                    //let extract_dest = &format!("./{}", FileProperty::name(tgz_file)?);
                    let extract_dest = &FileProperty::name(tgz_file)?
                        .trim_end_matches(".rpm.tgz")
                        .to_string();

                    extract_archive(tgz_file, extract_dest, ArchiveType::Gz)?;

                    remove_file(tgz_file)?;

                    println!("    Converting metadata...\n        Collecting...");

                    let spf_metadata_constructed = Meta::construct_contents(metadata)?;

                    println!("        Writing...");

                    let mut new_meta_file = File::create(format!("{extract_dest}/META"))?;
                    new_meta_file.write_all(spf_metadata_constructed.join("\n").as_bytes())?;

                    println!("    Packaging...");

                    let new_extracted_folder_name = output_location.trim_end_matches(".spf");

                    rename(extract_dest, new_extracted_folder_name)?;

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
