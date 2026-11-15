//! The functionality to inspect any form of an spf package. Whether it's a `.spf`
//! package, by the installed package name, or the straight up package metadata
//! file.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{fs::remove_dir_all, io::Error, path::Path};

use crate::libs::spf::{
    metadata::{Meta, PACKAGE_INSTALL_PATH},
    package::Package,
};

struct Inspector {
    package: Package,
    meta_path: String,
    remove_extracted: bool,
    extracted_path: String,
    constructed_meta: String,
    initial_metadata: Meta,
    paths: String,
}

impl Inspector {
    fn load(package: &str) -> Result<Self, Error> {
        let loaded_package = Package::from(package.to_string())?;

        let mut remove_extracted = false;

        let extracted_path = loaded_package.as_extracted().package_path;

        let meta_path = if (package.starts_with(PACKAGE_INSTALL_PATH)
            && Path::new(package).exists())
            || package.ends_with("/META")
        {
            package.to_string()
        } else if loaded_package.is_spf_package()? {
            loaded_package.extract(".")?;

            remove_extracted.toggle();

            format!("{extracted_path}/META")
        } else {
            format!("{PACKAGE_INSTALL_PATH}/{package}")
        };

        Ok(Self {
            package: loaded_package,
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
            Meta::construct_contents(initial_metadata.categories.to_owned())?.join("\n");
        self.initial_metadata = initial_metadata;

        Ok(())
    }

    fn prep_paths(&mut self) -> Result<(), Error> {
        let collected_paths = if self.remove_extracted {
            let packaged_paths = self.package.as_extracted().get_packaged_fs();

            remove_dir_all(&self.extracted_path)?;

            packaged_paths
        } else {
            self.initial_metadata.get_paths()
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
    println!("Loading package '{package}'...");

    let mut loaded_package = Inspector::load(package)?;

    loaded_package.prep_meta()?;
    println!("\nInstalled metadata:\n{}", loaded_package.constructed_meta);

    loaded_package.prep_paths()?;
    println!("\nInstalled paths:\n{}", loaded_package.paths);

    Ok(())
}
