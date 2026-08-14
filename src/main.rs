use crate::{
    install::spf_install, list::list_packages, package::create_spf_package,
    remove::remove_spf_package, sys::error, template::gen_meta_template,
};

// Shared
mod fs;
mod sys;

// Core
mod init;

// Commands
mod install;
mod list;
mod package;
mod remove;
mod template;

static VERSION: &str = "v0.2.0";

fn main() {
    init::init();

    // Keep this inside `main()` to prevent conflicts with other files
    static SOURCE_FILE: &str = "src/main.rs";

    // Intro
    println!("spf-{VERSION}\n");

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
            // Manually supply args because I hate this. Allows you to
            // remove multiple packages at once.
            let mut packages: Vec<String> = Vec::new();

            for package_arg in std::env::args().skip(2) {
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
        // Version is already mentioned at the top of this file (src/main.rs: line 22)
        "--version" | "-v" => println!(
            "Written by Owen Debiasio <owen.debiasio@gmail.com>. Licensed under GPL-3.0-or-later.\n\
            spf has NO WARRANTY and is not responsible for breaking your system."
        ),
        "" => available_commands(),
        _ => error(
            SOURCE_FILE,
            "main()",
            89,
            &format!("Invalid command: {root_arg}"),
        ),
    }
}

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
            \n\
            Available options:\n\
              --version   Display spf version"
        )
    )
}
