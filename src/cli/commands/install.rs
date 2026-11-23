//! The installation process of an spf package.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env::consts::ARCH,
    fs::{OpenOptions, copy, create_dir_all, remove_dir_all, remove_file},
    io::{
        Error,
        ErrorKind::{NotFound, PermissionDenied, Unsupported},
        Write, stdin,
    },
    path::Path,
};

use self_replace::self_replace;
use semver::Version;

use crate::{
    cli::args::{IN_DEBUG_MODE, args_contains},
    libs::{
        spf::{
            metadata::{Categories, Meta, PACKAGE_INSTALL_PATH, PackagedPaths},
            package::Package,
        },
        sys::{SUPPORTED_ARCHS, is_root},
    },
};

#[derive(Debug)]
struct Installer {
    package: Package,
    extracted_path: String,
    extracted_meta: String,
    is_confirmed: bool,
}

impl Installer {
    fn cleanup(&self) -> Result<(), Error> {
        remove_dir_all(&self.extracted_path)?;
        Ok(())
    }

    fn load(package_path: &str) -> Result<Self, Error> {
        let mut package = Package::from(package_path.to_string());
        let extracted_path = package.as_extracted().package_path;

        if Path::new(&extracted_path).exists() {
            remove_dir_all(extracted_path.clone())?
        }

        package.extract(".")?;

        let extracted_meta = format!("{extracted_path}/META");
        let metadata = Meta::from(&extracted_meta)?.categories;

        package.name = metadata.name.clone();
        package.metadata = metadata;

        for path in Package::from(extracted_path.clone()).get_packaged_fs()? {
            let found_path = path.trim_start_matches(&extracted_path).to_string();

            if found_path == "/META" {
                continue;
            }

            package.paths.push(found_path);
        }

        Ok(Self {
            package,
            extracted_path,
            extracted_meta,
            is_confirmed: false,
        })
    }

    fn initialize(&mut self) -> Result<(), Error> {
        let install_type = self.package.check_version()?;
        let message = install_type.as_message();

        if *IN_DEBUG_MODE {
            println!("DEBUG: Install Type: {install_type:?}, using message '{message}'")
        }

        println!(
            "Do you want to proceed to {message} {}-{}?\n\n\
        Description: {}\n\
        Repository: {}\n\
        License(s): {}\n\
        Author(s): {}\n\
        Arch: {}\n\n\
        (Y/N)",
            self.package.metadata.name,
            self.package.metadata.version,
            self.package.metadata.description,
            self.package.metadata.repository,
            self.package.metadata.license,
            self.package.metadata.authors,
            self.package.metadata.arch
        );

        let mut proceed_to_install = String::new();
        stdin().read_line(&mut proceed_to_install)?;

        proceed_to_install = proceed_to_install.trim().to_lowercase();

        self.is_confirmed = proceed_to_install == "y" || proceed_to_install.is_empty();

        Ok(())
    }

    fn install(&self) -> Result<(), Error> {
        create_dir_all(PACKAGE_INSTALL_PATH)?;

        let installed_package_meta = format!("{PACKAGE_INSTALL_PATH}{}", self.package.name);

        copy(&self.extracted_meta, &installed_package_meta)?;
        remove_file(&self.extracted_meta)?;

        let mut project_meta_file = OpenOptions::new()
            .append(true)
            .open(installed_package_meta)?;

        project_meta_file.write_all(format!("\n{}\n", PackagedPaths::START_HEADER).as_bytes())?;

        for path in &self.package.paths {
            let source_path = format!("{}{path}", self.extracted_path);

            if Path::new(&source_path).is_dir() {
                create_dir_all(path)?
            } else {
                println!("    Copying \"{source_path}\" -> \"{path}\"");

                // Replace the spf binary to prevent crashes. For some shitty reason
                // it will crash otherwise. Don't ask me.
                let spf_binary_path = "/usr/bin/spf";
                if path == spf_binary_path && Path::new(spf_binary_path).exists() {
                    self_replace(path)?;
                }

                copy(source_path, path)?;

                project_meta_file.write_all(format!("{path}\n").as_bytes())?;
            }
        }

        Ok(())
    }
}

pub fn spf_install(package_path: String) -> Result<(), Error> {
    if !is_root()? {
        eprintln!("Root permissions are required to install packages.");
        return Err(Error::from(PermissionDenied));
    }

    if package_path.is_empty() {
        eprintln!("Please provide an .spf package file.");

        return Err(Error::from(NotFound));
    }

    println!("Loading package '{package_path}'...\n");

    let mut installer_information = match Installer::load(&package_path) {
        Ok(info) => info,
        Err(err) => {
            println!("Failed to load package '{package_path}': {err}");
            return Err(err);
        }
    };

    if *IN_DEBUG_MODE {
        println!("DEBUG: Installer information:\n{installer_information:#?}")
    }

    if !args_contains("--ignore-arch")? && !installer_information.package.metadata.arch_is_compat()
    {
        eprintln!(
            "Package architecture: {} doesn't match current system architecture ({ARCH}).\n\
            NOTE: Bypass by passing: `--ignore-arch`",
            installer_information.package.metadata.arch
        );

        installer_information.cleanup()?;

        return Err(Error::from(Unsupported));
    } else {
        match installer_information.initialize() {
            Ok(()) => (),
            Err(err) => {
                eprintln!(
                    "Failed to initialize installation for '{}': {err}",
                    installer_information.package.package_path
                );
                return Err(err);
            }
        }

        if installer_information.is_confirmed {
            println!("\nInstalling...");

            let name = installer_information.package.name.clone();

            match installer_information.install() {
                Ok(()) => {
                    println!("\nDone! Installed '{name}'!");
                }
                Err(err) => {
                    eprintln!("\nFailed to install '{name}': {err}");

                    installer_information.cleanup()?;

                    return Err(err);
                }
            }
        } else {
            println!("\nAborted.");
        }
    }

    installer_information.cleanup()?;

    Ok(())
}

#[derive(Debug)]
enum InstallStatus {
    Install,
    Update,
    Downgrade,
    Reinstall,
}

impl InstallStatus {
    fn as_message(&self) -> &str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::Downgrade => "downgrade",
            Self::Reinstall => "reinstall",
        }
    }
}

impl Categories {
    fn arch_is_compat(&self) -> bool {
        SUPPORTED_ARCHS.contains(&self.arch.as_str())
    }
}

impl Package {
    fn check_version(&self) -> Result<InstallStatus, Error> {
        if !self.is_already_installed() {
            return Ok(InstallStatus::Install);
        }

        let get_installed_version = || -> Result<String, Error> {
            let version = Self::from_installed(self.name.to_owned())?.metadata.version;

            Ok(version)
        };

        let (package_version, installed_version) =
            (self.metadata.version.to_string(), get_installed_version()?);

        if package_version == installed_version {
            return Ok(InstallStatus::Reinstall);
        }

        let parse_version = |old: &str, new: &str| -> Result<InstallStatus, semver::Error> {
            if Version::parse(old)? <= Version::parse(new)? {
                Ok(InstallStatus::Update)
            } else {
                Ok(InstallStatus::Downgrade)
            }
        };

        let update_status = parse_version(
            installed_version.trim_start_matches("v"),
            package_version.trim_start_matches("v"),
        )
        .unwrap_or(InstallStatus::Update);

        Ok(update_status)
    }
}
