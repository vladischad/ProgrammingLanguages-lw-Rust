use std::fs;
use std::path::{Path, PathBuf};

/// Returns a vector of tuples containing file paths and their sizes, sorted by size.
///
/// # Arguments
/// * `dir` - A reference to the directory path to scan.
///
/// # Returns
/// A vector of (PathBuf, u64) where PathBuf is the file path and u64 is the file size in bytes.
pub fn get_sorted_files_by_size(dir: &Path) -> Vec<(PathBuf, u64)> {
    let mut files = Vec::new();
    collect_files(dir, &mut files);
    files.sort_by_key(|k| k.1);
    files
}

/// Recursively collects all files in the given directory and stores their paths and sizes.
///
/// # Arguments
/// * `dir` - A reference to the directory path to scan.
/// * `files` - A mutable reference to a vector to store file paths and sizes.
fn collect_files(dir: &Path, files: &mut Vec<(PathBuf, u64)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    files.push((path, metadata.len()));
                }
            } else if path.is_dir() {
                collect_files(&path, files);
            }
        }
    }
}
