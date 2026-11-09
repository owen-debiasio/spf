//! Library for managing spf files
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::{File, copy, create_dir, create_dir_all, remove_dir_all, write},
    io::{Error, ErrorKind},
    path::Path,
};

use tar::Builder;

use crate::libs::spf::metadata::{Categories, Meta, PackagePaths};

/// Implementation of an spf package
#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub package_path: String,
    pub metadata: Categories,
    pub paths: Vec<String>,
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
        let archive_path = File::open(&self.package_path)?;

        tar::Archive::new(archive_path).unpack(destination)?;

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

    /// Verifies if the package loaded is an spf package.
    ///
    /// It checks if the file extension is `spf`. Returns true if it is,
    /// otherwise it returns false.
    ///
    /// ```
    /// let package_path = String::from("test.spf");
    ///
    /// let package = Package::from(package_path)?;
    ///
    /// assert_eq!(package.is_spf_package(), true)
    /// ```
    pub fn is_spf_package(&self) -> Result<bool, Error> {
        let is_spf_package = Path::new(&self.package_path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("spf"));

        Ok(is_spf_package)
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
    pub fn output_location(&self, package_path: String) -> Result<Self, Error> {
        if !Self::from(package_path.clone())?.is_spf_package()? {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                format!("Output path '{package_path}' must be a .spf file"),
            ));
        }

        Ok(Self {
            name: self.name.clone(),
            package_path,
            metadata: self.metadata.clone(),
            paths: self.paths.clone(),
        })
    }

    /// Adds the definition of source and destination paths.
    ///
    /// Directories and files are both supported.
    ///
    /// ```
    /// let name_of_package = "test";
    ///
    /// let source = "./source/file";
    /// let dest = "/dest/file/location";
    ///
    /// let package = Package::new(name_of_package)
    ///     .add_path(source, dest)
    /// ```
    pub fn add_path(&self, source: &str, dest: &str) -> Self {
        Self {
            name: self.name.clone(),
            package_path: self.package_path.clone(),
            metadata: self.metadata.clone(),
            paths: {
                let mut coll_paths = self.paths.clone();
                coll_paths.push(PackagePaths::into_spf_path(source, dest));
                coll_paths
            },
        }
    }

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

        create_dir(raw_folder_name)?;

        let meta = self.metadata.clone();

        write(
            format!("{raw_folder_name}/META"),
            Meta::construct_contents(meta)?.join("\n"),
        )?;

        for path in self.paths.clone() {
            let (source, dest) = PackagePaths::get_src_dest(&path);

            PackagePaths::verify_src_dest(source, dest)?;

            let file_dest_str = format!("{raw_folder_name}{dest}");
            let file_dest = Path::new(&file_dest_str);

            if let Some(parent_dir) = file_dest.parent() {
                create_dir_all(parent_dir)?;
            }

            copy(source, file_dest)?;
        }

        let archive_file = File::create(&self.package_path)?;

        let mut archive = Builder::new(&archive_file);

        if let Some(path) = Path::new(raw_folder_name).file_name() {
            archive.append_dir_all(path, raw_folder_name)?;
        } else {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Failed to add to archive '{archive_file:?}': {raw_folder_name}"),
            ));
        }

        archive.finish()?;

        remove_dir_all(raw_folder_name)?;

        Ok(())
    }
}
