//! Contains functions that control the init process of spf.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use cmd_exists::cmd_exists;

use crate::sys::error;

/// Initializes what spf needs to function properly.
///
/// What it does currently:
///     - Check if commands `tar` and `ar` is installed
///
/// If the build of spf is a debug build (located in `./target/`), skip.
/// Useful for github workflows.
pub fn init() -> Result<(), std::io::Error> {
    match cmd_exists("tar") {
        Ok(()) => (),
        Err(err) => error(&format!(
            "Command \"tar\" not found! Please install it! Details: {err}"
        )),
    };

    match cmd_exists("ar") {
        Ok(()) => (),
        Err(err) => error(&format!(
            "Command \"ar\" not found! Please install it! Details: {err}"
        )),
    }

    Ok(())
}
