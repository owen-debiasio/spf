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
    io::{Error, ErrorKind, Write, stdin, stdout},
    os::unix::ffi::OsStrExt,
    path::Path,
    process::Command,
};

use cmd_exists::cmd_exists;
use deb_rs2::file::Deb;
use glob::glob;

use deb_rust::{DebArchitecture, DebFile, binary::DebPackage};
use rpm::{FileOptions, PackageBuilder, PackageMetadata};

use crate::libs::{
    core::{
        fs::{ArchiveType, create_archive, extract_archive},
        sys::convert_arch,
    },
    spf::{
        metadata::{Categories, Meta},
        package::Package,
    },
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
/// - Source / Input packages are provided
/// - Verify if the source package exists
/// - Source / Input packages are supported package types
/// - Source / Input packages are the same type (`.spf` -> `.spf`)
/// - User tries conversion between `.rpm` and `.deb`
fn verify_input_paths(source_package_path: &str, output_package_path: &str) -> Result<(), Error> {
    // Verify if the source package is provided
    if source_package_path.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Source package required",
        ));
    }

    if output_package_path.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Output location required",
        ));
    }

    // Verify if provided package paths exist
    if !Path::new(source_package_path).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Source package not found: {source_package_path}"),
        ));
    }

    let get_ext = |file: &str| -> String {
        Path::new(file)
            .extension()
            .unwrap_or_default()
            .display()
            .to_string()
    };

    // Get package file extensions
    let source_file_ext = get_ext(source_package_path);
    let output_file_ext = get_ext(output_package_path);

    // Verify supported package formats using the extensions
    if !matches!(
        source_file_ext.as_str(),
        PackageType::SPF | PackageType::DEB | PackageType::RPM
    ) {
        return Err(Error::new(
            ErrorKind::InvalidFilename,
            format!("Invalid input package type: {source_file_ext}"),
        ));
    } else if !matches!(
        output_file_ext.as_str(),
        PackageType::SPF | PackageType::DEB | PackageType::RPM
    ) {
        return Err(Error::new(
            ErrorKind::InvalidFilename,
            format!("Invalid output package type: {output_file_ext}"),
        ));
    }

    // Makes sure the use doesn't try to convert 2 of the same package types
    if source_file_ext == output_file_ext {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Source and output packages must not be the same: source_file_ext={source_file_ext}, output_file_ext={output_file_ext}"
            ),
        ));
    }

    // Make sure user doesn't attempt to convert `.deb` -> `.rpm` (and vice versa)
    if (source_file_ext == PackageType::RPM && output_file_ext == PackageType::DEB)
        || (source_file_ext == PackageType::DEB && output_file_ext == PackageType::RPM)
    {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                ".deb and .rpm files can not be converted back and forth: source_file_ext={source_file_ext}, output_file_ext={output_file_ext}"
            ),
        ));
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
    verify_input_paths(source_package_path, output_package_path)?;

    disclaimer()?;

    println!("Converting \"{source_package_path}\" -> \"{output_package_path}\"...");

    Converter::from(source_package_path.to_string())?.convert(output_package_path)?;

    println!("\nDone! Converted \"{source_package_path}\" -> \"{output_package_path}\"!");

    Ok(())
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
    pub fn get_from_extension(path_of_package: &str) -> Result<Self, Error> {
        let source_package_ext = Path::new(path_of_package)
            .extension()
            .unwrap_or_default()
            .display()
            .to_string();

        let package_type = match source_package_ext.as_ref() {
            Self::SPF => Self::Spf,
            Self::DEB => Self::Deb,
            Self::RPM => Self::Rpm,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid file type: {source_package_ext}"),
                ));
            }
        };

        Ok(package_type)
    }

    /// Retrieves the package type from the package extension.
    ///
    /// Returns as [`PackageType`]
    ///
    /// ```
    /// let package_type = PackageType::Deb;
    /// let extension = PackageType::to_extension(package_type)?;
    ///
    /// assert_eq!(extension, ".deb");
    /// ```
    pub fn to_extension(package_type: PackageType) -> Result<String, Error> {
        let extension = match package_type {
            Self::Spf => Self::SPF,
            Self::Deb => Self::DEB,
            Self::Rpm => Self::RPM,
        };

        Ok(format!(".{extension}"))
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
    source_package_type: PackageType,
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
    pub fn from(source_package_path: String) -> Result<Self, Error> {
        let source_package_type = PackageType::get_from_extension(&source_package_path)?;

        let source_package_details = Self {
            source_package_path,
            source_package_type,
        };

        Ok(source_package_details)
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

        let returned_meta = match self.source_package_type {
            // Get spf package metadata
            PackageType::Spf => {
                if !package_path.ends_with(".spf") {
                    return Err(Error::new(
                        ErrorKind::InvalidFilename,
                        format!("Must be .spf file: {package_path}"),
                    ));
                }

                Package::from(package_path.clone())?.extract(".")?;

                let metadata_path = &format!(
                    "{}/META",
                    Path::new(&package_path)
                        .file_name()
                        .unwrap_or_default()
                        .display()
                        .to_string()
                        .trim_end_matches(".spf")
                );

                Meta::from(metadata_path)?.categories()?
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
                let package = match PackageMetadata::open(&package_path) {
                    Ok(pkg) => pkg,
                    Err(err) => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Failed to open rpm package '{package_path}': {err}"),
                        ));
                    }
                };

                const UNKNOWN: &str = "Unknown (likely lost in conversion)";

                let make_str = |thing: Result<&str, rpm::Error>| -> String {
                    thing.unwrap_or(UNKNOWN).to_string()
                };

                Categories {
                    name: make_str(package.get_name()),
                    version: make_str(package.get_version()),
                    description: make_str(package.get_description()),
                    source: make_str(package.get_url()),
                    license: make_str(package.get_license()),
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
    pub fn convert(self, output_location: &str) -> Result<(), Error> {
        println!("    Loading package...");

        let metadata = self.load_source_metadata()?;

        let output_package_type = PackageType::get_from_extension(output_location)?;

        let source_name = Path::new(&self.source_package_path)
            .file_name()
            .unwrap_or_default()
            .display()
            .to_string();

        let extracted_source = source_name.trim_end_matches(&PackageType::to_extension(
            self.source_package_type.clone(),
        )?);

        let collected_paths = match glob(&format!("{extracted_source}/**/*")) {
            Ok(paths) => paths,
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to read directory '{extracted_source}: {err}'"),
                ));
            }
        };

        match self.source_package_type {
            PackageType::Spf => {
                println!("        Extracting...");

                extract_archive(&self.source_package_path, ".", ArchiveType::Tar)?;

                remove_file(format!("{extracted_source}/META"))?;

                match output_package_type {
                    PackageType::Spf => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Cannot convert a .spf package to a .spf package",
                        ));
                    }
                    // spf -> deb
                    PackageType::Deb => {
                        let mut package = DebPackage::new(&metadata.name);

                        let arch = metadata.arch;

                        // Convert the architectures to the `.deb` counterparts
                        let arch_to_use = match arch.as_str() {
                            "universal" => DebArchitecture::All,
                            "x86_64" => DebArchitecture::Amd64,
                            "x86" => DebArchitecture::I386,
                            "aarch64" => DebArchitecture::Arm64,
                            "arm" => DebArchitecture::Armhf,
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    format!("Failed converting arch '{arch}' to .deb equivalent"),
                                ));
                            }
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

                        for path in collected_paths {
                            let current_path = path
                                .map_err(|err| {
                                    Error::new(
                                        ErrorKind::InvalidFilename,
                                        format!("Failed to get current path: {err}"),
                                    )
                                })?
                                .display()
                                .to_string();

                            print!("\r\x1B[K        Writing path: \"{current_path}\"");
                            stdout().flush()?;

                            let destination = &current_path
                                .trim_start_matches(&self.source_package_path)
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

                        let converted_arch = convert_arch(&metadata.arch, output_package_type)?;

                        let mut package_details = PackageBuilder::new(
                            &metadata.name,
                            &metadata.version,
                            &metadata.license,
                            converted_arch,
                            &metadata.description,
                        );

                        package_details.url(metadata.source);
                        package_details.packager(metadata.authors);

                        let hostname = hostname::get()
                            .unwrap_or(OsStr::from_bytes(b"Unknown").to_owned())
                            .display()
                            .to_string();

                        let package = package_details
                            .build_host(format!("SPF package converter @ {hostname}"));

                        let collected_paths = match glob(&format!("{extracted_source}/**/*")) {
                            Ok(paths) => paths,
                            Err(err) => {
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    format!("Failed to read directory '{extracted_source}: {err}'"),
                                ));
                            }
                        };

                        for path in collected_paths {
                            let path = path?.display().to_string();
                            let destination = path.trim_start_matches(extracted_source);

                            print!("\r\x1B[K    Writing path: \"{path}\"");
                            stdout().flush()?;

                            if Path::new(&path).is_file() {
                                package
                                    .with_file(
                                        &path,
                                        FileOptions::new(path.trim_start_matches(extracted_source))
                                            .config()
                                            .noreplace(),
                                    )
                                    .map_err(|err| {
                                        Error::new(
                                            ErrorKind::InvalidData,
                                            format!("Failed to copy path: {err}"),
                                        )
                                    })?;
                            } else {
                                match package
                                    .with_dir(path.clone(), destination, |path| path.config())
                                {
                                    Ok(package) => package,
                                    Err(err) => {
                                        return Err(Error::new(
                                            ErrorKind::InvalidData,
                                            format!("Failed to create directory: {err}"),
                                        ));
                                    }
                                };
                            }
                        }

                        println!("\n    Building...");

                        let output = package.build().map_err(|err| {
                            Error::new(ErrorKind::InvalidData, format!("Failed to build: {err}"))
                        })?;

                        output.write_to(output_location).map_err(|err| {
                            Error::new(
                                ErrorKind::InvalidData,
                                format!("Failed write to output: {err}"),
                            )
                        })?;

                        remove_dir_all(extracted_source)?;
                    }
                }
            }
            PackageType::Deb => match output_package_type {
                // deb -> spf
                PackageType::Spf => {
                    println!("        Extracting...");
                    extract_archive(&self.source_package_path, ".", ArchiveType::Ar)?;

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

                    let output_location_name = Path::new(output_location)
                        .file_name()
                        .unwrap_or_default()
                        .display()
                        .to_string();

                    if let Some(new_package_dir) = output_location_name.split('.').next() {
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
                    } else {
                        return Err(Error::new(
                            ErrorKind::InvalidFilename,
                            "Failed to get new package directory",
                        ));
                    }
                }

                // deb -> deb
                PackageType::Deb => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Cannot convert a .deb package to a .deb package",
                    ));
                }
                // deb -> rpm
                PackageType::Rpm => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Cannot convert a .deb package to a .rpm package",
                    ));
                }
            },
            PackageType::Rpm => match output_package_type {
                // rpm -> spf
                PackageType::Spf => {
                    match cmd_exists("rpm2archive") {
                        Ok(status) => status,
                        Err(err) => {
                            return Err(Error::new(
                                ErrorKind::NotFound,
                                format!("Failed to run rpm2archive: {err}"),
                            ));
                        }
                    };

                    println!("        Extracting...");

                    Command::new("rpm2archive")
                        .arg(&self.source_package_path)
                        .status()?;

                    let tgz_file = &format!("{}.tgz", self.source_package_path);

                    let tgz_path_or_some_bullshit = &Path::new(tgz_file)
                        .file_name()
                        .unwrap_or_default()
                        .display()
                        .to_string();

                    let extract_dest = tgz_path_or_some_bullshit.trim_end_matches(".rpm.tgz");

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
                PackageType::Deb => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Cannot convert a .rpm package to a .deb package",
                    ));
                }
                // rpm -> rpm
                PackageType::Rpm => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Cannot convert a .rpm package to a .rpm package",
                    ));
                }
            },
        }

        Ok(())
    }
}
