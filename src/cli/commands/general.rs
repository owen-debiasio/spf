//! Basic command line utilities that don't need separate source files.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    io::{
        Error,
        ErrorKind::{InvalidInput, NotFound},
    },
    path::Path,
};

use crate::libs::spf::package::Package;

pub fn extractor(package_path: &str, mut output: &str) -> Result<(), Error> {
    let package = Package::from(package_path.to_string());

    if package_path.is_empty() {
        eprintln!("Please provide a package.");

        return Err(Error::from(InvalidInput));
    } else if !Path::new(package_path).exists() {
        eprintln!("Package '{package_path}' not found!");

        return Err(Error::from(NotFound));
    } else if !package.is_spf_package() {
        eprintln!("Package '{package_path}' does not look like a valid package!");

        return Err(Error::from(InvalidInput));
    }

    if output.is_empty() {
        output = "."
    }

    package.extract(output)?;

    Ok(())
}

pub mod template {
    //! Provides the functions that allow a sample package metadata
    //! config to be created.
    //!
    //! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
    //! SPDX-License-Identifier: GPL-3.0-or-later

    use std::{
        env::current_dir,
        fs::write,
        io::{Error, ErrorKind::InvalidInput},
        path::Path,
    };

    /// Sample metadata contents to be printed. Every possible feature
    /// of the metadata is included.
    ///
    /// Stored as [`str`]
    static TEMPLATE_CONTENTS: &str = "\
# Header for project meta
# NOTE: META MUST BE DEFINED BEFORE PATHS
:::META DEFINE START:::
PROJECT_NAME = name
VERSION = v0.1.0
DESCRIPTION = Sample config
REPOSITORY = https://github.com/owen-debiasio/spf
LICENSE = gplv3
AUTHORS = spf
ARCH = x86_64
:::META DEFINE END:::

# Paths to file
# Key:
# original/file/path:location/to/install
:::PATH DEFINE START:::
target/debug/spf:/usr/bin/spf
:::PATH DEFINE END:::\n";

    /// Generate template to chosen directory.
    /// By default, file is generated to current directory
    ///
    /// The template itself is stored in [`TEMPLATE_CONTENTS`] as [`str`].
    ///
    /// `output_location`, stored as [`String`], must be a directory. If
    /// the output location isn't a directory, [`gen_meta_template`] will
    /// panic.
    ///
    /// ```
    /// let location_to_send_template = "~/Documents/"
    ///
    /// gen_meta_template(location_to_send_template)
    ///
    /// // Output location is `~/Documents/spf_template`
    /// ```
    pub fn gen_meta_template(mut output_location: String) -> Result<(), Error> {
        if output_location.is_empty() {
            output_location = current_dir()?.to_string_lossy().to_string();

        // Output location has to be a directory because rust doesn't want to
        // listen to me
        } else if !Path::new(&output_location).is_dir() {
            eprintln!("Please enter a directory to generate to.");

            return Err(Error::from(InvalidInput));
        }

        output_location = format!("{output_location}/spf_template");

        // Write the contents
        write(&output_location, TEMPLATE_CONTENTS)?;

        println!("Generated template at: {output_location}");

        Ok(())
    }
}
