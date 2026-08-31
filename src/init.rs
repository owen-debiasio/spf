//! Contains functions that control the init process of spf.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{fs::create_dir_all, path::Path};

use cmd_exists::cmd_exists;

use crate::{
    metadata::PACKAGE_INSTALL_PATH,
    sys::{error, get_binary_path, is_root},
};

/// Initializes what spf needs to function properly.
///
/// What it does currently:
///     - Check if needed paths exist
///     - Check if command `tar` is installed
///
/// If the build of spf is a debug build (located in `./target/`), skip.
/// Useful for github workflows.
pub fn init() {
    // Packages that spf needs to check so it can function
    let paths_to_check = vec![PACKAGE_INSTALL_PATH];

    // Check if the current build is a debug build or non-release build. Bypasses
    // root requirement.
    if !get_binary_path().contains("/target/") {
        // Go through the paths and make sure they exist. Otherwise, create them.
        for path in paths_to_check {
            // If a needed path doesn't exist.
            if !Path::new(path).exists() {
                // Make sure user is running as root
                if !is_root() {
                    error("In order to initialize the filesystem, you must run spf as root")
                }

                // Create the needed directory
                create_dir_all(path)
                    .unwrap_or_else(|_| panic!("Failed to extract archive \"{path}\""));
            }
        }
    }

    // Check if command `tar is installed`
    match cmd_exists("tar") {
        Ok(()) => (),
        Err(_) => error("Command \"tar\" not found! Please install it!"),
    }
}
