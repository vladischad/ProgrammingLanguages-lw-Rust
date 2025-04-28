use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::Path;

/// Finds and returns the most common file extension in the given directory and its subdirectories.
///
/// # Arguments
/// * `dir` - A reference to the directory path to scan.
///
/// # Returns
/// * `Option<(OsString, u64)>` -
///   - `Some((extension, count))` where `extension` is the most frequent file extension and `count` is its frequency.
///   - `None` if no files with extensions are found.
///
/// # Behavior
/// - Recursively scans all files under the given directory.
/// - Counts the occurrences of each file extension.
/// - Returns the extension that occurs the most.
pub fn get_most_common_file_type(dir: &Path) -> Option<(OsString, u64)> {
    let mut extensions: HashMap<OsString, u64> = HashMap::new();
    process_extensions(dir, &mut extensions);
    let mut most_common: Option<(OsString, u64)> = None;
    for (ext, &num) in extensions.iter() {
        most_common = Some(match most_common {
            Some((old_ext, old_num)) => {
                if num > old_num {
                    (ext.clone(), num)
                } else {
                    (old_ext, old_num)
                }
            }
            None => (ext.clone(), num),
        })
    }
    most_common
}

/// Recursively processes a directory and counts the occurrences of each file extension.
///
/// # Arguments
/// * `dir` - A reference to the directory path to scan.
/// * `extensions` - A mutable reference to a `HashMap` where keys are file extensions (`OsString`) 
///   and values are counts of how many times each extension appears.
///
/// # Behavior
/// - For each file encountered, it extracts the extension and increments its count in the `HashMap`.
/// - For each directory encountered, it recursively processes its contents.
fn process_extensions(dir: &Path, extensions: &mut HashMap<OsString, u64>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                process_extensions(&entry.path(), extensions);
            } else {
                if let Some(extension) = path.extension() {
                    let count = extensions.entry(extension.to_os_string()).or_insert(0);
                    *count += 1;
                }
            }
        }
    }
}
