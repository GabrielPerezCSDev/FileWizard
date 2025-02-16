#[cfg(test)]
mod file_tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File as FsFile;
    use std::io::Write;
    use std::fs;
    use crate::model::file::File;
    use crate::model::metadata::base::BaseMetadata;
    use std::path::Path;

    // Helper function to create test files
    fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> std::path::PathBuf {
        let file_path = dir.path().join(name);
        let mut file = FsFile::create(&file_path).expect("Failed to create test file");
        file.write_all(content).expect("Failed to write content");
        file_path
    }

    #[test]
    fn test_file_creation() {
        let test_id = "[FILE_CREATION]";
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let test_content = b"Hello, World!";
        let file_path = create_test_file(&temp_dir, "test.txt", test_content);

        let file = File::new(&file_path, None);

        // Now that name and url are stored in metadata, we access them via metadata getters.
        assert_eq!(file.metadata.name(), "test.txt");
        assert_eq!(file.metadata.path().to_string_lossy(), file_path.to_string_lossy());
        assert!(file.error.is_none());
        assert!(file.parent.is_none());
    }

    #[test]
    fn test_file_name_modification() {
        let test_id = "[FILE_RENAME]";
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let test_content = b"Hello, World!";
        let file_path = create_test_file(&temp_dir, "test.txt", test_content);

        let mut file = File::new(&file_path, None);
        file.modify_name("renamed.txt".to_string()).expect("Failed to rename file");

        // Check that the metadata's name has been updated.
        assert_eq!(file.metadata.name(), "renamed.txt");
        // Verify that the new path (stored in metadata) exists.
        assert!(file.metadata.path().as_path().exists());
        // The original file path should no longer exist.
        assert!(!file_path.exists(), "Old file should not exist");
    }

    #[test]
    fn test_file_extension_modification() {
        let test_id = "[FILE_EXTENSION]";
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let test_content = b"Hello, World!";
        let file_path = create_test_file(&temp_dir, "test.txt", test_content);

        let mut file = File::new(&file_path, None);
        file.modify_extension("md".to_string()).expect("Failed to change extension");

        // Check that the metadata's extension is updated.
        assert_eq!(file.metadata.extension(), "md", "Extension should be updated to 'md'");

        // Check that the new path stored in metadata exists.
        let new_path = file.metadata.path();
        assert!(new_path.exists(), "New file path should exist");

        // The original file should no longer exist.
        assert!(!file_path.exists(), "Old file should not exist");

        // Also verify that the new path has the correct extension.
        assert_eq!(
            new_path.extension().unwrap().to_str().unwrap(),
            "md",
            "New file extension should be 'md'"
        );
    }

    #[test]
    fn test_file_size_update() {
        let test_id = "[FILE_SIZE]";
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let initial_content = b"Hello";
        let file_path = create_test_file(&temp_dir, "test.txt", initial_content);

        let mut file = File::new(&file_path, None);

        // Write new content to file
        let mut fs_file = FsFile::create(&file_path).expect("Failed to open file");
        fs_file.write_all(b"Hello, World!").expect("Failed to write new content");

        // Update size
        file.update_size().expect("Failed to update size");

        assert_eq!(file.metadata.size(), 13); // "Hello, World!" is 13 bytes
    }

    #[test]
    fn test_error_cases() {
        let test_id = "[FILE_ERRORS]";
        println!("\n\n\n{} Testing error cases...", test_id);

        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let test_content = b"Hello, World!";
        let file_path = create_test_file(&temp_dir, "test.txt", test_content);

        let mut file = File::new(&file_path, None);

        // Test invalid name modification.
        // Assuming "invalid/name.txt" is an invalid filename.
        assert!(
            file.modify_name("invalid/name.txt".to_string()).is_err(),
            "Should reject invalid name modification"
        );

        // Test modifying a non-existent file.
        fs::remove_file(&file_path).expect("Failed to remove file");
        assert!(file.update_size().is_err(), "update_size should error on non-existent file");
        assert!(
            file.modify_extension("md".to_string()).is_err(),
            "modify_extension should error on non-existent file"
        );
    }
    
}
