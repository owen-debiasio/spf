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
