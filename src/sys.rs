use std::{
    env::{current_exe, var},
    process::exit,
};

static SOURCE_FILE: &str = "src/sys.rs";

/// Provides env variables that are used in various situations
/// (Ex. detecting if user is root **(See `is_root()`)**).
///
/// Outputs are returned as `String`.
pub struct Env;

impl Env {
    /// Retrieves the user's `HOME` directory
    pub fn home() -> String {
        var("HOME").unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "Env::home()",
                18,
                &format!("Failed to retrieve user home: {err}"),
            )
        })
    }

    /// Retrieves the user's username
    pub fn name() -> String {
        var("USER").unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "Env::name()",
                30,
                &format!("Failed to retrieve user name: {err}"),
            )
        })
    }
}

/// Detects if user is running as root. Returns true or false. That is all.
///
/// Returns `true` or `false`.
///
/// It checks if the home directory or username is associated with `root`.
pub fn is_root() -> bool {
    if Env::name() == "root" || Env::home().contains("/root") {
        return true;
    }

    false
}

/// Provide types of errors:
/// - `fatal`: Error that really shouldn't occur (usually means there might be a bug)
///   - Requires source file (`source_file`), source function (`function`), where
///     the error is called (`error_call_line`), and the message of the error (`message`).
/// - `normal`: Errors that are usually expected if the user does something wrong (such as
///   the user forgetting to input a .spf package to install)
///     - Requires a message to let the user know what's wrong (`message`)
///
/// Both exit with status `1`, and return `!`.
pub struct Error;

impl Error {
    /// Alert user that the program itself has encountered a serious error
    pub fn fatal(source_file: &str, function: &str, error_call_line: usize, message: &str) -> ! {
        eprintln!(
            "FATAL: {source_file}::{function}: {message}\n\
            Error called on line: {error_call_line}"
        );

        exit(1)
    }

    /// Alert user that they likely did something wrong
    pub fn normal(message: &str) -> ! {
        eprintln!("{message}");

        exit(1)
    }
}

/// Does what you think it does. Retrieves the current path of the running
/// `spf` binary. Returns as `String`.
pub fn get_binary_path() -> String {
    current_exe()
        .unwrap_or_else(|err| {
            Error::fatal(
                SOURCE_FILE,
                "get_binary_path()",
                88,
                &format!("Failed to get binary path: {err}"),
            )
        })
        .to_str()
        .unwrap()
        .to_string()
}
