use std::{ffi::OsStr, fs::remove_dir_all, io::Error, path::Path};

use crate::libs::spf::{
    metadata::{Meta, PACKAGE_INSTALL_PATH},
    package::Package,
};

pub fn inspect(package: &str) -> Result<(), Error> {
    println!("Loading package '{package}'...");

    let mut remove_extracted: bool = false;
    let extracted = Path::new(&package.trim_end_matches("spf"))
        .file_stem()
        .unwrap_or_default()
        .display()
        .to_string();

    let meta_path = if package.starts_with(PACKAGE_INSTALL_PATH) && Path::new(package).exists() {
        package
    } else if Path::new(package).extension().unwrap_or(OsStr::new("")) == OsStr::new("spf") {
        Package::from(package.to_string())?.extract(".")?;
        remove_extracted = true;
        &format!("{extracted}/META")
    } else {
        &format!("{PACKAGE_INSTALL_PATH}/{package}")
    };

    let initial_metadata = Meta::from(meta_path)?;

    let metadata_categories =
        Meta::construct_contents(initial_metadata.get_metadata()?)?.join("\n");
    println!("\nInstalled metadata:\n{metadata_categories}");

    if !remove_extracted {
        let installed_paths = initial_metadata.get_paths()?.join("\n");
        println!("\nInstalled paths:\n{installed_paths}");
    }

    if remove_extracted {
        remove_dir_all(extracted)?;
    }

    Ok(())
}
