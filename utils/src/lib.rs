use std::path::PathBuf;

/// Get the base path for the project (File Wizard directory)
pub fn get_base_path() -> PathBuf {
    // Use the parent of the executable directory as the root project directory
    std::env::current_exe()
        .unwrap()
        .parent() // Get the directory containing the executable
        .unwrap()
        .parent() // Go up one level to the project root
        .unwrap()
        .parent() // Go up one level to the project root
        .unwrap()
        .to_path_buf()
}