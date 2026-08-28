use std::process::exit;

use crate::{
    inspect::inspect,
    install::spf_install,
    list::list_packages,
    package::create_spf_package,
    remove::remove_spf_package,
    sys::{error, return_args},
    template::gen_meta_template,
};

// Shared
mod fs;
mod metadata;
mod sys;

// Core
mod init;

// Commands
mod inspect;
mod install;
mod list;
mod package;
mod remove;
mod template;

static VERSION: &str = "v0.5.1";

fn main() {
    init::init();

    // Intro
    println!("spf-{VERSION}\n");

    // Collect user args
    let mut collected_args = return_args();
    collected_args.retain(|arg| !matches!(arg.as_str(), "--ignore-args"));

    /*
    The first action after running `spf` in a cli:
    $ spf <root arg> <other actions>

    Note: I'm not sure why `.map_or` works, but it
    does so I'm keeping it.
    */
    let root_arg = collected_args.first().map_or("", |a| a).to_string();
    let secondary_arg = collected_args.get(1).map_or("", |a| a).to_string();
    let tertiary_arg = collected_args.get(2).map_or("", |a| a).to_string();

    // Parse args
    // If there are more args than the root arg, pass them on to the desired function
    match root_arg.as_str() {
        // Create package
        "create" | "-c" => {
            // `secondary_arg` is the file with the list of paths to package
            create_spf_package(&secondary_arg, &tertiary_arg)
        }
        // Install package
        "install" | "-i" => {
            // `secondary_arg` is the package to install
            spf_install(secondary_arg)
        }
        "remove" | "-r" => {
            // Manually supply args because I hate this. Allows you to
            // remove multiple packages at once.
            let mut packages: Vec<String> = Vec::new();

            // The first two args that are skipped are `spf remove`. Everything
            // else after that is a package to check.
            for package_arg in collected_args.into_iter().skip(1) {
                // Don't process args
                if package_arg.starts_with('-') {
                    continue;
                }

                packages.push(package_arg);
            }

            remove_spf_package(packages);
        }
        "list" | "-l" => {
            // `secondary_arg` is the optional string to search
            list_packages(secondary_arg);
        }

        "template" | "-t" => {
            // `secondary_arg` is the optional output location
            gen_meta_template(secondary_arg);
        }

        "inspect" | "-is" => {
            // `secondary_arg` is the package to inspect
            inspect(secondary_arg);
        }

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
        "{}",
        format_args!(
            "\
            Available Commands:\n\n\
              create     <metadata file> <output directory>   Create package\n\
              install    <.spf package location>              Install package\n\
              remove     <package to uninstall>               Uninstall package\n\
              list       <(optional) string to match>         List installed packages\n\
              template   <(optional) output location>         Generate package metadata template\n\
              inspect    <package to inspect>                 Inspect metadata of a package
            \n\
            Available options:\n\
              --version       Display spf version\n\
              --ignore-args   Force the installation of a package with a different architecture"
        )
    )
}
