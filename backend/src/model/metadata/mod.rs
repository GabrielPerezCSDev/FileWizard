pub mod base;
#[cfg(windows)]
pub mod windows;











// Required imports
use std::collections::HashMap;
use std::path::Path;
use std::fs::{ self, Metadata };
use std::ffi::OsStr;
use chrono::{ DateTime, Local };
use std::time::{SystemTime, UNIX_EPOCH};

// Metadata key constants
pub const KEY_RAW_SIZE: &str = "raw_size";
pub const KEY_SIZE: &str = "size";
pub const KEY_CREATED: &str = "created";
pub const KEY_MODIFIED: &str = "modified";
pub const KEY_FILE_EXTENSION: &str = "file_extension";
pub const VALUE_NO_EXTENSION: &str = "None";
pub const VALUE_NOT_AVAILABLE: &str = "N/A";
pub const KEY_IS_DIRECTORY: &str = "is_directory";
pub const KEY_CHILDREN_COUNT: &str = "children_count";
pub const KEY_PERMISSIONS: &str = "permissions";
pub const KEY_READ_ONLY: &str = "read_only";
pub const KEY_HIDDEN: &str = "hidden";
pub const KEY_SYSTEM: &str = "system";

// Extract common metadata for both files and folders
pub fn file_folder_metadata(metadata: &mut HashMap<String, String>, path: &Path) {
    if let Ok(meta) = fs::metadata(path) {
        insert_size(metadata, &meta);
        insert_creation_time(metadata, &meta);
        insert_modification_time(metadata, &meta);
        insert_permissions(metadata, &meta);
    } else {
        eprintln!("Warning: Could not retrieve metadata for path: {:?}", path);
    }
}
/// Extracts permissions and adds them to metadata
fn insert_permissions(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = meta.permissions().mode();
        let permissions_str = format!(
            "{}{}{}",
            if (mode & 0o400) != 0 { "r" } else { "-" }, // Owner Read
            if (mode & 0o200) != 0 { "w" } else { "-" }, // Owner Write
            if (mode & 0o100) != 0 { "x" } else { "-" }  // Owner Execute
        );
        metadata.insert(KEY_PERMISSIONS.to_string(), permissions_str);
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        let attributes = meta.file_attributes();
        metadata.insert(KEY_READ_ONLY.to_string(), ((attributes & 0x1) != 0).to_string());
        metadata.insert(KEY_HIDDEN.to_string(), ((attributes & 0x2) != 0).to_string());
        metadata.insert(KEY_SYSTEM.to_string(), ((attributes & 0x4) != 0).to_string());
    }
}

fn insert_size(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    let size_in_bytes = meta.len();
    let readable_size = format_size(size_in_bytes);

    metadata.insert(KEY_RAW_SIZE.to_string(), size_in_bytes.to_string()); // Store raw byte size
    metadata.insert(KEY_SIZE.to_string(), readable_size); // Store human-readable size
}

pub fn update_size(metadata: &mut HashMap<String, String>, size: u64) {
    let readable_size = format_size(size);
    // Update both raw and formatted size
    metadata.insert(KEY_RAW_SIZE.to_string(), size.to_string());
    metadata.insert(KEY_SIZE.to_string(), readable_size);
}

// Insert creation time into metadata (stored as Unix timestamp)
fn insert_creation_time(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    let created_time = meta.created()
        .map_err(|_| VALUE_NOT_AVAILABLE.to_string()) // Convert io::Error to string
        .and_then(|created| created.duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs().to_string()) // Convert to String
            .map_err(|_| VALUE_NOT_AVAILABLE.to_string()) // Convert SystemTimeError to string
        )
        .unwrap_or_else(|err| err); // Use the mapped error message

    metadata.insert(KEY_CREATED.to_string(), created_time);
}

// Insert last modified time into metadata (stored as Unix timestamp)
fn insert_modification_time(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    let modified_time = meta.modified()
        .map_err(|_| VALUE_NOT_AVAILABLE.to_string()) // Convert io::Error to string
        .and_then(|modified| modified.duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs().to_string()) // Convert to String
            .map_err(|_| VALUE_NOT_AVAILABLE.to_string()) // Convert SystemTimeError to string
        )
        .unwrap_or_else(|err| err); // Use the mapped error message

    metadata.insert(KEY_MODIFIED.to_string(), modified_time);
}

// File-specific metadata
pub fn file_specific_metadata(metadata: &mut HashMap<String, String>, path: &Path) {

    if let Ok(meta) = fs::metadata(path) {
        if meta.is_file() {
            file_folder_metadata(metadata, path);
            metadata.insert(KEY_IS_DIRECTORY.to_string(), "false".to_string());
            metadata.insert(KEY_CHILDREN_COUNT.to_string(), VALUE_NOT_AVAILABLE.to_string());
            insert_file_extension(metadata, path);
        }
    }
}

// Insert file extension into metadata
fn insert_file_extension(metadata: &mut HashMap<String, String>, path: &Path) {
    if let Some(extension) = path.extension().and_then(OsStr::to_str) {
        metadata.insert(KEY_FILE_EXTENSION.to_string(), extension.to_string());
    } else {
        metadata.insert(KEY_FILE_EXTENSION.to_string(), VALUE_NO_EXTENSION.to_string());
    }
}

pub fn update_children_count(metadata: &mut HashMap<String, String>, children: u64) {
    metadata.insert(KEY_CHILDREN_COUNT.to_string(), children.to_string());
}

// Folder-specific metadata
pub fn folder_specific_metadata(metadata: &mut HashMap<String, String>, path: &Path) {
    

    if let Ok(meta) = fs::metadata(path) {
        if meta.is_dir() {
            file_folder_metadata(metadata, path);
            metadata.insert(KEY_IS_DIRECTORY.to_string(), "true".to_string());

            metadata.insert(KEY_CHILDREN_COUNT.to_string(), "0".to_string());
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
        (mode & 0o444) != 0 // Checks the read permission bits
    }

    #[cfg(windows)]
    {
        // On Windows, permissions are more complex. A simple heuristic could be:
        !metadata.permissions().readonly() // Check if the entry is read-only
    }
}

// Function to format file sizes into human-readable strings using metric prefixes
pub fn format_size(bytes: u64) -> String {
    const KI_B: f64 = 1024.0;
    const MI_B: f64 = KI_B * 1024.0;
    const GI_B: f64 = MI_B * 1024.0;
    const TI_B: f64 = GI_B * 1024.0;
    const PI_B: f64 = TI_B * 1024.0;

    let size = bytes as f64;

    // Helper function to format with consistent rounding
    let format_with_precision = |value: f64, unit: &str| {
        format!("{:.2} {}", (value * 100.0).round() / 100.0, unit)
    };

    if size < KI_B {
        format!("{} bytes", bytes)
    } else if size < MI_B {
        format_with_precision(size / KI_B, "KB")
    } else if size < GI_B {
        format_with_precision(size / MI_B, "MB")
    } else if size < TI_B {
        format_with_precision(size / GI_B, "GB")
    } else if size < PI_B {
        format_with_precision(size / TI_B, "TB")
    } else {
        format_with_precision(size / PI_B, "PB")
    }
}