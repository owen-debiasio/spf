//! Functions related to and providing tools for retrieving spf package
//! metadata.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;

/// Where spf package metadata is installed to.
pub static PACKAGE_INSTALL_PATH: &str = "/usr/share/spf/packages/";

/// [`Meta`] refers to the metadata found within either a .spf package or an already installed
/// package that has its metadata stored at `/usr/share/spf/packages/` ([`PACKAGE_INSTALL_PATH`]).
///
/// - 'meta_file_contents` refers to the actually text (the metadata itself) inside the file.
/// - `meta_file` refers to the file to observe and search for metadata
///
/// You can load package metadata by using [`Meta::from`], and then you can extract a value from
/// a category by using [`Meta::load_value`].
///
/// Both functions are public, and derive [`Clone`]. Because of that, you will need to use `.clone()`
/// a lot if you want to retrieve the values.
/// ```
/// // Example: Loading the package's name
///
/// // The metadata file to load
/// let package_metadata_location = "/usr/share/spf/packages/some_package";
///
/// // Load the contents of the metadata file
/// let metadata_contents = Meta::from(package_metadata_location);
///
/// // The category to extract the value from
/// let category_to_extract = "PROJECT_NAME";
///
/// // The extracted project name
/// let extracted_value = metadata_contents.load_value(category_to_extract);
/// ```
#[derive(Clone)]
pub struct Meta {
    meta_file_contents: String,
    meta_file: String,
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
    /// let metadata_contents = Meta::from(metadata_file);
    /// ```
    pub fn from(loaded_meta_file: &str) -> Meta {
        let meta_file_contents =
            fs::read_to_string(loaded_meta_file).expect("Failed to retrieve project metadata");

        Meta {
            meta_file: loaded_meta_file.to_string(),
            meta_file_contents,
        }
    }

    /// Extracts the desired value from the metadata loaded by [`Meta::from`].
    ///
    /// Returned as [`String`]`.
    ///
    /// ```
    /// // Metadata file to load
    /// let metadata_file = "/usr/share/spf/packages/some_package";
    ///
    /// // `metadata_contents` contains the loaded metadata from desired location
    /// // (`metadata_file` in this case).
    /// let metadata_contents = Meta::from(metadata_file);
    ///
    /// // For example, load from the package architecture
    /// let category_to_extract_value = "ARCH";
    ///
    /// // Retrieved the stored architecture
    /// let extracted_value = metadata_contents.load_value(category_to_extract_value)
    /// ```
    ///
    /// NOTE: You may need to use `clone`
    pub fn load_value(self, category_to_find: &'static str) -> String {
        let string_prior_to_value = &format!("{category_to_find} =");

        self.meta_file_contents.split('\n')
        // Find the line that contains the category.
        //
        // Must not be a comment, and it must start with the
        // category + the value identifier (`string_prior_to_value`)
        .find(|entry| entry.starts_with(string_prior_to_value) && !entry.starts_with('#'))
        .unwrap_or_else(|| panic!(
            "Failed to retrieve metadata value from category \"{category_to_find}\" in file \"{}\"", self.meta_file
        ))
        // Return the value of the category by stripping out the category name
        .trim_start_matches(string_prior_to_value)
        .trim()
        .to_string()
    }
}
