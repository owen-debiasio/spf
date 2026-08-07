use std::{fs, path::Path};

use crate::error;

/// Create .spf package
///
/// dir list format:
/// path/of/original/file:/path/of/install/location
pub fn create_spf_package(dir_list: &str) {
    let dir_list_has_extension = Path::new(dir_list)
        .extension()
        .unwrap_or_default()
        .is_empty();

    if !dir_list_has_extension || dir_list.is_empty() {
        error("Please provide a text file with no file extension!")
    }

    let dir_list = &*fs::read_to_string(dir_list)
        .unwrap_or_else(|err| error(&format!("Failed to open file: {err}")));

    for entry in dir_list.lines() {
        let original_file = entry.split(':').next().unwrap().to_string();
        let file_destination = entry.split(':').next_back().unwrap().to_string();

        // check if the entry is formatted correctly
        check_entry(entry, original_file, file_destination);
    }
}

/// Check if:
/// 1. original file is mentioned
/// 2. file destination is mentioned
/// 3. entry is formatted correctly
fn check_entry(entry: &str, original: String, destination: String) {
    if !(!original.is_empty()
        && !destination.is_empty()
        && entry == &format!("{original}:{destination}")
        && entry.contains(':'))
    {
        error(&format!("Failed to parse entry: {entry}"))
    }
}
