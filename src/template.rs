//! Provides the functions that allow a sample package metadata
//! config to be created.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env,
    fs::{self},
    path::Path,
    process::exit,
};

use crate::sys::error;

/// Sample metadata contents to be printed
static TEMPLATE_CONTENTS: &str = "\
# Header for project meta
# NOTE: META MUST BE DEFINED BEFORE PATHS
:::META DEFINE START:::
PROJECT_NAME = name
VERSION = v0.1.0
DESCRIPTION = Sample config
LICENSE = gplv3
AUTHORS = spf
ARCH = x86_64
:::META DEFINE END:::

# Paths to file
# Key:
# original/file/path:location/to/install
:::PATH DEFINE START:::
target/debug/spf:/usr/bin/spf
:::PATH DEFINE END:::
";

/// Generate template to chosen directory.
/// By default, file is generated to current directory
pub fn gen_meta_template(mut output_location: String) {
    if output_location.is_empty() {
        output_location = env::current_dir()
            .expect("Failed to get current directory")
            .to_str()
            .unwrap()
            .to_string();

    // Output location has to be a directory because rust doesn't want to
    // listen to me
    } else if !Path::new(&output_location).is_dir() {
        error("Please enter a directory to generate to.");
    }

    output_location = format!("{output_location}/spf_template");

    // Write the contents
    fs::write(&output_location, TEMPLATE_CONTENTS)
        .unwrap_or_else(|_| panic!("Failed to generate template to \"{output_location}\""));

    println!(
        "Generated template at: {}",
        output_location.replace("//spf_template", "/spf_template")
    );

    exit(0)
}
