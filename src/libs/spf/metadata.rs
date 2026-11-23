//! Functions related to and providing tools for retrieving spf package
//! metadata.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use crate::VERSION;
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
    pub repository: String,

    /// Package license
    pub license: String,

    /// Package authors/maintainers
    pub authors: String,

    /// Package architecture
    pub arch: String,
}

impl Categories {
    pub const PROJECT_NAME: &str = "PROJECT_NAME";
    pub const VERSION: &str = "VERSION";
    pub const DESCRIPTION: &str = "DESCRIPTION";
    pub const REPOSITORY: &str = "REPOSITORY";
    pub const LICENSE: &str = "LICENSE";
    pub const AUTHORS: &str = "AUTHORS";
    pub const ARCH: &str = "ARCH";

    /// Create new instance of categories.
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            repository: String::new(),
            license: String::new(),
            authors: String::new(),
            arch: String::new(),
        }
    }

    /// Get the metadata categories from an existing [`META`] instance using
    /// [`load_value`].
    ///
    /// This comes in handy when the [`META`] instance already has the contents
    /// covered, but not the categories itself.
    ///
    /// ```
    /// let meta_file_contents = read_to_string("META")?;
    ///
    /// let mut meta = Self {
    ///     contents: meta_file_contents,
    ///     categories: Categories::new(),
    /// };
    ///
    /// let categories = Categories::from(meta.clone());
    ///
    /// meta.categories = categories;
    /// ```
    pub fn from(meta: Meta) -> Self {
        Self {
            name: meta.load_value(Self::PROJECT_NAME),
            version: meta.load_value(Self::VERSION),
            description: meta.load_value(Self::DESCRIPTION),
            repository: meta.load_value(Self::REPOSITORY),
            license: meta.load_value(Self::LICENSE),
            authors: meta.load_value(Self::AUTHORS),
            arch: meta.load_value(Self::ARCH),
        }
    }
}

/// [`Meta`] refers to the metadata found within either a .spf package or an already installed
/// package that has its metadata stored at `/usr/share/spf/packages/` ([`PACKAGE_INSTALL_PATH`]).
///
/// - `meta_file_contents` refers to the actually text (the metadata itself) inside the file.
/// - `categories` refers to the actual metadata itself, using [`Categories`].
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
#[derive(Clone, Debug)]
pub struct Meta {
    pub contents: String,
    pub categories: Categories,
}

impl Meta {
    /// Create a new metadata instance
    pub const fn new() -> Self {
        Self {
            contents: String::new(),
            categories: Categories::new(),
        }
    }

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
    pub fn from(loaded_meta_file_path: &str) -> Result<Self, Error> {
        let meta_file_contents = read_to_string(loaded_meta_file_path)?;

        let mut meta = Self {
            contents: meta_file_contents,
            categories: Categories::new(),
        };

        meta.categories = Categories::from(meta.clone());

        Ok(meta)
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
    pub fn load_value(&self, category_to_find: &str) -> String {
        let string_prior_to_value = &format!("{category_to_find} =");

        self.contents
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
            .to_string()
    }

    /// Create the contents of the spf package metadata.
    ///
    /// Inputs:
    /// - `metadata` ([`Categories`]): The metadata you want to be constructed
    ///
    /// ```
    /// let meta = Meta::from("sample.spf")?;
    ///
    /// let metadata = meta.categories;
    ///
    /// let metadata_contents = construct_contents(metadata);
    /// ```
    pub fn construct_contents(metadata: Categories) -> Vec<String> {
        vec![
            format!("### PACKAGED WITH SPF {VERSION} ###\n"),
            format!("{} = {}", Categories::PROJECT_NAME, metadata.name),
            format!("{} = {}", Categories::VERSION, metadata.version),
            format!("{} = {}", Categories::DESCRIPTION, metadata.description),
            format!("{} = {}", Categories::REPOSITORY, metadata.repository),
            format!("{} = {}", Categories::LICENSE, metadata.license),
            format!("{} = {}", Categories::AUTHORS, metadata.authors),
            format!("{} = {}", Categories::ARCH, metadata.arch),
        ]
    }

    /// Loads the packages from a package build config.
    ///
    /// Only collects entries between `:::PATH DEFINE START:::` and `:::PATH DEFINE END:::`.
    ///
    /// ```
    /// let package_config_path: "spf_config";
    /// let package = Meta::from("sample.spf")?;
    ///
    /// let paths = package.get_paths();
    /// ```
    pub fn get_paths(&self) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();

        let mut header_is_read = false;

        for entry in self.contents.lines() {
            if entry == PackagedPaths::START_HEADER {
                header_is_read = true;
                continue;
            } else if !header_is_read {
                continue;
            } else if entry == PackagedPaths::END_HEADER {
                break;
            }

            paths.push(entry.to_string());
        }

        paths
    }
}

/// Construct paths used in metadata
#[derive(Clone, Debug)]
pub struct PackagedPaths {
    pub source: String,
    pub dest: String,
    pub packaged_paths: String,
}

#[allow(unused)]
impl PackagedPaths {
    /// The header that starts the section in a package builder files
    /// that defines the paths.
    pub const START_HEADER: &str = ":::PATH DEFINE START:::";

    /// The header that ends the section in a package builder files
    /// that defines the paths.
    pub const END_HEADER: &str = ":::PATH DEFINE END:::";

    /// New instance of [`PackagedPaths`]
    pub const fn new() -> Self {
        Self {
            source: String::new(),
            dest: String::new(),
            packaged_paths: String::new(),
        }
    }

    /// Take separate source and destination paths and convert them to
    /// [`PackagedPaths`].
    ///
    /// ```
    /// let source = "./source/file".to_string();
    /// let dest = "/dest/location/file".to_string();
    ///
    /// let paths = PackagedPaths::from_source_dest(source, dest);
    ///
    /// assert_eq!(source, paths.source);
    /// assert_eq!(dest, paths.dest);
    /// ```
    pub fn from_source_dest(source: String, dest: String) -> Self {
        Self {
            source: source.clone(),
            dest: dest.clone(),
            packaged_paths: Self::into_spf_path(source, dest),
        }
    }

    /// Take a packaged location path and convert it to [`PackagedPaths`].
    ///
    /// ```
    /// let config = "./source/file:/dest/location/file".to_string();
    ///
    /// let paths = PackagedPaths::from_formatted(config);
    ///
    /// assert_eq!(config, paths.packaged_path);
    /// ```
    pub fn from_formatted(packaged_paths: String) -> Self {
        let make_string = |thing: Option<&str>| -> String { thing.unwrap_or_default().to_string() };

        let mut paths_split = packaged_paths.split(':');

        let (source, dest) = (paths_split.next(), paths_split.next_back());

        Self {
            source: make_string(source),
            dest: make_string(dest),
            packaged_paths,
        }
    }

    /// Takes the source and dest paths, and formats them into what is inside a
    /// package's config.
    ///
    /// ```
    /// let source = "./source/path".to_string();
    /// let dest = "/dest/path".to_string();
    ///
    /// let spf_formatted = PackagePaths::into_spf_path(source, dest);
    ///
    /// assert_eq!(spf_formatted, &format!("{source}:{dest}"));
    /// ```
    pub fn into_spf_path(source: String, dest: String) -> String {
        format!("{source}:{dest}")
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
    pub fn verify(&self) -> Result<(), Error> {
        if self.source.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                "No source path provided",
            ));
        } else if !Path::new(&self.source).exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Source path not found: {}", self.source),
            ));
        }

        if self.dest.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                "No destination path provided",
            ));
        } else if !self.dest.starts_with("/") {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                format!("Destination path must start from the root: {}", self.dest),
            ));
        }

        Ok(())
    }
}
