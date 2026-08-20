//! Contains functions that control the init process of spf.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{fs::create_dir_all, path::Path};

use crate::{
    metadata::PACKAGE_INSTALL_PATH,
    sys::{Error, is_root},
};

static SOURCE_FILE: &str = "src/init.rs";

/// Initializes what spf needs to function properly.
///
/// What it does currently:
///     - Check if needed paths exist (see `paths_to_check`)
pub fn init() {
    // Packages that spf needs to check so it can function
    let paths_to_check = vec![PACKAGE_INSTALL_PATH];

    // Go through the paths and make sure they exist. Otherwise, create them.
    for path in paths_to_check {
        // If a needed path doesn't exist.
        if !Path::new(path).exists() {
            // Make sure user is running as root
            if !is_root() {
                Error::normal("In order to initialize the filesystem, you must run spf as root")
            }

            // Create the needed directory
            create_dir_all(path).unwrap_or_else(|err| {
                Error::fatal(
                    SOURCE_FILE,
                    "init()",
                    34,
                    &format!("Failed init spf: Failed to create directory \"{path}\": {err}"),
                )
            });
        }
    }
}
