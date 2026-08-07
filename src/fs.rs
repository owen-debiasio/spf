// use std::path::Path;

// pub fn path_exists(path: &str) -> bool {
//     Path::new(path).exists()
// }

use std::{fs::File, path::Path};
use zip::ZipWriter;

use crate::error;

pub fn create_archive(name: &str, paths_to_include: Vec<&str>) {
    let path = Path::new(name);
    let file =
        File::create(&path).unwrap_or_else(|file| error(&format!("Failed to create file: {file}")));
    let mut zip = ZipWriter::new(file);
}
