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

        let meta_path = if package.starts_with(PACKAGE_INSTALL_PATH) && Path::new(package).exists()
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

    fn prep_meta(&self) -> Result<Self, Error> {
        let initial_metadata = Meta::from(&self.meta_path)?;

        let loaded_metadata =
            Meta::construct_contents(initial_metadata.get_metadata()?)?.join("\n");

        Ok(Self {
            package: self.package.clone(),
            meta_path: self.meta_path.clone(),
            remove_extracted: self.remove_extracted,
            extracted_path: self.extracted_path.clone(),
            constructed_meta: loaded_metadata,
            initial_metadata,
            paths: String::new(),
        })
    }

    fn prep_paths(&self) -> Result<Self, Error> {
        let mut paths = if self.remove_extracted {
            let paths = self.package.as_extracted().get_packaged_fs()?.join("\n");

            remove_dir_all(&self.extracted_path)?;

            paths
        } else {
            self.initial_metadata.get_paths()?.join("\n")
        };

        paths = paths
            .replace(&self.extracted_path, "")
            .replace("/META", "")
            .trim_start()
            .to_string();

        Ok(Self {
            package: self.package.clone(),
            meta_path: self.meta_path.clone(),
            remove_extracted: self.remove_extracted,
            extracted_path: self.extracted_path.clone(),
            constructed_meta: self.constructed_meta.clone(),
            initial_metadata: self.initial_metadata.clone(),
            paths,
        })
    }
}

pub fn inspect(package: &str) -> Result<(), Error> {
    println!("Loading package '{package}'...");

    let loaded_package = Inspector::load(package)?;

    let metadata_categories = loaded_package.prep_meta()?.constructed_meta;
    println!("\nInstalled metadata:\n{metadata_categories}");

    let installed_paths = loaded_package.prep_paths()?.paths;
    println!("\nInstalled paths:\n{installed_paths}");

    Ok(())
}
