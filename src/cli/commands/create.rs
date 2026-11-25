//! Functionality to create an spf package a user desires.
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

use crate::{
    cli::args::IN_DEBUG_MODE,
    libs::spf::{
        metadata::{Meta, PackagedPaths},
        package::Package,
    },
};

pub fn create(package_config: String, output_location: String) -> Result<(), Error> {
    if package_config.is_empty() {
        eprintln!("Package configuration not provided!");

        return Err(Error::from(InvalidInput));
    } else if !Path::new(&package_config).exists() {
        eprintln!("Package configuration '{package_config}' not found!");

        return Err(Error::from(NotFound));
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

    let package_metadata = Meta::from(&package_config)?;

    if *IN_DEBUG_MODE {
        println!(
            "DEBUG: Collected metadata: {:?}",
            package_metadata.categories
        );
    }

    let mut package = Package::new(package_metadata.categories.name.clone());

    package.set_metadata(package_metadata.categories.clone());

    if !output_location.is_empty() {
        package.set_output_location(output_location.clone())
    }

    println!("    Collecting paths...");

    for path in package_metadata.get_paths() {
        println!("        Adding path: '{path}'");

        let path = PackagedPaths::from_formatted(path);

        package.add_path(path.source, path.dest)
    }

    if *IN_DEBUG_MODE {
        println!("DEBUG: Final package Information: {package:?}");
    }

    println!("    Building...");

    match package.build() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Failed to build package '{output_location}': {err}");

            return Err(err);
        }
    }

    println!("Done!");

    Ok(())
}
