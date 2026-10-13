use std::io::Error;

use crate::{
    convert::convert,
    create::create_spf_package,
    fs::{ArchiveType, extract_archive},
    inspect::inspect,
    install::spf_install,
    remove::remove_packages,
};

/// Implementation of an spf package
pub struct Package {
    /// SPF package to inspect, create, or modify
    package_path: String,
}

impl Package {
    /// Extracts an spf package to a location.
    ///
    /// Since the package is a `tar` file, use [`extract_archive`] with
    /// [`ArchiveType::Tar`].
    ///
    /// ```
    /// let package_path = String::from("test/spf.spf");
    /// let package = Package::from(package_path)?;
    ///
    /// let destination = "./output";
    /// package.extract(destination)?;
    /// ```
    pub fn extract(&self, destination: &str) -> Result<(), Error> {
        extract_archive(&self.package_path, destination, ArchiveType::Tar)?;

        Ok(())
    }
}

impl Package {
    /// Loads an spf package.
    ///
    /// ```
    /// let package_path = String::from("test/spf.spf");
    /// let package = Package::from(package_path)?;
    /// ```
    pub fn from(package_path: String) -> Result<Self, Error> {
        let details = Self { package_path };

        Ok(details)
    }

    pub fn create(package_config: &str, output_location: &str) -> Result<(), Error> {
        create_spf_package(package_config, output_location)?;

        Ok(())
    }

    pub fn install(&self) -> Result<(), Error> {
        spf_install(self.package_path.clone())?;

        Ok(())
    }

    pub fn remove(collected_args: Vec<String>) -> Result<(), Error> {
        // Manually supply args because I hate this. Allows you to
        // remove multiple packages at once.
        let mut packages: Vec<String> = Vec::new();

        // The first two args that are skipped are `spf remove`. Everything
        // else after that is a package to check.
        for package_arg in collected_args.into_iter().skip(1) {
            packages.push(package_arg);
        }

        remove_packages(packages)?;

        Ok(())
    }

    pub fn inspect(package_path: &str) -> Result<(), Error> {
        inspect(package_path)?;

        Ok(())
    }

    /// Converts package formats.
    ///
    /// Supports conversion in the following orientations:
    ///
    /// - `.spf` -> `.deb`
    /// - `.deb` -> `.spf`
    /// - `.rpm` -> `.spf`
    /// - `.spf` -> `.rpm`
    ///
    /// ```
    /// let package_path = String::from("test/spf.spf");
    /// let package = Package::from(package_path)?;
    ///
    /// let output_file = "test.deb";
    /// package.convert(output_file)?;
    /// ```
    pub fn convert(&self, output_file: &str) -> Result<(), Error> {
        convert(&self.package_path, output_file)?;

        Ok(())
    }
}
