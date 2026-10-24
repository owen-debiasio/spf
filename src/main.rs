use std::{io::Error, process::exit};

use crate::{
    list::list_packages,
    package::Package,
    sys::{Args, error, return_args},
    template::gen_meta_template,
};

// Shared
mod fs;
mod metadata;
mod sys;

mod package;

// Commands
mod convert;
mod create;
mod inspect;
mod install;
mod list;
mod remove;
mod template;

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

    // Collect user args
    let mut parsed_args = return_args()?;
    parsed_args.retain(|arg| !arg.as_str().starts_with("-"));

    let collected_args = Args {
        all: parsed_args.clone(),
        root: parsed_args.first().unwrap_or(&String::new()).to_string(),
        second: parsed_args.get(1).unwrap_or(&String::new()).to_string(),
        third: parsed_args.get(2).unwrap_or(&String::new()).to_string(),
    };

    parse_spf_cli(collected_args)?;

    exit(0)
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
        "create" => Package::create(&collected_args.second, &collected_args.third)?,

        "install" => Package::from(collected_args.second)?.install()?,

        "remove" => Package::remove(collected_args.all)?,

        "list" => list_packages(&collected_args.second)?,

        "template" => gen_meta_template(collected_args.second)?,

        "inspect" => Package::inspect(&collected_args.second)?,

        "convert" => Package::from(collected_args.second)?.convert(&collected_args.third)?,

        // Version is already mentioned at the top of this file
        "--version" | "-v" => println!(
            "Written by Owen DeBiasio <owen.debiasio@gmail.com>. Licensed under GPL-3.0-or-later.\n\
            spf has NO WARRANTY and is not responsible for breaking your system."
        ),

        // If no args are provided, just show the usage menu
        "" => available_commands(),

        // If the arg provided isn't provided, throw error
        _ => error(&format!("Invalid command: {}", collected_args.root)),
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
          inspect    <package to inspect>                  Inspect metadata of a package\n  \
          convert    <source package> <converted format>   Convert package formats
        \n\
        Available options:\n\n \
          --version       Display spf version\n \
          --ignore-arch   Force the installation of a package with a different architecture"
    );
}
