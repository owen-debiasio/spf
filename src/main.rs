//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{io::Error, process::exit};

use crate::cli::{
    args::{Args, IN_DEBUG_MODE, return_args},
    commands::{
        create::create, inspect::inspect, install::spf_install, list::list_packages,
        remove::remove_packages, template::gen_meta_template,
    },
};

mod cli;
mod libs;

/// Retrieve spf version as a minimal string
///
/// ```
/// let spf_version = VERSION;
/// println!("{spf_version}")
/// ```
pub static VERSION: &str = "v0.8.0";

fn main() -> Result<(), Error> {
    // Intro
    println!("spf-{VERSION}\n");

    let all = return_args()?;

    let as_str_and_starts_with_dash = |thing: &String| -> bool { thing.as_str().starts_with("-") };

    // Parse commands, strip flags
    let mut commands = all.clone();
    commands.retain(|arg| !as_str_and_starts_with_dash(arg));

    // Parse commands, strip flags
    let mut flags = all.clone();
    flags.retain(|arg| as_str_and_starts_with_dash(arg));

    let make_str =
        |thing: Option<&String>| -> String { thing.unwrap_or(&String::new()).to_string() };

    let collected_args = Args {
        commands: commands.clone(),
        flags,
        root: make_str(commands.first()),
        second: make_str(commands.get(1)),
        third: make_str(commands.get(2)),
    };

    if *IN_DEBUG_MODE {
        println!("WARNING: Debug mode on!\n");

        println!("DEBUG: Collected args: {collected_args:?}\n")
    }

    match parse_spf_cli(collected_args.clone()) {
        Ok(()) => exit(0),
        Err(err) => {
            if *IN_DEBUG_MODE {
                eprintln!(
                    "\nDEBUG: Failed to run command '{}': {err}",
                    collected_args.root
                );
            }

            exit(1)
        }
    }
}

/// Parses arguments passed, and maps them to the available commands and actions
/// in spf.
///
/// ```
/// let args = vec![String::from("arg1"), String::from("arg1"), String::from("arg1")];
///
/// let collected_args = Args {
///    all: args,
///    root: args.first(),
///    second: args.get(1),
///    third: args.get(2),
/// };
///
/// parse_spf_cli(collected_args)?;
/// ```
fn parse_spf_cli(collected_args: Args) -> Result<(), Error> {
    match collected_args.root.as_str() {
        "create" => create(collected_args.second, collected_args.third)?,

        "install" => spf_install(collected_args.second)?,

        "remove" => {
            // Manually supply args because I hate this. Allows you to
            // remove multiple packages at once.
            let mut packages: Vec<String> = Vec::new();

            // The first two args that are skipped are `spf remove`. Everything
            // else after that is a package to check.
            for package_arg in collected_args.commands.into_iter().skip(1) {
                packages.push(package_arg);
            }

            remove_packages(packages)?
        }

        "list" => list_packages(&collected_args.second)?,

        "template" => gen_meta_template(collected_args.second)?,

        "inspect" => inspect(&collected_args.second)?,

        _ => {
            let flags = collected_args.flags;
            if flags.contains(&"--version".to_string()) {
                println!(
                    "Written by Owen DeBiasio <owen.debiasio@gmail.com>. Licensed under GPL-3.0-or-later.\n\
                    spf has NO WARRANTY and is not responsible for breaking your system."
                )
            } else {
                available_commands()
            }
        }
    }

    Ok(())
}

/// All this does is list the available commands, flags, and args for spf. Does nothing else.
fn available_commands() {
    println!(
        "\
        Available Commands:\n\n  \
          create     <metadata file> <output directory>    Create package\n  \
          install    <.spf package location>               Install package\n  \
          remove     <package to uninstall>                Uninstall package\n  \
          list       <(optional) string to match>          List installed packages\n  \
          template   <(optional) output location>          Generate package metadata template\n  \
          inspect    <package to inspect>                  Inspect metadata of a package
        \n\
        Available options:\n\n \
          --version            Display spf version\n \
          --ignore-arch        Force the installation of a package with a different architecture\n \
          --debug              Display debug information as spf runs"
    );
}
