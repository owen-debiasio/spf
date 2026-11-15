//! Functionality to list installed spf packages.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Error;

use crate::{cli::args::IN_DEBUG_MODE, libs::spf::package::Package};

pub fn list_packages(optional_string: &str) -> Result<(), Error> {
    let packages = Package::get_installed_names()?;

    if *IN_DEBUG_MODE {
        println!("DEBUG: Retrieved packages: {packages:?}")
    }

    let mut packages_found = 0;

    for installed in packages {
        if !installed.contains(optional_string) {
            if *IN_DEBUG_MODE {
                println!("DEBUG: '{installed}' does not contain '{optional_string}', skipping...")
            }

            continue;
        }

        let metadata = Package::from_installed(installed.clone())?.metadata;

        if *IN_DEBUG_MODE {
            println!("DEBUG: metadata of '{installed}': {metadata:?}")
        }

        println!(
            "> {installed} {}\n    > {}",
            metadata.version, metadata.description
        );

        packages_found += 1
    }

    if !optional_string.is_empty() && packages_found == 0 {
        println!("No packages found containing '{optional_string}'")
    }

    Ok(())
}
