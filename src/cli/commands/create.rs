//! Experimental new functionality with new spf libraries.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use crate::{
    cli::args::args_contains,
    libs::spf::{
        metadata::{Meta, PackagePaths},
        package::Package,
    },
};

pub fn create(package_config: String, output_location: String) -> Result<(), Error> {
    if package_config.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "No package config provided",
        ));
    } else if !Path::new(&package_config).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Package config not found: {package_config}"),
        ));
    }

    println!(
        "Building package '{package_config}'{}",
        if output_location.is_empty() {
            "...".to_string()
        } else {
            format!(" to '{output_location}'...")
        }
    );

    println!("    Collecting metadata...");

    let meta = Meta::from(&package_config)?.categories()?;

    if args_contains("--debug")? {
        println!("DEBUG: Collected metadata: {meta:?}");
    }

    let mut package = Package::new(meta.name.clone()).add_metadata(meta);

    if !output_location.is_empty() {
        package = package.output_location(&output_location)?
    }

    println!("    Collecting paths...");

    for path in PackagePaths::load_from_config(package_config)? {
        println!("        Adding path: '{path}'");

        let (source, dest) = PackagePaths::get_src_dest(&path);

        package = package.add_path(source, dest)
    }

    if args_contains("--debug")? {
        println!("DEBUG: Final package Information: {package:?}");
    }

    println!("    Building...");

    package.build()?;

    println!("Done!");

    Ok(())
}
