#[cfg(test)]
mod windows_file_tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::fileapi::SetFileAttributesW;
    use crate::model::file::windows_file::WindowsFile;
    use crate::model::metadata::base::BaseMetadata;
    use crate::model::file::base_file::BaseFileOps;

    fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> std::path::PathBuf {
        let file_path = dir.path().join(name);
        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(content).expect("Failed to write content");
        file_path
    }

    /// Helper function to set file attributes on Windows.
    fn set_file_attributes(path: &Path, attrs: u32) -> std::io::Result<()> {
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
    fn test_windows_file_creation() {
        let test_id = "[WIN_FILE_CREATION]";
        println!("\n\n\n{} Testing Windows file creation...", test_id);

        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let test_content = b"Hello, Windows! This is a test file for the system wowie wowie";
        let file_path = create_test_file(&temp_dir, "test.txt", test_content);

        let file = WindowsFile::new(&file_path, None);

        // Test base properties
        assert_eq!(file.get_name(), "test.txt");
        assert!(!file.has_error());
        assert_eq!(file.get_extension(), "txt");
        assert!(file.exists());
        assert_eq!(file.size(), test_content.len() as u64);
        assert_eq!(file.get_path().as_path(), file_path.as_path(), "Metadata path should match file path");
        // Test Windows-specific properties
        assert!(!file.is_read_only());
        assert!(!file.is_hidden());
        assert!(!file.is_system());
        assert!(file.is_archive()); // New files typically get archive bit set

        println!("{} Created file metadata:\n{}", test_id, file.get_formatted_metadata());
    }

    #[test]
    fn test_windows_file_attributes() {
        let test_id = "[WIN_FILE_ATTRIBUTES]";
        println!("\n\n\n{} Testing Windows file attributes...", test_id);

        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = create_test_file(&temp_dir, "attributes.txt", b"Test content");
        let mut file = WindowsFile::new(&file_path, None);

        // Set and test individual attributes
        println!("{} Testing read-only attribute...", test_id);
        file.set_read_only(true).expect("Failed to set read-only");
        assert!(file.is_read_only());

        println!("{} Testing hidden attribute...", test_id);
        file.set_hidden(true).expect("Failed to set hidden");
        assert!(file.is_hidden());

        println!("{} Testing system attribute...", test_id);
        file.set_system(true).expect("Failed to set system");
        assert!(file.is_system());

        // Set multiple attributes at once using raw attributes
        let target_attributes = 0x00000001 | 0x00000002 | 0x00000004 | 0x00000020;
        set_file_attributes(&file_path, target_attributes).expect("Failed to set file attributes");

        // Refresh file metadata
        file = WindowsFile::new(&file_path, None);

        assert!(file.is_read_only());
        assert!(file.is_hidden());
        assert!(file.is_system());
        assert!(file.is_archive());

        println!(
            "{} File attributes after modification:\n{}",
            test_id,
            file.get_formatted_metadata()
        );
    }

    #[test]
    fn test_windows_file_operations() {
        let test_id = "[WIN_FILE_OPERATIONS]";
        println!("\n\n\n{} Testing Windows file operations...", test_id);

        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = create_test_file(&temp_dir, "operations.txt", b"Test content");
        let mut file = WindowsFile::new(&file_path, None);

        // Test rename
        println!("{} Testing file rename...", test_id);
        file.modify_name("renamed.txt".to_string()).expect("Failed to rename file");
        assert_eq!(file.get_name(), "renamed.txt");

        // Test extension change
        println!("{} Testing extension change...", test_id);
        file.modify_extension("doc".to_string()).expect("Failed to change extension");
        assert_eq!(file.get_extension(), "doc");

        // Test size update after content change
        println!("{} Testing size update...", test_id);
        let new_path = temp_dir.path().join("renamed.doc");
        {
            let mut fs_file = std::fs::File::create(&new_path).expect("Failed to open file");
            fs_file.write_all(b"Updated content that is longer").expect("Failed to write new content");
            fs_file.sync_all().expect("Failed to sync file");
            // fs_file will be dropped here
        }
        file.update_size().expect("Failed to update size");
        //assert_eq!(file.size(), 28); // "Updated content that is longer" is 28 bytes

        println!("{} File after operations:\n{}", test_id, file.get_formatted_metadata());
    }

    #[test]
    fn test_windows_error_cases() {
        let test_id = "[WIN_ERROR_CASES]";
        println!("\n\n\n{} Testing Windows error cases...", test_id);

        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = create_test_file(&temp_dir, "error_test.txt", b"Test content");
        let mut file = WindowsFile::new(&file_path, None);

        // Test invalid characters in filename
        println!("{} Testing invalid characters...", test_id);
        let invalid_names: Vec<String> = vec![
            "test<>.txt".to_string(), // Invalid characters
            "test|file.txt".to_string(), // Pipe
            "test*.txt".to_string(), // Asterisk
            "test?.txt".to_string(), // Question mark
            "test:rename.txt".to_string(), // Colon
            "test\\.txt".to_string(), // Backslash
            "test/file.txt".to_string(), // Forward slash
            "test\"quotes\".txt".to_string(), // Quotes
            "CON.txt".to_string(), // Reserved name
            "PRN.doc".to_string(), // Reserved name
            "AUX.pdf".to_string(), // Reserved name
            "NUL.any".to_string(), // Reserved name
            "COM1.txt".to_string(), // Reserved device name
            "LPT1.txt".to_string(), // Reserved device name
            "".to_string(), // Empty name
            ".".to_owned() + &"a".repeat(256) // Too long (> 255 chars)
        ];

        for invalid_name in invalid_names {
            println!("{} Testing invalid name: {}", test_id, invalid_name);
            assert!(
                file.modify_name(invalid_name.clone()).is_err(),
                "Should reject invalid name: {}",
                invalid_name
            );
        }

        // Test operations on read-only file
        println!("{} Testing read-only operations...", test_id);
        file.set_read_only(true).expect("Failed to set read-only");

        // For Windows read-only files, we need to set the attribute using the Windows API
        let attrs = winapi::um::winnt::FILE_ATTRIBUTE_READONLY;
        set_file_attributes(&file_path, attrs).expect("Failed to set read-only attribute");

        // Attempt to modify the read-only file
        assert!(
            file.modify_name("new_name.txt".to_string()).is_err(),
            "Should not be able to rename read-only file"
        );

        // Test trailing spaces and periods
        println!("{} Testing trailing characters...", test_id);
        let trailing_cases = vec![
            "test ", // Space at end
            "test.", // Period at end
            "test.  ", // Multiple spaces at end
            "test..", // Multiple periods at end
            " test", // Space at start
            ".... " // All special
        ];

        for case in trailing_cases {
            println!("{} Testing trailing case: '{}'", test_id, case);
            assert!(
                file.modify_name(case.to_string()).is_err(),
                "Should reject name with special trailing characters: '{}'",
                case
            );
        }

        // Test operations on non-existent file
        println!("{} Testing non-existent file...", test_id);
        let non_existent = WindowsFile::new(&temp_dir.path().join("non_existent.txt"), None);
        assert!(non_existent.has_error());

        println!("{} Error cases tested", test_id);
    }
}
