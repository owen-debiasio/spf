use std::io::Error;

use crate::{
    convert::convert,
    create::create_spf_package,
    fs::{ArchiveType, extract_archive},
    inspect::inspect,
    install::spf_install,
    remove::remove_packages,
};

pub struct Package {
    package_path: String,
}

#[allow(clippy::new_ret_no_self)]
impl Package {
    pub fn from(package_path: String) -> Result<Self, Error> {
        let details = Self { package_path };

        Ok(details)
    }

    pub fn extract(&self) -> Result<(), Error> {
        extract_archive(&self.package_path, ".", ArchiveType::Tar)?;

        Ok(())
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
            // Don't process args
            if package_arg.starts_with('-') {
                continue;
            }

            packages.push(package_arg);
        }

        remove_packages(packages)?;

        Ok(())
    }

    pub fn inspect(package_path: &str) -> Result<(), Error> {
        inspect(package_path)?;

        Ok(())
    }

    pub fn convert(&self, output_file: &str) -> Result<(), Error> {
        convert(&self.package_path, output_file)?;
        Ok(())
    }
}
