use tempfile::TempDir;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::model::metadata::base::{BaseMetadata, CommonMetadata, MetadataError};

fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> std::path::PathBuf {
    let file_path = dir.path().join(name);
    let mut file = File::create(&file_path).expect("Failed to create test file");
    file.write_all(content).expect("Failed to write content");
    file_path
}

#[test]
fn test_common_metadata_file() {
    let test_id = "[BASE_METADATA_FILE]";
    println!("\n\n\n{} Testing file metadata creation...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_filename = "test.txt";
    let test_content = b"Hello, World!";
    let file_path = create_test_file(&temp_dir, test_filename, test_content);

    let metadata = CommonMetadata::new(&file_path).expect("Failed to create metadata");

    println!("{} Created metadata: {:?}", test_id, metadata);

    assert_eq!(metadata.size(), test_content.len() as u64);
    assert!(!metadata.is_directory());
    assert!(metadata.is_file());
    assert_eq!(metadata.name(), test_filename);
    assert!(metadata.exists());
    assert!(metadata.created() >= UNIX_EPOCH);
    assert!(metadata.modified() >= UNIX_EPOCH);
}

#[test]
fn test_common_metadata_directory() {
    let test_id = "[BASE_METADATA_DIR]";
    println!("\n\n\n{} Testing directory metadata creation...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let dir_path = temp_dir.path();

    let metadata = CommonMetadata::new(dir_path).expect("Failed to create metadata");

    println!("{} Created directory metadata: {:?}", test_id, metadata);

    assert!(metadata.is_directory());
    assert!(!metadata.is_file());
    assert!(metadata.exists());
    assert!(metadata.created() >= UNIX_EPOCH);
    assert!(metadata.modified() >= UNIX_EPOCH);
}

#[test]
fn test_invalid_path() {
    let test_id = "[BASE_METADATA_INVALID]";
    println!("\n\n\n{} Testing invalid path handling...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let invalid_path = temp_dir.path().join("nonexistent.txt");

    println!("{} Testing invalid path: {:?}", test_id, invalid_path);

    match CommonMetadata::new(&invalid_path) {
        Err(MetadataError::InvalidPath(err_msg)) => {
            println!("{} Got expected error: {}", test_id, err_msg);
            assert!(err_msg.contains("Path does not exist"));
        }
        other => panic!("{} Expected InvalidPath error, got {:?}", test_id, other),
    }
}

#[test]
fn test_time_consistency() {
    let test_id = "[BASE_METADATA_TIME]";
    println!("\n\n\n{} Testing timestamp consistency...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_filename = "time_test.txt";
    let file_path = create_test_file(&temp_dir, test_filename, b"Test content");

    let metadata = CommonMetadata::new(&file_path).expect("Failed to create metadata");
    let now = SystemTime::now();

    println!("{} File timestamps:", test_id);
    println!("Created: {:?}", metadata.created());
    println!("Modified: {:?}", metadata.modified());
    println!("Current time: {:?}", now);

    assert!(metadata.created() <= now);
    assert!(metadata.modified() <= now);
    assert!(metadata.created() <= metadata.modified());
}

#[test]
fn test_file_extensions() {
    let test_id = "[BASE_METADATA_EXTENSIONS]";
    println!("\n\n\n{} Testing file extension handling...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_cases = vec![
        ("normal.txt", "txt"),
        ("multiple.extension.pdf", "pdf"),
        ("no_extension", "None"),
        ("hidden.file.", "None"),  // Ends with dot
        (".hidden", "None"),     // Hidden file in Unix
        (".hidden.txt", "txt"),    // Hidden file with extension
        ("with space.doc", "doc"), // File with space
        ("CAPS.TXT", "TXT"),      // Upper case extension
        ("mixed.TxT", "TxT"),     // Mixed case extension
    ];

    for (filename, expected_ext) in test_cases {
        println!("{} Testing filename: {}", test_id, filename);
        let file_path = create_test_file(&temp_dir, filename, b"test content");
        let metadata = CommonMetadata::new(&file_path).expect("Failed to create metadata");
        
        assert_eq!(
            metadata.extension(),
            expected_ext,
            "Failed extension test for {}",
            filename
        );
        println!("{} Extension test passed for {}", test_id, filename);
    }
}


#[test]
fn test_directory_extension() {
    let test_id = "[BASE_METADATA_DIR_EXT]";
    println!("\n\n\n{} Testing directory extension handling...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test regular directory
    let metadata = CommonMetadata::new(temp_dir.path()).expect("Failed to create metadata");
    assert_eq!(metadata.extension(), "", "Directory should have empty extension");

    // Test directory with dot in name
    let dir_with_dot = temp_dir.path().join("folder.ext");
    std::fs::create_dir(&dir_with_dot).expect("Failed to create test directory");
    let metadata = CommonMetadata::new(&dir_with_dot).expect("Failed to create metadata");
    assert_eq!(metadata.extension(), "", "Directory with dot should have empty extension");
}

#[test]
fn test_special_paths() {
    let test_id = "[BASE_METADATA_SPECIAL]";
    println!("\n\n\n{} Testing special path handling...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test file with very long name (but valid characters)
    let long_name = "a".repeat(100) + ".txt";
    let file_path = create_test_file(&temp_dir, &long_name, b"test content");
    let metadata = CommonMetadata::new(&file_path).expect("Failed to create metadata");
    assert_eq!(metadata.extension(), "txt", "Failed to handle long filename");

    // Test file with only dots
    //let dots_file = create_test_file(&temp_dir, "....", b"test content");
    //let metadata = CommonMetadata::new(&dots_file).expect("Failed to create metadata");
    //assert_eq!(metadata.extension(), "None", "Failed to handle dots-only filename");

    // Test file with Windows-compatible special characters
    let special_chars = vec![
        "file-name.txt",    // hyphen
        "file_name.txt",    // underscore
        "file (1).txt",     // parentheses and numbers
        "file~1.txt",       // tilde
        "file 123.txt",     // spaces and numbers
    ];

    for filename in special_chars {
        println!("{} Testing special filename: {}", test_id, filename);
        let file_path = create_test_file(&temp_dir, filename, b"test content");
        let metadata = CommonMetadata::new(&file_path).expect("Failed to create metadata");
        assert_eq!(
            metadata.extension(),
            "txt",
            "Failed to handle special characters in {}",
            filename
        );
    }
}

#[test]
fn test_utf8_paths() {
    let test_id = "[BASE_METADATA_UTF8]";
    println!("\n\n\n{} Testing UTF-8 path handling...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    let test_cases = vec![
        ("文件.文本", "文本"),         // Chinese
        ("ファイル.テキスト", "テキスト"), // Japanese
        ("파일.텍스트", "텍스트"),       // Korean
        ("αρχείο.κείμενο", "κείμενο"),  // Greek
        ("файл.текст", "текст"),       // Russian
    ];

    for (filename, expected_ext) in test_cases {
        println!("{} Testing UTF-8 filename: {}", test_id, filename);
        let file_path = create_test_file(&temp_dir, filename, b"test content");
        let metadata = CommonMetadata::new(&file_path).expect("Failed to create metadata");
        
        assert_eq!(
            metadata.extension(),
            expected_ext,
            "Failed UTF-8 extension test for {}",
            filename
        );
    }
}

#[test]
fn test_file_setters() {
    let test_id = "[BASE_METADATA_SETTERS]";
    println!("\n\n\n{} Testing metadata setters...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_filename = "original.txt";
    let test_content = b"Initial content";
    let file_path = create_test_file(&temp_dir, test_filename, test_content);

    // Create initial metadata from the file.
    let mut metadata = CommonMetadata::new(&file_path).expect("Failed to create metadata");
    println!("{} Original metadata: {:?}", test_id, metadata);

    // Update the name and extension.
    let new_name = "updated.txt".to_string();
    let new_extension = "md".to_string();
    metadata.set_name(new_name.clone());
    metadata.set_extension(new_extension.clone());
    assert_eq!(metadata.name(), new_name, "Name should be updated");
    assert_eq!(metadata.extension(), new_extension, "Extension should be updated");

    // Simulate an update in file size.
    metadata.set_size(12345);
    assert_eq!(metadata.size(), 12345, "Size should be updated");

    // For a file, children remains None.
    metadata.set_children(42);
    assert!(metadata.children().is_none(), "Children should remain None for files");

    // Update the modified timestamp.
    let new_modified = SystemTime::now();
    metadata.set_modified(new_modified);
    assert_eq!(metadata.modified(), new_modified, "Modified time should be updated");

    // Update the exists flag.
    metadata.set_exists(false);
    assert!(!metadata.exists(), "Exists flag should be updated to false");

    println!("{} Updated metadata: {:?}", test_id, metadata);
}


#[test]
fn test_directory_setters() {
    let test_id = "[BASE_METADATA_DIR_SETTERS]";
    println!("\n\n\n{} Testing directory metadata setters...", test_id);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    // Use the temporary directory as the target folder.
    let mut metadata = CommonMetadata::new(temp_dir.path()).expect("Failed to create metadata");

    println!("{} Original directory metadata: {:?}", test_id, metadata);

    // For directories, children should initially be Some(0)
    assert_eq!(metadata.children(), Some(0), "Initial children count should be 0 for directories");

    // Update children count
    metadata.set_children(5);
    assert_eq!(metadata.children(), Some(5), "Children count should be updated for directories");

    // Update name
    let new_name = "updated_folder".to_string();
    metadata.set_name(new_name.clone());
    assert_eq!(metadata.name(), new_name, "Name should be updated");

    // Update extension (even if directories usually have an empty extension)
    let new_extension = "folder_ext".to_string();
    metadata.set_extension(new_extension.clone());
    assert_eq!(metadata.extension(), new_extension, "Extension should be updated");

    // Update size
    metadata.set_size(9876);
    assert_eq!(metadata.size(), 9876, "Size should be updated");

    // Update modified timestamp.
    let new_modified = SystemTime::now();
    metadata.set_modified(new_modified);
    assert_eq!(metadata.modified(), new_modified, "Modified time should be updated");

    // Update exists flag.
    metadata.set_exists(false);
    assert!(!metadata.exists(), "Exists flag should be updated to false");

    println!("{} Updated directory metadata: {:?}", test_id, metadata);
}