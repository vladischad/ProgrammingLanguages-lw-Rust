use std::fs;
use std::path::Path;

// Demenstration of a tuple struct
pub struct Counts(pub usize, pub usize);

/// Recursively counts files and folders in a given directory.
/// 
/// # Arguments
/// * `dir_path` - A reference to the path you want to scan.
/// 
/// # Returns
/// A Counts tuple Struct `(file_count, folder_count)`.
pub fn count_files_and_folders(dir_path: &Path) -> Counts {
    let mut file_count = 0;
    let mut folder_count = 0;

    if dir_path.is_dir() {
        folder_count += 1;                              // Count the current directory itself

        if let Ok(entries) = fs::read_dir(dir_path) {   // Returns an iterator for entries if it is a valid directory
            for entry_result in entries {               // Iterate over the entries in the directory
                if let Ok(entry) = entry_result {       // Check if the entry is valid
                    let path = entry.path();            // Get the path of the current entry
                    if path.is_file() {                 // if path is a file then increment the file count and continue in the loop
                        file_count += 1;
                    } else if path.is_dir() {           // if path is a directory then recursively count files and folders in it
                        let Counts(child_files, child_folders) = count_files_and_folders(&path);
                        file_count += child_files;      // Add the count of files from the child directory
                        folder_count += child_folders;  // Add the count of folders from the child directory
                    }
                }
            }
        }
    }

    Counts(file_count, folder_count)                    // Return the counts as a tuple struct, don't need to use return keyword here if you leave out the semicolon
}
