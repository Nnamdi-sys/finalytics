use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the path to the current executable
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

    // Define the paths to the database files
    let db_files = [("finalytics", "src/database/sqlite/finalytics.db")];

    // Copy the database files to the executable directory
    for (file_name, source_path) in &db_files {
        let dest_path = PathBuf::from(exe_dir).join(file_name);
        fs::copy(source_path, dest_path).expect("Failed to copy database file");
    }
}
