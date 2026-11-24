//! Shared functions and variables that assist with returning system properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    collections::HashSet,
    fs::{remove_dir_all, remove_file},
    io::{
        Error,
        ErrorKind::{InvalidInput, NotFound, PermissionDenied},
        stdin,
    },
    path::Path,
};

use crate::{
    cli::args::IN_DEBUG_MODE,
    libs::{
        spf::package::Package,
        sys::{get_binary_path, is_root},
    },
};

use self_replace::self_replace;

#[derive(Debug)]
struct Remover {
    provided_packages: Vec<String>,
    installed: Vec<String>,
    not_installed: Vec<String>,
}

impl Remover {
    fn gather_packages(provided_packages: Vec<String>) -> Self {
        let mut remover = Self {
            provided_packages,
            installed: Vec::new(),
            not_installed: Vec::new(),
        };

        remover.sanitize();

        remover
    }

    fn sanitize(&mut self) {
        let package_set: HashSet<_> = self.provided_packages.drain(..).collect();

        self.provided_packages.clear();

        self.provided_packages.extend(package_set);
    }

    fn determine_installed(&mut self) {
        let mut not_installed_packages = Vec::new();
        let mut installed_packages = Vec::new();

        for package in self.provided_packages.clone() {
            if !Package::new(package.clone()).is_already_installed() {
                not_installed_packages.push(package);
            } else {
                installed_packages.push(package);
            }
        }

        self.not_installed = not_installed_packages;
        self.installed = installed_packages;
    }

    fn verify(&self) -> Result<(), Error> {
        if !self.not_installed.is_empty() {
            eprintln!(
                "\nError: The following packages are not installed:\n\n{}\n",
                self.not_installed.join("\n")
            );

            Err(Error::from(NotFound))
        } else {
            Ok(())
        }
    }

    fn initialize_removal(&self) -> Result<(), Error> {
        println!(
            "\nAre you sure that you want to remove the following:\n\n{}\n\n(Y/N)",
            self.installed.join("\n")
        );

        let mut proceed_to_remove = String::new();
        stdin().read_line(&mut proceed_to_remove)?;

        if proceed_to_remove.trim().to_lowercase() != "y" {
            println!("Aborted.");

            Err(Error::from(InvalidInput))
        } else {
            Ok(())
        }
    }

    fn remove_installed(&self) -> Result<(), Error> {
        for package in self.installed.clone() {
            println!("Removing {package}...");

            let package_as_package = Package::from_installed(package)?;

            println!("    Removing installed files...");

            for path in package_as_package.clone().paths {
                let path_as_path = Path::new(&path);

                if !path_as_path.exists() {
                    println!("        Path '{path}' not found... Skipping...");
                    continue;
                } else {
                    if *IN_DEBUG_MODE {
                        println!("        Removing '{path}'...");
                    }
                }

                if path_as_path.is_dir() {
                    remove_dir_all(path)?;
                } else {
                    if path == get_binary_path()? {
                        self_replace(path.clone())?;
                    }

                    remove_file(path)?;
                }
            }

            println!("    Removing package entry...");
            let package_entry = package_as_package.get_install_location();
            remove_file(package_entry)?;
        }

        Ok(())
    }
}

pub fn remove_packages(packages: Vec<String>) -> Result<(), Error> {
    if !is_root()? {
        eprintln!("Root permissions are required to remove packages.");
        return Err(Error::from(PermissionDenied));
    }

    if packages.is_empty() {
        eprintln!("Please provide package(s) to remove!");
        return Err(Error::from(InvalidInput));
    }

    println!("Initializing...");
    let mut remover = Remover::gather_packages(packages);

    println!("Evaluating installed...");
    remover.determine_installed();
    remover.verify()?;

    match remover.initialize_removal() {
        Ok(()) => (),
        Err(err) => return Err(err),
    }

    println!("\nRemoving Packages...\n");
    match remover.remove_installed() {
        Ok(()) => println!("Done!"),
        Err(err) => {
            eprintln!("Failed to remove package(s): {err}");
            return Err(err);
        }
    };

    Ok(())
}
