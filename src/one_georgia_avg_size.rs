use std::fs;
use std::path::Path;

/// Calculates the average size of all files in the given directory and its subdirectories.
///
/// # Arguments
/// * `dir` - A reference to the directory path to scan.
///
/// # Returns
/// The average file size in bytes as an f64. Returns 0.0 if there are no files.
pub fn average_file_size(dir: &Path) -> f64 {
    let mut total_size: u64 = 0;
    let mut file_count: u64 = 0;
    collect_file_sizes(dir, &mut total_size, &mut file_count);

    if file_count == 0 {
        0.0
    } else {
        total_size as f64 / file_count as f64
    }
}

/// Recursively traverses the directory and accumulates total file size and file count.
///
/// # Arguments
/// * `dir` - A reference to the directory path to scan.
/// * `total_size` - A mutable reference to accumulate total file sizes.
/// * `file_count` - A mutable reference to accumulate the number of files.
fn collect_file_sizes(dir: &Path, total_size: &mut u64, file_count: &mut u64) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    *total_size += metadata.len();
                    *file_count += 1;
                }
            } else if path.is_dir() {
                collect_file_sizes(&path, total_size, file_count);
            }
        }
    }
}