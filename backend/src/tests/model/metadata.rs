use std::collections::HashMap;
use std::path::Path;
use std::fs;
use tempfile::TempDir;

// Update this import path
use crate::model::metadata::{
    file_folder_metadata,
    file_specific_metadata,
    folder_specific_metadata,
    update_size,
    is_accessible
};

/// Helper function to create a temporary file for testing
fn create_temp_file(dir: &TempDir, content: &[u8]) -> std::path::PathBuf {
    let file_path = dir.path().join("test_file.txt");
    fs::write(&file_path, content).expect("Failed to create test file");
    file_path
}

/// Helper function to create a temporary directory for testing
fn create_temp_dir(parent_dir: &TempDir) -> std::path::PathBuf {
    let dir_path = parent_dir.path().join("test_dir");
    fs::create_dir(&dir_path).expect("Failed to create test directory");
    dir_path
}

#[cfg(test)]
mod metadata_tests {
    use super::*;

    #[test]
    fn test_format_size() {
        let test_cases = vec![
            (0, "0 bytes"),
            (500, "500 bytes"),
            (1500, "1.50 KB"),
            (1500000, "1.50 MB"),
            (1500000000, "1.50 GB"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(format_size(input), expected.to_string());
        }
    }

    #[test]
    fn test_file_metadata_basic() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let test_content = b"Hello, World!";
        let file_path = create_temp_file(&temp_dir, test_content);
        
        let mut metadata = HashMap::new();
        file_folder_metadata(&mut metadata, &file_path);

        // Check if basic metadata fields exist
        assert!(metadata.contains_key("size"));
        assert!(metadata.contains_key("raw_size"));
        assert!(metadata.contains_key("created"));
        assert!(metadata.contains_key("modified"));
        
        // Verify size is correct
        assert_eq!(metadata.get("raw_size").unwrap(), "13"); // "Hello, World!" is 13 bytes
    }

    #[test]
    fn test_file_specific_metadata() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let test_content = b"Test content";
        let file_path = create_temp_file(&temp_dir, test_content);
        
        let mut metadata = HashMap::new();
        file_specific_metadata(&mut metadata, &file_path);

        // Check file extension
        assert_eq!(metadata.get("file_extension").unwrap(), "txt");
    }

    #[test]
    fn test_update_size() {
        let mut metadata = HashMap::new();
        update_size(&mut metadata, 1500);

        assert_eq!(metadata.get("raw_size").unwrap(), "1500");
        assert_eq!(metadata.get("size").unwrap(), "1.50 KB");
    }
}