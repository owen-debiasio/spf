//! Shared functions and variables that assist with returning system properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env::{current_exe, var},
    io::{Error, ErrorKind},
};

use crate::cli::commands::convert::PackageType;

/// Provides env variables that are used in various situations
/// (Ex. detecting if user is root (See [`is_root()`])).
///
/// Outputs are returned as [`String`].
///
/// Example: Retrieve the user's `HOME` directory
///
/// ```
/// let home = Env::home()?;
///
/// // Displays whatever your home dir is (something like `/home/username/`)
/// println!("{home}");
/// ```
///
/// Example: Retrieve the user's name
/// ```
/// let username = Env::name()?;
///
/// // Displays whatever your username is.
/// println!("{username}");
/// ```
pub struct Env;

impl Env {
    /// Retrieves the user's `HOME` directory
    ///
    /// ```
    /// let home = Env::home()?;
    ///
    /// // Displays whatever your home dir is (something like `/home/username/`)
    /// println!("{home}");
    /// ```
    pub fn home() -> Result<String, Error> {
        Ok(var("HOME").unwrap_or_default())
    }

    /// Retrieves the user's username
    ///
    /// ```
    /// let username = Env::name()?;
    ///
    /// // Displays whatever your username is.
    /// println!("{username}");
    /// ```
    pub fn name() -> Result<String, Error> {
        Ok(var("USER").unwrap_or_default())
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
/// if is_root()? {
///     println!("I am root")
/// }
///
/// // User isn't root (user is not associated with root)
/// if !is_root()? {
///     println!("I am not root")
/// }
/// ```
pub fn is_root() -> Result<bool, Error> {
    let identifies_as_root = Env::name()? == "root" || Env::home()?.contains("/root");

    Ok(identifies_as_root)
}

/// Does what you think it does. Retrieves the current path of the running
/// `spf` binary. Returns as [`String`].
///
/// ```
/// let binary_path = get_binary_path()?;
///
/// println!("{binary_path}")
///
/// // From a proper installation of spf, the output should be:
/// // `/usr/bin/spf`
/// ```
pub fn get_binary_path() -> Result<String, Error> {
    let path = current_exe()?.display().to_string();

    Ok(path)
}

/// A list of supported architectures.
///
/// Entry `universal` means to work on any system.
///
/// Stored as [[`str`]; 5]
pub static SUPPORTED_ARCHS: [&str; 5] = ["universal", "x86", "x86_64", "arm", "aarch64"];

/// Converts the package architecture type to another packages.
///
/// - `input_arch` ([str]) is the architecture you want converted
/// - `output_package_type` ([PackageType]) is the output format.
///
/// Returns as [str]
///
/// Example: Convert `.spf` arch to `.deb` equivalent
/// ```
/// let arch_to_convert = "x86_64";
/// let output_type = PackageType::Deb;
///
/// let converted_arch = convert_arch(arch_to_convert, output_type)?;
///
/// assert_eq!("amd64", converted_arch);
/// ```
pub fn convert_arch(input_arch: &str, output_package_type: PackageType) -> Result<&str, Error> {
    let arch_convert_error = Err(Error::new(
        ErrorKind::InvalidData,
        format!("Invalid architecture (can't convert): {input_arch}"),
    ));

    match output_package_type {
        // .deb/.rpm -> .spf
        PackageType::Spf => match input_arch {
            "all" | "noarch" | "src" | "nosrc" => Ok("universal"),
            "amd64" | "x86_64" => Ok("x86_64"),
            "i386" | "i686" => Ok("x86"),
            "arm64" | "aarch64" => Ok("aarch64"),
            "armhf" | "armv7hl" | "armvhl" => Ok("arm"),
            _ => arch_convert_error,
        },
        // .spf/.rpm -> .deb
        PackageType::Deb => match input_arch {
            "universal" | "noarch" | "src" | "nosrc" => Ok("all"),
            "x86_64" => Ok("amd64"),
            "x86" | "i386" | "i686" => Ok("i386"),
            "aarch64" => Ok("arm64"),
            "arm" | "armv7hl" | "armvhl" => Ok("armhf"),
            _ => arch_convert_error,
        },
        // .spf/.deb -> .rpm
        PackageType::Rpm => match input_arch {
            "universal" | "all" => Ok("noarch"),
            "x86_64" => Ok("amd64"),
            "x86" | "i386" => Ok("i686"),
            "aarch64" | "arm64" => Ok("aarch64"),
            "arm" | "armhf" => Ok("armv7hl"),
            _ => arch_convert_error,
        },
    }
}
