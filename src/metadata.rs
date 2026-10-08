//! Functions related to and providing tools for retrieving spf package
//! metadata.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    VERSION,
    convert::PackageType,
    sys::{SUPPORTED_ARCHS, convert_arch},
};
use std::{fs::read_to_string, io::Error};

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
    pub fn from(loaded_meta_file: &str) -> Result<Meta, Error> {
        let meta_file_contents = read_to_string(loaded_meta_file)?;

        Ok(Meta { meta_file_contents })
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

        let constructed_meta = vec![
            format!("### PACKAGED WITH SPF {VERSION} ###\n"),
            format!("PROJECT_NAME = {}", metadata.name),
            format!("VERSION = {}", metadata.version),
            format!("DESCRIPTION = {}", metadata.description),
            format!("REPOSITORY = {}", metadata.source),
            format!("LICENSE = {}", metadata.license),
            format!("AUTHORS = {}", metadata.authors),
            format!("ARCH = {arch}"),
        ];

        Ok(constructed_meta)
    }
}
