//! Shared functions and variables that assist with returning system properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env::{args, current_exe, var},
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

/// Determine if the args that have been retrieved from [`args_contains`] contains
/// what the user wants to find (`arg` (as [`str`])).
///
/// Returns as bool.
///
/// Example: Arg list contains matching input arg:
/// ```
/// // Arg list: --arga, argb, -c
///
/// let arg_to_find = "argb";
///
/// let does_arg_contain = args_contains(arg_to_find);
///
/// // Output should be `true`
/// println!("{does_arg_contain}");
/// ```
///
/// Example: Arg list does not contain input arg:
/// ```
/// // Arg list: --arga, argb, -c
///
/// let arg_to_find = "abcd";
///
/// let does_arg_contain = args_contains(arg_to_find);
///
/// // Output should be `false`
/// println!("{does_arg_contain}");
/// ```
pub fn args_contains(arg: &str) -> bool {
    return_args().contains(&arg.to_string())
}

/// Returns collected args that have been supplied.
///
/// Returns the following types of args:
///     - "-a"
///     - "--arg"
///     - "arg"
///
/// Allows you to combine args like this:
///     `$ spf -a --ard arg -bcd`
///
/// Returns as this:
///     `[spf, -a, -ard, arg, -b, -c, -d]`
///
/// Returns as [`Vec<String>`].
///
/// ```
/// let collected_args = return_args();
/// ```
pub fn return_args() -> Vec<String> {
    let mut collected_args = Vec::new();

    for given_arg in args().skip(1) {
        // If the arg is a normal arg (`--arg`)
        if given_arg.starts_with("--") {
            collected_args.push(given_arg);

        // If the arg is a small arg (`-a`)
        } else if given_arg.starts_with('-') && given_arg != "-" {
            for character in given_arg.chars().skip(1) {
                collected_args.push(format!("-{character}"));
            }
        // If the arg is a normal arg (`arg`)
        } else {
            collected_args.push(given_arg);
        }
    }

    collected_args
}

/// A list of architectures that exist.
///
/// Stored as [[`str`]; 20]
pub static LIST_OF_ARCHS: [&str; 20] = [
    "x86",
    "x86_64",
    "arm",
    "aarch64",
    "m68k",
    "mips",
    "mips32r6",
    "mips64",
    "mips64r6",
    "csky",
    "powerpc",
    "powerpc64",
    "riscv32",
    "riscv64",
    "s390x",
    "sparc",
    "sparc64",
    "hexagon",
    "loongarch32",
    "loongarch64",
];
