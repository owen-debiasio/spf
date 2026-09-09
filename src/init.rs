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
///     - Check if command `tar` is installed
///
/// If the build of spf is a debug build (located in `./target/`), skip.
/// Useful for github workflows.
pub fn init() -> Result<(), std::io::Error> {
    match cmd_exists("tar") {
        Ok(()) => Ok(()),
        Err(err) => error(&format!("Command \"tar\" not found! Please install it! Details: {err}")),
    }

    match cmd_exists("ar") {
        Ok(()) -> Ok(()),
        Err(err) => error(&format!("Command \"ar\" not found! Please install it! Details: {err}"))
    }
}

