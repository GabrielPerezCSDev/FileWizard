// Required imports
use std::collections::HashMap;
use std::path::Path;
use std::fs::{self, Metadata};
use std::ffi::OsStr;
use chrono::{DateTime, Local};

// Extract common metadata for both files and folders
pub fn file_folder_metadata(metadata: &mut HashMap<String, String>, path: &Path) {
    if let Ok(meta) = fs::metadata(path) {
        insert_size(metadata, &meta);
        insert_creation_time(metadata, &meta);
        insert_modification_time(metadata, &meta);
    } else {
        eprintln!("Warning: Could not retrieve metadata for path: {:?}", path);
    }
}

// Insert size into metadata
fn insert_size(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    let size_in_bytes = meta.len();
    let readable_size = format_size(size_in_bytes);
    metadata.insert("size".to_string(), readable_size);
}

// Insert creation time into metadata
fn insert_creation_time(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    if let Ok(created) = meta.created() {
        let datetime: DateTime<Local> = created.into();
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        metadata.insert("created".to_string(), formatted_time);
    } else {
        metadata.insert("created".to_string(), "N/A".to_string());
    }
}

// Insert last modified time into metadata
fn insert_modification_time(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    if let Ok(modified) = meta.modified() {
        let datetime: DateTime<Local> = modified.into();
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        metadata.insert("modified".to_string(), formatted_time);
    } else {
        metadata.insert("modified".to_string(), "N/A".to_string());
    }
}

// File-specific metadata
pub fn file_specific_metadata(metadata: &mut HashMap<String, String>, path: &Path) {
    file_folder_metadata(metadata, path);

    if let Ok(meta) = fs::metadata(path) {
        if meta.is_file() {
            insert_file_extension(metadata, path);
        }
    }
}

// Insert file extension into metadata
fn insert_file_extension(metadata: &mut HashMap<String, String>, path: &Path) {
    if let Some(extension) = path.extension().and_then(OsStr::to_str) {
        metadata.insert("file_extension".to_string(), extension.to_string());
    } else {
        metadata.insert("file_extension".to_string(), "None".to_string());
    }
}

// Folder-specific metadata
pub fn folder_specific_metadata(metadata: &mut HashMap<String, String>, path: &Path) {
    file_folder_metadata(metadata, path);
    
    if let Ok(meta) = fs::metadata(path) {
        if meta.is_dir() {
            //metadata.insert("is_directory".to_string(), "true".to_string());
        }
    }
}

// Function to check if a directory/file is accessible
pub fn is_accessible(metadata: &Metadata) -> bool {
    #[cfg(unix)]
    {
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        // Check if the owner, group, or others have read permission (r bit)
        mode & 0o444 != 0 // Checks the read permission bits
    }

    #[cfg(windows)]
    {
        // On Windows, permissions are more complex. A simple heuristic could be:
        !metadata.permissions().readonly() // Check if the entry is read-only
    }
}

// Function to format file sizes into human-readable strings
fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;
    const PB: f64 = TB * 1024.0;

    let size = bytes as f64;

    if size < KB {
        format!("{} bytes", bytes)
    } else if size < MB {
        format!("{:.2} KB", size / KB)
    } else if size < GB {
        format!("{:.2} MB", size / MB)
    } else if size < TB {
        format!("{:.2} GB", size / GB)
    } else if size < PB {
        format!("{:.2} TB", size / TB)
    } else {
        format!("{:.2} PB", size / PB)
    }
}