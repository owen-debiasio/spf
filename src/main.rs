use std::process::exit;

use crate::{
    install::spf_install, list::list_packages, package::create_spf_package,
    remove::remove_spf_package,
};

mod fs;
mod init;
mod install;
mod list;
mod package;
mod remove;

static VERSION: &str = "v0.1.0";

/// Display an error. When called, exit with status 1
pub fn error(message: &str) -> ! {
    eprintln!("{message}");

    exit(1)
}

fn main() {
    init::init();

    // Intro
    println!("spf {VERSION}\n");

    // Collect user args
    let mut user_args: Vec<String> = Vec::new();
    for arg in std::env::args().skip(1) {
        user_args.push(arg);
    }

    let get_arg =
        |arg: usize| -> String { user_args.get(arg).unwrap_or(&String::new()).to_string() };

    // The first action after running `spf` in a cli:
    // $ spf <root arg> <other actions>
    let root_arg = get_arg(0);
    let secondary_arg = get_arg(1);
    let tertiary_arg = get_arg(2);

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
            // `secondary_arg` is the package to remove
            remove_spf_package(&secondary_arg);
        }
        "list" | "-l" => {
            // `secondary_arg` is the optional string to search
            list_packages(secondary_arg);
        }
        "" => available_commands(),
        _ => error(&format!("Invalid command: {root_arg}")),
    }
}

fn available_commands() {
    println!(
        "{}",
        format_args!(
            "\
            Available Commands:\n\n\
              create    <list of entries in text file> <output directory>\n\
              install   <.spf package location>\n\
              remove    <package to uninstall>\n\
              list      <(optional) string to match>"
        )
    )
}
