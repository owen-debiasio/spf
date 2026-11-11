use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use crate::libs::spf::{
    metadata::{Meta, PACKAGE_INSTALL_PATH},
    package::Package,
};

impl Package {
    pub fn get_install_location(&self) -> Result<String, Error> {
        let location = format!("{PACKAGE_INSTALL_PATH}/{}", self.name.clone());

        Ok(location)
    }

    pub fn get_installed_paths(&self) -> Result<Vec<String>, Error> {
        let mut paths: Vec<String> = Vec::new();

        for path in Meta::from(&self.package_path)?.get_paths()? {
            paths.push(path)
        }

        Ok(paths)
    }

    pub fn from_installed(name: String) -> Result<Self, Error> {
        let installed_path = format!("{PACKAGE_INSTALL_PATH}{name}");

        if !Path::new(&installed_path).exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Package not installed: {name}"),
            ));
        }

        let metadata = Meta::from(&installed_path)?.get_metadata()?;

        let mut package = Self::new(name).add_metadata(metadata);

        package.package_path = package.get_install_location()?;

        for path in package.get_installed_paths()? {
            package.paths.push(path);
        }

        Ok(package)
    }

    pub fn is_already_installed(&self) -> bool {
        Path::new(&self.package_path).exists()
    }
}
