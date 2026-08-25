//! Shared functions and variables that assist with returning system properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env::{current_exe, var},
    process::exit,
};

/// Provides env variables that are used in various situations
/// (Ex. detecting if user is root **(See `is_root()`)**).
///
/// Outputs are returned as `String`.
pub struct Env;

impl Env {
    /// Retrieves the user's `HOME` directory
    pub fn home() -> String {
        var("HOME").expect("Failed to retrieve env var \"USER\"")
    }

    /// Retrieves the user's username
    pub fn name() -> String {
        var("USER").expect("Failed to retrieve env var \"USER\"")
    }
}

/// Detects if user is running as root. Returns true or false. That is all.
///
/// Returns `true` or `false`.
///
/// It checks if the home directory or username is associated with `root`.
pub fn is_root() -> bool {
    if Env::name() == "root" || Env::home().contains("/root") {
        return true;
    }

    false
}

/// Basic non-fatal error.
///
/// The error message is the provided argument `message`.
///
/// Exits with the provided code (`code`)
pub fn error(message: &str) -> ! {
    eprintln!("{message}");

    exit(1)
}

/// Does what you think it does. Retrieves the current path of the running
/// `spf` binary. Returns as `String`.
pub fn get_binary_path() -> String {
    current_exe()
        .expect("Failed to get binary path")
        .to_str()
        .unwrap()
        .to_string()
}
