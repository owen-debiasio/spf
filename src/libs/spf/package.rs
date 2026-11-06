//! Library for managing spf files
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{copy, create_dir, create_dir_all, remove_dir_all, write},
    io::{Error, ErrorKind},
    path::Path,
};

use crate::{
    cli::commands::convert::PackageType,
    libs::{
        core::fs::{ArchiveType, create_archive, extract_archive},
        spf::metadata::{Categories, Meta},
    },
};

/// Implementation of an spf package
#[derive(Clone, Debug)]
pub struct Package {
    name: String,
    package_path: String,
    metadata: Categories,
    paths: Vec<String>,
}

impl Package {
    /// Extracts an spf package to a location.
    ///
    /// Since the package is a `tar` file, use [`extract_archive`] with
    /// [`ArchiveType::Tar`].
    ///
    /// ```
    /// let package_path = String::from("test/spf.spf");
    /// let package = Package::from(package_path)?;
    ///
    /// let destination = "./output";
    /// package.extract(destination)?;
    /// ```
    pub fn extract(&self, destination: &str) -> Result<(), Error> {
        extract_archive(&self.package_path, destination, ArchiveType::Tar)?;

        Ok(())
    }
}

impl Package {
    /// Loads an spf package.
    ///
    /// ```
    /// let package_path = String::from("test/spf.spf");
    /// let package = Package::from(package_path)?;
    /// ```
    pub fn from(package_path: String) -> Result<Self, Error> {
        let details = Self {
            name: String::new(),
            package_path,
            metadata: Categories::new(),
            paths: Vec::new(),
        };

        Ok(details)
    }

    /// Initializes an spf package.
    ///
    /// Only the name of the package must be provided here.
    ///
    /// **NOTE**:
    /// The name is NOT the name of the package FILE, it serves as the name
    /// of the package in the metadata.
    ///
    /// By default, the output path will be `<name>.spf`
    ///
    /// ```
    /// let name_of_package = "test";
    ///
    /// let package = Package::new(name_of_package);
    /// ```
    pub fn new(name: String) -> Self {
        let default_output = format!("{name}.spf");

        Self {
            name,
            package_path: default_output,
            metadata: Categories::new(),
            paths: Vec::new(),
        }
    }

    /// Adds the package metadata.
    ///
    /// Using the [`Categories`] struct, this defines what metadata the
    /// package will contain.
    ///
    /// ```
    /// let name_of_package = "test";
    /// let package = Package::new(name_of_package)
    ///     .add_metadata(Categories::new());
    /// ```
    pub fn add_metadata(&self, metadata: Categories) -> Self {
        Self {
            name: self.name.clone(),
            package_path: self.package_path.clone(),
            metadata,
            paths: self.paths.clone(),
        }
    }

    /// Defines the output location of the package.
    ///
    /// ```
    /// let name_of_package = "test";
    /// let package = Package::new(name_of_package)
    ///     .output_location("./test_output.spf")
    /// ```
    pub fn output_location(&self, path: &str) -> Result<Self, Error> {
        if Path::new(path)
            .extension()
            .unwrap_or_default()
            .display()
            .to_string()
            != PackageType::SPF
        {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                format!("Output path '{path}' must be a .spf file"),
            ));
        }

        Ok(Self {
            name: self.name.clone(),
            package_path: path.to_string(),
            metadata: self.metadata.clone(),
            paths: self.paths.clone(),
        })
    }

    #[allow(unused)]
    /// Adds one path to be copied.
    ///
    /// Directories and files are both supported.
    ///
    /// ```
    /// let name_of_package = "test";
    /// let package = Package::new(name_of_package)
    ///     .add_path("./target/debug/spf")
    /// ```
    pub fn add_path(&self, path: String) -> Self {
        Self {
            name: self.name.clone(),
            package_path: self.package_path.clone(),
            metadata: self.metadata.clone(),
            paths: {
                let mut coll_paths = self.paths.clone();
                coll_paths.push(path);
                coll_paths
            },
        }
    }

    #[allow(unused)]
    /// Builds and assembles the package.
    ///
    /// ```
    /// let name_of_package = "test";
    /// let package = Package::new(name_of_package)
    ///     .add_metadata(Categories::new())
    ///     .output_location("./test_output.spf")
    ///     .add_path("./target/debug/spf")
    ///     .add_paths(["./path/to/thing", "./other/path/to/thing/"]);
    ///
    /// package.build()?
    /// ```
    pub fn build(&self) -> Result<(), Error> {
        let raw_folder_name = Path::new(&self.package_path)
            .file_name()
            .unwrap_or_default()
            .display()
            .to_string();

        let raw_folder_name = raw_folder_name.trim_end_matches(".spf");

        create_dir(raw_folder_name);

        let meta = self.metadata.clone();

        write(
            format!("{raw_folder_name}/META"),
            Meta::construct_contents(meta)?.join("\n"),
        );

        for path in self.paths.clone() {
            let source = path.split(':').next().unwrap_or_default();

            if source.is_empty() {
                return Err(Error::new(
                    ErrorKind::InvalidFilename,
                    "No source path provided",
                ));
            }

            let dest = path.split(':').next_back().unwrap_or_default();

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

            let file_dest_str = format!("{raw_folder_name}{dest}");
            let file_dest = Path::new(&file_dest_str);

            if let Some(parent_dir) = file_dest.parent() {
                create_dir_all(parent_dir)?;
            }

            copy(source, file_dest)?;
        }

        create_archive(&self.package_path, raw_folder_name, ArchiveType::Tar)?;

        remove_dir_all(raw_folder_name)?;

        Ok(())
    }
}
