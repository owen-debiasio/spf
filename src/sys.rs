use std::{
    env::{current_exe, var},
    process::exit,
};

pub struct Env;

impl Env {
    fn home() -> String {
        var("HOME").unwrap_or_else(|err| error(&format!("Failed to retrieve user home: {err}")))
    }

    fn name() -> String {
        var("USER").unwrap_or_else(|err| error(&format!("Failed to retrieve user name: {err}")))
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
pub fn error(message: &str) -> ! {
    eprintln!("{message}");

    exit(1)
}

pub fn get_binary_path() -> String {
    current_exe()
        .unwrap_or_else(|err| error(&format!("Failed to get binary path: {err}")))
        .to_str()
        .unwrap()
        .to_string()
}
