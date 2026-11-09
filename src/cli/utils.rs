//! Extension of the main spf library. This one contains tools that
//! shouldn't be in the main one.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

#![allow(unused)]

use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use crate::libs::spf::{
    metadata::{Meta, PACKAGE_INSTALL_PATH},
    package::Package,
};

impl Package {
    /// Gets the tentative install location of a package.
    ///
    /// ```
    /// let package = Package::new("test");
    ///
    /// let install_location = package.install_location()?;
    ///
    /// assert_eq!(install_location, format!("/usr/share/spf/packages/test"))
    /// ```
    pub fn get_install_location(&self) -> Result<String, Error> {
        let location = format!("{PACKAGE_INSTALL_PATH}/{}", self.name.clone());

        Ok(location)
    }

    /// Gets the list of packages that were installed when the package was installed.
    ///
    /// It leads the metadata install package to do so.
    ///
    /// ```
    /// let package = Package::new("test".to_string());
    ///
    /// let paths = package.get_installed_paths()?;
    /// ```
    pub fn get_installed_paths(&self) -> Result<Vec<String>, Error> {
        let mut paths: Vec<String> = Vec::new();

        for path in Meta::from(&self.package_path)?.get_paths()? {
            paths.push(path)
        }

        Ok(paths)
    }

    /// Loads details from an installed package. It reads the metadata file
    /// from `/usr/share/spf/packages/`.
    ///
    /// ```
    /// let package_name = "test".to_string();
    /// let package = from_installed(package_name)?;
    /// ```
    pub fn from_installed(name: String) -> Result<Self, Error> {
        let installed_path = format!("{PACKAGE_INSTALL_PATH}{name}");

        if !Path::new(&installed_path).exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Package not installed: {name}"),
            ));
        }

        let metadata = Meta::from(&installed_path)?.get_metadata()?;

        let mut package = Self::new(name).add_metadata(metadata);

        package.package_path = package.get_install_location()?;

        for path in package.get_installed_paths()? {
            package.paths.push(path);
        }

        Ok(package)
    }
}
