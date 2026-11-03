//! Manager the parsed args for the spf cli.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{env::args, io::Error};

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
pub fn args_contains(arg: &str) -> Result<bool, Error> {
    let args = return_args()?.contains(&arg.to_string());

    Ok(args)
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
pub fn return_args() -> Result<Vec<String>, Error> {
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

    Ok(collected_args)
}

/// Defines the collected args passed through the CLI.
///
/// All are stored as [`String`].
///
/// ```
/// let args = vec![String::from("arg1"), String::from("--flag"), String::from("arg1"), String::from("arg1")];
///
/// let collected_args = Args {
///    commands: vec![args.first(), args.get(2), args.get(3)],
///    flags: vec![args.get(1)]
///    root: args.first(),
///    second: args.get(2),
///    third: args.get(3),
/// };
///
/// // Verify groups (commands and flags)
/// assert_eq!(vec![args.first(), args.get(2), args.get(3)], collected_args.commands);
/// assert_eq!(vec![args.get(1)], collected_args.flags);
///
/// // Verify individual commands
/// assert_eq!(args.first(), collected_args.root);
/// assert_eq!(args.get(2), collected_args.second);
/// assert_eq!(args.get(3), collected_args.third);
/// ```
#[derive(Debug)]
pub struct Args {
    /// Commands, subcommands, etc
    pub commands: Vec<String>,

    /// Flags such as `--version`
    pub flags: Vec<String>,

    /// The root action (Passed arg #1)
    pub root: String,

    /// Passed arg #2
    pub second: String,

    /// Passed arg #3
    pub third: String,
}
