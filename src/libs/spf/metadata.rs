//! Functions related to and providing tools for retrieving spf package
//! metadata.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    VERSION,
    cli::commands::convert::PackageType,
    libs::core::sys::{SUPPORTED_ARCHS, convert_arch},
};
use std::{
    fs::read_to_string,
    io::{Error, ErrorKind},
    path::Path,
};

/// Where spf package metadata is installed to.
pub static PACKAGE_INSTALL_PATH: &str = "/usr/share/spf/packages/";

/// These are the available categories for `spf` packages.
///
/// Every entry is stored as [`String`], and is public.
#[derive(Clone, Debug)]
pub struct Categories {
    /// Package name
    pub name: String,

    /// Package version
    pub version: String,

    /// Package description
    pub description: String,

    /// Package homepage, repository, or source
    pub source: String,

    /// Package license
    pub license: String,

    /// Package authors/maintainers
    pub authors: String,

    /// Package architecture
    pub arch: String,
}

impl Categories {
    /// Create new instance of categories.
    pub const fn new() -> Categories {
        Categories {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            source: String::new(),
            license: String::new(),
            authors: String::new(),
            arch: String::new(),
        }
    }
}

/// [`Meta`] refers to the metadata found within either a .spf package or an already installed
/// package that has its metadata stored at `/usr/share/spf/packages/` ([`PACKAGE_INSTALL_PATH`]).
///
/// - `meta_file_contents` refers to the actually text (the metadata itself) inside the file.
///
/// You can load package metadata by using [`Meta::from`], and then you can extract a value from
/// a category by using [`Meta::load_value`].
///
/// ```
/// // Example: Loading the package's name
///
/// // The metadata file to load
/// let package_metadata_location = "/usr/share/spf/packages/some_package";
///
/// // Load the contents of the metadata file
/// let metadata_contents = Meta::from(package_metadata_location)?;
///
/// // The category to extract the value from
/// let category_to_extract = "PROJECT_NAME";
///
/// // The extracted project name
/// let extracted_value = metadata_contents.load_value(category_to_extract)?;
/// ```
#[derive(Clone)]
pub struct Meta {
    meta_file_contents: String,
}

impl Meta {
    /// Loads the contents of the metadata file provided as `loaded_meta_file`.
    ///
    /// Stored as [`String`].
    ///
    /// ```
    /// // Metadata file to load
    /// let metadata_file = "/usr/share/spf/packages/some_package";
    ///
    /// // `metadata_contents` contains the loaded metadata from desired location
    /// // (`metadata_file` in this case).
    /// let metadata_contents = Meta::from(metadata_file)?;
    /// ```
    pub fn from(loaded_meta_file: &str) -> Result<Self, Error> {
        let meta_file_contents = read_to_string(loaded_meta_file)?;

        Ok(Self { meta_file_contents })
    }

    pub fn categories(&self) -> Result<Categories, Error> {
        let retrieved_categories = Categories {
            name: self.load_value("PROJECT_NAME")?,
            version: self.load_value("VERSION")?,
            description: self.load_value("DESCRIPTION")?,
            source: self.load_value("REPOSITORY")?,
            license: self.load_value("LICENSE")?,
            authors: self.load_value("AUTHORS")?,
            arch: self.load_value("ARCH")?,
        };

        Ok(retrieved_categories)
    }

    /// Extracts the desired value from the metadata loaded by [`Meta::from`].
    ///
    /// Returned as [`String`].
    ///
    /// ```
    /// // Metadata file to load
    /// let metadata_file = "/usr/share/spf/packages/some_package";
    ///
    /// // `metadata_contents` contains the loaded metadata from desired location
    /// // (`metadata_file` in this case).
    /// let metadata_contents = Meta::from(metadata_file)?;
    ///
    /// // For example, load from the package architecture
    /// let category_to_extract_value = "ARCH";
    ///
    /// // Retrieved the stored architecture
    /// let extracted_value = metadata_contents.load_value(category_to_extract_value)?
    /// ```
    pub fn load_value(&self, category_to_find: &'static str) -> Result<String, Error> {
        let string_prior_to_value = &format!("{category_to_find} =");

        let meta_contents = self
            .meta_file_contents
            .split('\n')
            // Find the line that contains the category.
            //
            // Must not be a comment, and it must start with the
            // category + the value identifier (`string_prior_to_value`)
            .find(|entry| entry.starts_with(string_prior_to_value) && !entry.starts_with('#'))
            .unwrap_or_default()
            // Return the value of the category by stripping out the category name
            .trim_start_matches(string_prior_to_value)
            .trim()
            .to_string();

        Ok(meta_contents)
    }

    /// Create the contents of the spf package metadata.
    ///
    /// Inputs:
    /// - `metadata` ([`Categories`]): The metadata you want to be constructed
    ///
    /// ```
    /// let package = Meta::from("sample.spf")?;
    ///
    /// let metadata = Categories {
    ///     name: package.load_value("PROJECT_NAME")?,
    ///     version: package.load_value("VERSION")?,
    ///     description: package.load_value("DESCRIPTION")?,
    ///     source: package.load_value("REPOSITORY")?,
    ///     license: package.load_value("LICENSE")?,
    ///     authors: package.load_value("AUTHORS")?,
    ///     arch: package.load_value("ARCH")?,
    /// };
    ///
    /// let metadata_contents = construct_contents(metadata)?;
    /// ```
    pub fn construct_contents(metadata: Categories) -> Result<Vec<String>, Error> {
        // A safeguard to help prevent mismatched arch types. Useful during tyhe
        // conversion process.
        let arch = if SUPPORTED_ARCHS.contains(&metadata.arch.as_str()) {
            &metadata.arch
        } else {
            convert_arch(&metadata.arch, PackageType::Spf)?
        };

        Ok(vec![
            format!("### PACKAGED WITH SPF {VERSION} ###\n"),
            format!("PROJECT_NAME = {}", metadata.name),
            format!("VERSION = {}", metadata.version),
            format!("DESCRIPTION = {}", metadata.description),
            format!("REPOSITORY = {}", metadata.source),
            format!("LICENSE = {}", metadata.license),
            format!("AUTHORS = {}", metadata.authors),
            format!("ARCH = {arch}"),
        ])
    }
}

/// Construct paths used in metadata
#[derive(Clone, Debug)]
pub struct PackagePaths;

impl PackagePaths {
    /// Loads the packages from a package build config.
    ///
    /// Only collects entries between `:::PATH DEFINE START:::` and `:::PATH DEFINE END:::`.
    ///
    /// ```
    /// let package_config_path: "spf_config";
    /// let paths = PackagePaths::load_from_config(package_config_path)?;
    /// ```
    pub fn load_from_config(package_config_path: String) -> Result<Vec<String>, Error> {
        let mut paths: Vec<String> = Vec::new();

        const START_HEADER: &str = ":::PATH DEFINE START:::";

        let mut header_is_read = false;

        for entry in read_to_string(package_config_path)?.lines() {
            if entry == START_HEADER {
                header_is_read = true;
                continue;
            } else if !header_is_read {
                continue;
            } else if entry == ":::PATH DEFINE END:::" {
                break;
            }

            paths.push(entry.to_string());
        }

        Ok(paths)
    }

    #[allow(unused)]
    /// Takes the source and dest paths, and formats them into what is inside a
    /// package's config.
    ///
    /// ```
    /// let source = "./source/path";
    /// let dest = "/dest/path"
    ///
    /// let spf_formatted = PackagePaths::into_spf_path(source, dest);
    ///
    /// assert_eq!(spf_formatted, &format!("{source}:{dest}"));
    /// ```
    pub fn into_spf_path(source: &str, dest: &str) -> String {
        format!("{source}:{dest}")
    }

    /// Reads the formatted path you would find in the package config, and
    /// returns both the source and destination paths.
    ///
    /// ```
    /// let formatted_paths = "./source/file:/destination/location/path"
    ///
    /// let (source, dest) = get_src_dest(formatted_paths);
    ///
    /// assert_eq!(source, "./source/file");
    /// assert_eq!(dest, "/destination/location/path");
    /// ```
    pub fn get_src_dest(formatted_path: &str) -> (&str, &str) {
        let source = formatted_path.split(':').next().unwrap_or_default();
        let dest = formatted_path.split(':').next_back().unwrap_or_default();

        (source, dest)
    }

    /// Make sure that the paths entered follow the standards correctly.
    ///
    /// Checks if:
    ///
    /// - Source package is entered and exists
    /// - Destination package is entered and starts from the root directory
    ///
    /// ```
    /// let src = "./source/package";
    /// let dest = "/dest/to/location";
    ///
    /// PackagePaths::verify_src_dest(src, dest)?; // Succeeds because all checks are passed
    /// ```
    pub fn verify_src_dest(src: &str, dest: &str) -> Result<(), Error> {
        if src.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                "No source path provided",
            ));
        } else if !Path::new(src).exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Source path not found: {src}"),
            ));
        }

        if dest.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                "No destination path provided",
            ));
        } else if !dest.starts_with("/") {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                format!("Destination path must start from the root: {dest}"),
            ));
        }

        Ok(())
    }
}
