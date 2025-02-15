use tempfile::TempDir;
use std::fs::File;
use std::io::Write;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::path::Path;
use crate::model::metadata::windows::WindowsMetadata;
use crate::model::metadata::base::BaseMetadata;

fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> std::path::PathBuf {
    let file_path = dir.path().join(name);
    let mut file = File::create(&file_path).expect("Failed to create test file");
    file.write_all(content).expect("Failed to write content");
    file_path
}

#[cfg(test)]
mod windows_tests {
    use super::*;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::fileapi::SetFileAttributesW;

    /// Helper function to set file attributes on Windows.
    fn set_file_attributes(path: &Path, attrs: u32) -> std::io::Result<()> {
        // Convert the path to a wide string with a null terminator.
        let path_wide: Vec<u16> = path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let result = unsafe { SetFileAttributesW(path_wide.as_ptr(), attrs) };
        if result == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_valid_file() {
        let test_id = "[BASE_METADATA_FILE]";
        println!("\n\n\n{} Testing file metadata creation...", test_id);

        // Create a temporary directory and a test file.
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let test_filename = "test.txt";
        let test_content = b"Hello, World! This is a test file for the system wowie wowie";
        let file_path = create_test_file(&temp_dir, test_filename, test_content);

        // Obtain the metadata.
        let metadata = WindowsMetadata::new(&file_path).expect("Failed to create metadata");

        // Print the formatted metadata.
        println!("{}", metadata.formatted_metadata());

        // Assertions for common metadata (platform independent)
        assert_eq!(metadata.name(), test_filename.to_string(), "File name should match");
        assert_eq!(metadata.extension(), "txt".to_string(), "Extension should be 'txt'");
        assert_eq!(
            metadata.size(),
            test_content.len() as u64,
            "File size should match the content length"
        );
        assert!(!metadata.is_directory(), "Should not be a directory");
        assert!(metadata.is_file(), "Should be a file");
        assert!(metadata.exists(), "File should exist");

        // Assertions for Windows-specific metadata.
        // Typically, a newly created file is not read-only, hidden, or a system file.
        assert!(!metadata.is_read_only(), "File should not be read-only by default");
        assert!(!metadata.is_hidden(), "File should not be hidden by default");
        assert!(!metadata.is_system(), "File should not be a system file by default");
        // On Windows, newly created files usually have the archive attribute set.
        assert!(metadata.is_archive(), "File should have the archive attribute set");
        // Assert raw attributes match expected value (0x00000020)
        assert_eq!(
            metadata.raw_attributes(),
            0x00000020,
            "Raw attributes should match expected Windows value"
        );
    }

    #[test]
    fn test_varied_attributes() {
        println!("\n\n\n[VARIED_ATTRIBUTES] Testing varied Windows file attributes...");

        // Create a temporary directory and a test file.
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let test_filename = "test_attributes.txt";
        let test_content = b"Test file for varied attributes";
        let file_path = create_test_file(&temp_dir, test_filename, test_content);

        // Define target attributes:
        // FILE_ATTRIBUTE_READONLY = 0x00000001
        // FILE_ATTRIBUTE_HIDDEN   = 0x00000002
        // FILE_ATTRIBUTE_SYSTEM   = 0x00000004
        // FILE_ATTRIBUTE_ARCHIVE  = 0x00000020
        let target_attributes = 0x00000001 | 0x00000002 | 0x00000004 | 0x00000020;

        // Set the file attributes using the helper function.
        set_file_attributes(&file_path, target_attributes).expect("Failed to set file attributes");

        // Retrieve the metadata after updating attributes.
        let metadata = WindowsMetadata::new(&file_path).expect("Failed to get metadata");

        // Verify Windows-specific attributes.
        assert!(metadata.is_read_only(), "File should be read-only");
        assert!(metadata.is_hidden(), "File should be hidden");
        assert!(metadata.is_system(), "File should be a system file");
        assert!(metadata.is_archive(), "File should have the archive attribute set");
        assert_eq!(
            metadata.raw_attributes(),
            target_attributes,
            "Raw attributes should match target attributes"
        );

        // Optionally, print the formatted metadata for visual inspection.
        println!("{}", metadata.formatted_metadata());
    }

    #[test]
    fn test_windows_metadata_setters_file() {
        let test_id = "[WINDOWS_METADATA_SETTERS_FILE]";
        println!("\n\n\n{} Testing Windows metadata setters for a file...", test_id);

        // Create a temporary directory and a test file.
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let test_filename = "test_setters_file.txt";
        let test_content = b"Setter test content";
        let file_path = create_test_file(&temp_dir, test_filename, test_content);

        // Create a WindowsMetadata instance from the file.
        let mut metadata = WindowsMetadata::new(&file_path).expect("Failed to create metadata");

        // Test Windows-specific setters.
        metadata.set_read_only(true);
        assert!(metadata.is_read_only(), "File should be read-only after setting");

        metadata.set_hidden(true);
        assert!(metadata.is_hidden(), "File should be hidden after setting");

        metadata.set_system(true);
        assert!(metadata.is_system(), "File should be a system file after setting");

        metadata.set_archive(false);
        assert!(!metadata.is_archive(), "File archive attribute should be false after setting");

        // Verify raw attributes reflect changes.
        let raw = metadata.raw_attributes();
        assert_eq!(raw & 0x1, 0x1, "Raw attributes should include READONLY");
        assert_eq!(raw & 0x2, 0x2, "Raw attributes should include HIDDEN");
        assert_eq!(raw & 0x4, 0x4, "Raw attributes should include SYSTEM");
        assert_eq!(raw & 0x20, 0x0, "Raw attributes should not include ARCHIVE");

        // Test delegated setters for common metadata.
        let new_name = "updated_file.txt".to_string();
        metadata.set_name(new_name.clone());
        assert_eq!(metadata.name(), new_name, "Name should be updated");

        let new_extension = "md".to_string();
        metadata.set_extension(new_extension.clone());
        assert_eq!(metadata.extension(), new_extension, "Extension should be updated");

        metadata.set_size(54321);
        assert_eq!(metadata.size(), 54321, "Size should be updated");

        // For files, children should remain None even if we try to update.
        metadata.set_children(10);
        assert!(metadata.children().is_none(), "Children should remain None for files");

        let new_modified = SystemTime::now();
        metadata.set_modified(new_modified);
        assert_eq!(metadata.modified(), new_modified, "Modified time should be updated");

        metadata.set_exists(false);
        assert!(!metadata.exists(), "Exists flag should be updated to false");

        println!("{} Updated file metadata: {}", test_id, metadata.formatted_metadata());
    }

    #[test]
    fn test_windows_metadata_setters_directory() {
        let test_id = "[WINDOWS_METADATA_SETTERS_DIR]";
        println!("\n\n\n{} Testing Windows metadata setters for a directory...", test_id);

        // Use the temporary directory itself as the target directory.
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let mut metadata = WindowsMetadata::new(temp_dir.path()).expect(
            "Failed to create metadata"
        );

        // Test Windows-specific setters.
        metadata.set_read_only(true);
        assert!(metadata.is_read_only(), "Directory should be read-only after setting");

        metadata.set_hidden(true);
        assert!(metadata.is_hidden(), "Directory should be hidden after setting");

        metadata.set_system(true);
        assert!(metadata.is_system(), "Directory should be a system directory after setting");

        metadata.set_archive(false);
        assert!(
            !metadata.is_archive(),
            "Directory archive attribute should be false after setting"
        );

        // Test delegated setters for common metadata.
        let new_name = "updated_folder".to_string();
        metadata.set_name(new_name.clone());
        assert_eq!(metadata.name(), new_name, "Directory name should be updated");

        // Although directories normally have no extension, we can still update it.
        let new_extension = "dir_ext".to_string();
        metadata.set_extension(new_extension.clone());
        assert_eq!(metadata.extension(), new_extension, "Directory extension should be updated");

        metadata.set_size(98765);
        assert_eq!(metadata.size(), 98765, "Directory size should be updated");

        // For directories, children can be updated.
        metadata.set_children(7);
        assert_eq!(
            metadata.children(),
            Some(7),
            "Children count should be updated for directories"
        );

        let new_modified = SystemTime::now();
        metadata.set_modified(new_modified);
        assert_eq!(metadata.modified(), new_modified, "Modified time should be updated");

        metadata.set_exists(false);
        assert!(!metadata.exists(), "Exists flag should be updated to false");

        println!("{} Updated directory metadata: {}", test_id, metadata.formatted_metadata());
    }
}
