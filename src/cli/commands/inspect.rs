//! The functionality to inspect any form of an spf package. Whether it's a `.spf`
//! package, by the installed package name, or the straight up package metadata
//! file.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env::temp_dir,
    fs::remove_dir_all,
    io::{
        Error,
        ErrorKind::{InvalidFilename, InvalidInput},
    },
    path::Path,
};

use crate::{
    cli::args::IN_DEBUG_MODE,
    libs::spf::{
        metadata::{Meta, PACKAGE_INSTALL_PATH},
        package::Package,
    },
};

#[derive(Debug)]
struct Inspector {
    meta_path: String,
    remove_extracted: bool,
    extracted_path: String,
    constructed_meta: String,
    initial_metadata: Meta,
    paths: String,
}

impl Inspector {
    fn load(package: &str) -> Result<Self, Error> {
        let loaded_package = Package::from(package.to_string());

        let mut remove_extracted = false;

        let temp_dir = temp_dir().to_string_lossy().to_string();

        let extracted_path = format!("{temp_dir}/{}", loaded_package.as_extracted().package_path);

        let meta_path = if (package.starts_with(PACKAGE_INSTALL_PATH) || package.ends_with("/META"))
            && Path::new(package).exists()
        {
            package.to_string()
        } else if loaded_package.is_spf_package() {
            loaded_package.extract(&temp_dir)?;

            remove_extracted.toggle();

            format!("{extracted_path}/META")
        } else if Package::from_installed(package.to_string())?.is_already_installed() {
            format!("{PACKAGE_INSTALL_PATH}/{package}")
        } else {
            return Err(Error::from(InvalidFilename));
        };

        Ok(Self {
            meta_path,
            remove_extracted,
            extracted_path,
            constructed_meta: String::new(),
            initial_metadata: Meta::new(),
            paths: String::new(),
        })
    }

    fn prep_meta(&mut self) -> Result<(), Error> {
        let initial_metadata = Meta::from(&self.meta_path)?;

        self.constructed_meta =
            Meta::construct_contents(initial_metadata.categories.to_owned()).join("\n");
        self.initial_metadata = initial_metadata;

        Ok(())
    }

    fn prep_paths(&mut self) -> Result<(), Error> {
        let collected_paths = if self.remove_extracted {
            let packaged_paths = Package::from(self.extracted_path.clone()).get_packaged_fs();

            remove_dir_all(&self.extracted_path)?;

            packaged_paths
        } else {
            Ok(self.initial_metadata.get_paths())
        }?
        .join("\n");

        self.paths = collected_paths
            .replace(&self.extracted_path, "")
            .replace("/META", "")
            .trim_start()
            .to_string();

        Ok(())
    }
}

pub fn inspect(package: &str) -> Result<(), Error> {
    if package.is_empty() {
        eprintln!("Please provide an extracted package, metadata file, or .spf file.");
        return Err(Error::from(InvalidInput));
    }

    println!("Loading package '{package}'...");

    let mut loaded_package = match Inspector::load(package) {
        Ok(loaded) => loaded,
        Err(err) => {
            eprintln!("\nFailed to inspect '{package}': {err}");
            return Err(err);
        }
    };

    loaded_package.prep_meta()?;
    println!("\nInstalled metadata:\n{}", loaded_package.constructed_meta);

    loaded_package.prep_paths()?;
    println!("\nInstalled paths:\n{}", loaded_package.paths);

    if *IN_DEBUG_MODE {
        println!("DEBUG: loaded package: {loaded_package:#?}")
    }
    Ok(())
}
