use std::{
    env::{current_exe, var},
    process::exit,
};

static SOURCE_FILE: &str = "src/sys.rs";

pub struct Env;

impl Env {
    fn home() -> String {
        var("HOME").unwrap_or_else(|err| {
            error(
                SOURCE_FILE,
                "Env::home()",
                13,
                &format!("Failed to retrieve user home: {err}"),
            )
        })
    }

    fn name() -> String {
        var("USER").unwrap_or_else(|err| {
            error(
                SOURCE_FILE,
                "Env::name()",
                24,
                &format!("Failed to retrieve user name: {err}"),
            )
        })
    }
}

/// Detects if user is running as root. Returns true or false. That is all.
pub fn is_root() -> bool {
    if Env::name() == "root" || Env::home().contains("/root") {
        return true;
    }

    false
}

/// Display an error. When called, exit with status 1
pub fn error(source_file: &str, function: &str, error_call_line: usize, message: &str) -> ! {
    eprintln!(
        "{source_file}:{function}:{message}\n\
        Error called on line: {error_call_line}"
    );

    exit(1)
}

/// Does what you think it does. Retrieves the current path of the running
/// `spf` binary.
pub fn get_binary_path() -> String {
    current_exe()
        .unwrap_or_else(|err| {
            error(
                SOURCE_FILE,
                "get_binary_path()",
                55,
                &format!("Failed to get binary path: {err}"),
            )
        })
        .to_str()
        .unwrap()
        .to_string()
}
