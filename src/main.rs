/// Main module imports submodules responsible for different tasks:
/// - `one_georgia_avg_size`:           Calculates average file size.
/// - `two_tiernan_file_count`:         Counts files and folders.
/// - `three_vlad_sort_by_size`:        Sorts files by size.
/// - `four_evan_common_file_type`:     Determines the most common file extension.
mod four_evan_common_file_type;
mod one_georgia_avg_size;
mod three_vlad_sort_by_size;
mod two_tiernan_file_count;

use std::env;
use std::path::Path;

/// Main entry point of the application.
///
/// # Behavior
/// - Expects a directory path as a command-line argument.
/// - Validates the path.
/// - Calculates and displays:
///   - Average file size
///   - Number of files and folders
///   - Files sorted by size
///   - Most common file extension
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        return;
    }

    let dir_path = Path::new(&args[1]);
    if !dir_path.is_dir() {
        eprintln!("Provided path is not a directory");
        return;
    }

    println!();
    
    // Georgia: Average file size
    println!("Average file size: {:.2} bytes", one_georgia_avg_size::average_file_size(dir_path));

    // Tiernan: Counting files and folders
    println!("\nCounting Files & directories:");
    let counts = two_tiernan_file_count::count_files_and_folders(dir_path); // returns a tuple struct (file_count, folder_count)
    println!("Number of files: {}", counts.0);
    println!("Number of Folders: {}", counts.1);
    
    // Vlad: Files sorted by size
    println!("\nFiles sorted by size:");
    let sorted = three_vlad_sort_by_size::get_sorted_files_by_size(dir_path);
    for (path, size) in sorted {
        println!("{} - {} bytes", path.display(), size);
    }

    println!();

    // Evan: Most common file extension
    match four_evan_common_file_type::get_most_common_file_type(dir_path) {
        Some((os_extension, occurrences)) => match os_extension.into_string() {
            Ok(extension) => {
                println!("Most common file extension: {extension} - {occurrences} occurrences")
            }
            Err(_) => println!("No files in that directory have extensions"),
        },
        None => println!("No files in that directory have extensions"),
    }

    println!();
}
