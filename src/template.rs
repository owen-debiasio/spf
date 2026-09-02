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

/// Sample metadata contents to be printed. Every possible feature
/// of the metadata is included.
///
/// Stored as [`str`]
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
///
/// The template itself is stored in [`TEMPLATE_CONTENTS`] as [`str`].
///
/// `output_location`, stored as [`String`], must be a directory. If
/// the output location isn't a directory, [`gen_meta_template`] will
/// panic.
///
/// ```
/// let location_to_send_template = "~/Documents/"
///
/// gen_meta_template(location_to_send_template)
///
/// // Output location is `~/Documents/spf_template`
/// ```
pub fn gen_meta_template(mut output_location: String) -> Result<(), std::io::Error> {
    if output_location.is_empty() {
        output_location = env::current_dir()?.to_str().unwrap_or_default().to_string();

    // Output location has to be a directory because rust doesn't want to
    // listen to me
    } else if !Path::new(&output_location).is_dir() {
        error("Please enter a directory to generate to.");
    }

    output_location = format!("{output_location}/spf_template");

    // Write the contents
    fs::write(&output_location, TEMPLATE_CONTENTS)?;

    println!(
        "Generated template at: {}",
        output_location.replace("//spf_template", "/spf_template")
    );

    exit(0)
}
