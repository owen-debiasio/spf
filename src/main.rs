use std::{io::Error, process::exit};

use crate::{
    convert::convert,
    list::list_packages,
    package::SpfPackage,
    sys::{error, return_args},
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

pub static VERSION: &str = "v0.7.0";

fn main() -> Result<(), Error> {
    // Intro
    println!("spf-{VERSION}\n");

    // Collect user args
    let mut collected_args = return_args()?;
    collected_args.retain(|arg| !matches!(arg.as_str(), "--ignore-args"));

    /*
    The first action after running `spf` in a cli:
    $ spf <root arg> <other actions>

    Note: I'm not sure why `.map_or` works, but it
    does, so I'm keeping it.
    */
    let root_arg = collected_args.first().map_or("", |a| a).to_string();
    let secondary_arg = collected_args.get(1).map_or("", |a| a).to_string();
    let tertiary_arg = collected_args.get(2).map_or("", |a| a).to_string();

    // Parse args
    // If there are more args than the root arg, pass them on to the desired function
    match root_arg.as_str() {
        "create" => SpfPackage::create(&secondary_arg, &tertiary_arg)?,

        "install" => SpfPackage::install(secondary_arg)?,

        "remove" => SpfPackage::remove(collected_args)?,

        "list" => list_packages(&secondary_arg)?,

        "template" => gen_meta_template(secondary_arg)?,

        "inspect" => SpfPackage::inspect(&secondary_arg)?,

        "convert" => convert(&secondary_arg, &tertiary_arg)?,

        // Version is already mentioned at the top of this file
        "--version" | "-v" => println!(
            "Written by Owen DeBiasio <owen.debiasio@gmail.com>. Licensed under GPL-3.0-or-later.\n\
            spf has NO WARRANTY and is not responsible for breaking your system."
        ),

        // If no args are provided, just show the usage menu
        "" => available_commands(),

        // If the arg provided isn't provided, throw error
        _ => error(&format!("Invalid command: {root_arg}")),
    }

    exit(0)
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
