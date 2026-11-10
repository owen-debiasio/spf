use std::{
    env::consts::ARCH,
    fs::{OpenOptions, copy, remove_dir_all, remove_file},
    io::{Error, Write, stdin},
    path::Path,
};

use semver::Version;

use crate::{
    cli::args::args_contains,
    libs::{
        spf::{
            metadata::{Categories, Meta, PACKAGE_INSTALL_PATH},
            package::Package,
        },
        sys::SUPPORTED_ARCHS,
    },
};

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
        let mut package = Package::from(package_path.to_string())?;

        package.extract(".")?;

        let spf_package = Path::new(&package_path)
            .file_stem()
            .unwrap_or_default()
            .display()
            .to_string();

        let extracted_path = spf_package.trim_end_matches(".spf").to_string();

        let extracted_meta = format!("{extracted_path}/META");
        let metadata = Meta::from(&extracted_meta)?.get_metadata()?;

        package.name = metadata.name.clone();
        package.metadata = metadata;

        for path in Package::from(extracted_path.clone())?.get_packaged_fs()? {
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

    fn initialize(&self) -> Result<Self, Error> {
        let install_type = self.package.check_version()?;
        let message = install_type.as_message();

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
            self.package.metadata.source,
            self.package.metadata.license,
            self.package.metadata.authors,
            self.package.metadata.arch
        );

        let mut proceed_to_install = String::new();
        stdin().read_line(&mut proceed_to_install)?;

        proceed_to_install = proceed_to_install.trim().to_lowercase();

        let is_confirmed = proceed_to_install == "y" || proceed_to_install.is_empty();

        Ok(Self {
            package: self.package.clone(),
            extracted_path: self.extracted_path.clone(),
            extracted_meta: self.extracted_meta.clone(),
            is_confirmed,
        })
    }

    fn install(&self) -> Result<(), Error> {
        let installed_package_meta = format!("{PACKAGE_INSTALL_PATH}{}", self.package.name);

        copy(self.extracted_meta.clone(), &installed_package_meta)?;
        remove_file(self.extracted_meta.clone())?;

        let mut project_meta_file = OpenOptions::new()
            .append(true)
            .open(installed_package_meta)?;

        project_meta_file.write_all(b"\n:::PATH DEFINE START:::\n")?;

        for path in self.package.paths.clone() {
            let source_path = format!("{}{path}", self.extracted_path.clone());

            if Path::new(&source_path).is_dir() {
                continue;
            } else {
                println!("    Copying \"{source_path}\" -> \"{path}\"");

                copy(source_path, &path)?;

                project_meta_file.write_all(format!("{path}\n").as_bytes())?;
            }
        }

        Ok(())
    }
}

pub fn spf_install(package_path: String) -> Result<(), Error> {
    println!("Loading package '{package_path}'...\n");

    let mut installer_information = Installer::load(&package_path)?;

    if !arch_is_compat(installer_information.package.metadata.clone())? {
        eprintln!(
            "Package architecture: {} doesn't match current system architecture ({ARCH}).\n\
            NOTE: Bypass by passing: `--ignore-arch`",
            installer_information.package.metadata.arch
        );

        installer_information.cleanup()?;

        return Ok(());
    }

    installer_information = installer_information.initialize()?;

    if installer_information.is_confirmed {
        println!("\nInstalling...\n");

        installer_information.install()?;
    } else {
        println!("\nAborted.");

        installer_information.cleanup()?;
    }

    installer_information.cleanup()?;

    println!(
        "\nDone! Installed '{}'!",
        installer_information.package.name
    );

    Ok(())
}

fn arch_is_compat(meta: Categories) -> Result<bool, Error> {
    if args_contains("--ignore-arch")? {
        return Ok(true);
    }

    let status = SUPPORTED_ARCHS.contains(&meta.arch.as_str());

    Ok(status)
}

enum InstallStatus {
    Install,
    Update,
    Downgrade,
    Reinstall,
}

impl InstallStatus {
    fn as_message(&self) -> &str {
        match &self {
            InstallStatus::Install => "install",
            InstallStatus::Update => "update",
            InstallStatus::Downgrade => "downgrade",
            InstallStatus::Reinstall => "reinstall",
        }
    }
}

impl Package {
    fn check_version(&self) -> Result<InstallStatus, Error> {
        if !self.is_already_installed() {
            return Ok(InstallStatus::Install);
        }

        let get_installed_version = || -> Result<String, Error> {
            let version = self.get_installed_meta()?.get_metadata()?.version;
            Ok(version)
        };

        let (package_version, installed_version) =
            (self.metadata.version.clone(), get_installed_version()?);

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

    fn get_installed_meta(&self) -> Result<Meta, Error> {
        Meta::from(&format!("{PACKAGE_INSTALL_PATH}{}", self.name))
    }

    fn is_already_installed(&self) -> bool {
        self.get_installed_meta().is_ok()
    }
}
