use std::{fs::create_dir_all, path::Path};

use crate::{error, sys::is_root};

static SOURCE_FILE: &str = "src/init.rs";

pub fn init() {
    let paths_to_check = vec!["/usr/share/spf/packages/"];

    for path in paths_to_check {
        if !Path::new(path).exists() {
            if !is_root() {
                error(
                    SOURCE_FILE,
                    "init()",
                    11,
                    "In order to initialize the filesystem, you must run spf as root",
                )
            }

            create_dir_all(path).unwrap_or_else(|err| {
                error(
                    SOURCE_FILE,
                    "init()",
                    20,
                    &format!("Failed init spf: Failed to create directory \"{path}\": {err}"),
                )
            });
        }
    }
}
