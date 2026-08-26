//! Shared functions and variables that assist with returning system properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env::{current_exe, var},
    process::exit,
};

/// Provides env variables that are used in various situations
/// (Ex. detecting if user is root (See [`is_root()`])).
///
/// Outputs are returned as [`String`].
///
/// Example: Retrieve the user's `HOME` directory
///
/// ```
/// let home = Env::home();
///
/// // Displays whatever your home dir is (something like `/home/username/`)
/// println!("{home}");
/// ```
///
/// Example: Retrieve the user's name
/// ```
/// let username = Env::name();
///
/// // Displays whatever your username is.
/// println!("{username}");
/// ```
pub struct Env;

impl Env {
    /// Retrieves the user's `HOME` directory
    ///
    /// ```
    /// let home = Env::home();
    ///
    /// // Displays whatever your home dir is (something like `/home/username/`)
    /// println!("{home}");
    /// ```
    pub fn home() -> String {
        var("HOME").expect("Failed to retrieve env var \"USER\"")
    }

    /// Retrieves the user's username
    ///
    /// ```
    /// let username = Env::name();
    ///
    /// // Displays whatever your username is.
    /// println!("{username}");
    /// ```
    pub fn name() -> String {
        var("USER").expect("Failed to retrieve env var \"USER\"")
    }
}

/// Detects if user is running as root by utilizing [`Env`], by checking if
/// the home directory or username is associated with `root`.
///
/// It checks if the current username is `root` by checking [`Env::name()`],
/// and checks the home directory by checking [`Env::home`].
///
/// Returns true or false.
///
/// ```
/// // User is root (username is `root` or home dir contains `root`)
/// if is_root() {
///     println!("I am root")
/// }
///
/// // User isn't root (user is not associated with root)
/// if !is_root() {
///     println!("I am not root")
/// }
/// ```
pub fn is_root() -> bool {
    if Env::name() == "root" || Env::home().contains("/root") {
        return true;
    }

    false
}

/// Basic non-fatal error.
///
/// The error message is the provided argument `message` (provided as [`str`]),
/// returns as `!`.
///
/// Exits with code `1`.
///
/// ```
/// let error_message = "error_message";
/// error(error_message);
///
/// // No further code is run.
/// ```
pub fn error(message: &str) -> ! {
    eprintln!("{message}");

    exit(1)
}

/// Does what you think it does. Retrieves the current path of the running
/// `spf` binary. Returns as [`String`].
///
/// ```
/// let binary_path = get_binary_path();
///
/// println!("{binary_path}")
///
/// // From a proper installation of spf, the output should be:
/// // `/usr/bin/spf`
/// ```
pub fn get_binary_path() -> String {
    current_exe()
        .expect("Failed to get binary path")
        .to_str()
        .unwrap()
        .to_string()
}
