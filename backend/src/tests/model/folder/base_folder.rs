#[cfg(test)]
mod folder_tests {
    use crate::model::file::base_file::File;
    use crate::model::folder::Folder;
    use crate::model::path_type::PathType;
    use crate::model::metadata::base::BaseMetadata;
    use crate::model::folder::base_folder::{ FolderError, RemoveError };
    use crate::model::validation::*;
    use std::sync::{ Arc, Mutex };
    use tempfile::TempDir;
    use std::fs;
    use std::io::Write;
    use std::path::{ Path, PathBuf };

    // Helper function to create a test folder.
    fn create_test_folder(dir: &TempDir, name: &str) -> Folder {
        let folder_path = dir.path().join(name);
        println!("[TEST_HELPER] Creating folder at: {:?}", folder_path);
        fs::create_dir(&folder_path).expect("Failed to create folder");

        Folder::new(&folder_path, None, 0)
    }

    fn create_test_file_in(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let file_path = dir.join(name);
        println!("[TEST_HELPER] Creating file at: {:?}", file_path);
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        file.write_all(content).expect("Failed to write content");
    
        file_path
    }

    #[test]
    fn test_folder_creation() {
        let test_id = "[FOLDER_CREATION]";
        println!("\n\n\n{} Testing folder creation...", test_id);

        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        let folder = create_test_folder(&temp_dir, "test_folder");

        println!("{} Folder metadata:\n{}", test_id, folder.metadata.formatted_metadata());

        assert_eq!(folder.metadata.children().unwrap_or(0), 0);
        assert_eq!(folder.metadata.size(), 0);
        assert!(folder.metadata.is_directory());
        assert!(folder.metadata.exists());
    }

    #[test]
    fn test_folder_rename() {
        let test_id = "[FOLDER_RENAME]";
        println!("\n\n\n{} Testing folder rename...", test_id);

        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        let mut folder = create_test_folder(&temp_dir, "folder1");

        let original_path = folder.metadata.path();
        println!("{} Original folder metadata:\n{}", test_id, folder.metadata.formatted_metadata());

        folder.modify_name("renamed_folder".to_string()).expect("Failed to rename folder");

        println!(
            "{} Folder metadata after rename:\n{}",
            test_id,
            folder.metadata.formatted_metadata()
        );

        assert_eq!(folder.metadata.name(), "renamed_folder");
        assert!(folder.metadata.path().exists());
        assert!(!original_path.exists());
    }

    #[test]
fn test_add_and_remove_child() {
    let test_id = "[FOLDER_CHILD]";
    println!("\n\n\n{} Testing adding and removing children...", test_id);
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a parent folder.
    let mut parent_folder = create_test_folder(&temp_dir, "parent");
    let parent_path = parent_folder.metadata.path();

    // ✅ Use helper to create the file in the system.
    let child_file_path = create_test_file_in(&parent_path, "child.txt", b"Child content");

    // ✅ Now instantiate the `File` instance.
    let file_child = PathType::File(File::new(&child_file_path, Some(parent_path.to_path_buf())));

    let initial_children = parent_folder.metadata.children().unwrap_or(0);
    let initial_size = parent_folder.metadata.size();

    println!(
        "{} Initial folder metadata:\n{}",
        test_id,
        parent_folder.metadata.formatted_metadata()
    );

    // Add the child.
    parent_folder
        .add_child(file_child.clone())
        .expect("Failed to add child");

    println!(
        "{} Folder metadata after adding child:\n{}",
        test_id,
        parent_folder.metadata.formatted_metadata()
    );

    assert_eq!(parent_folder.metadata.children().unwrap_or(0), initial_children + 1);

    // ✅ Extract file size dynamically
    let file_size = match &file_child {
        PathType::File(f) => f.metadata.size(),
        _ => panic!("Unexpected PathType when retrieving file size"),
    };
    assert_eq!(parent_folder.metadata.size(), initial_size + file_size);

    // Remove the child.
    let removed = parent_folder
        .remove_child(&file_child)
        .expect("Failed to remove child");

    println!(
        "{} Folder metadata after removing child:\n{}",
        test_id,
        parent_folder.metadata.formatted_metadata()
    );

    match removed {
        PathType::File(_) => {
            assert_eq!(parent_folder.metadata.children().unwrap_or(0), initial_children);
            assert_eq!(parent_folder.metadata.size(), initial_size);
        }
        _ => panic!("Removed child was not a file"),
    }
}



    #[test]
    fn test_get_formatted_metadata() {
        let test_id = "[FOLDER_METADATA]";
        println!("\n\n\n{} Testing formatted metadata...", test_id);
        
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let folder = create_test_folder(&temp_dir, "folder_meta");
    
        let meta_str = folder.metadata.formatted_metadata();
        println!("{} Formatted metadata:\n{}", test_id, meta_str);
    
        assert!(meta_str.contains(&folder.metadata.name()));
        assert!(meta_str.contains(&*folder.metadata.path().to_string_lossy()));
    }
    


    #[test]
    fn test_remove_child_validations() {
        let test_id = "[FOLDER_REMOVE_VALIDATION]";
        println!("\n\n\n{} Testing remove child validations...", test_id);
    
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let mut parent_folder = create_test_folder(&temp_dir, "parent");
    
        // Test case 1: Remove non-existent child.
        let outside_file_path = create_test_file_in(&temp_dir.path(), "nonexistent.txt", b"content");
        let outside_file = PathType::File(File::new(&outside_file_path, None));
        let result = parent_folder.remove_child(&outside_file);
        assert!(matches!(result, Err(RemoveError::NotFound(_))));
    
        // Test case 2: Verify state after successful removal.
        let file_path = create_test_file_in(&parent_folder.metadata.path(), "test.txt", b"content");
    
        let file = PathType::File(File::new(&file_path, Some(parent_folder.metadata.path().to_path_buf())));
    
        parent_folder
            .add_child(file.clone())
            .expect("Add should succeed");
    
        let initial_size = parent_folder.metadata.size();
    
        // ✅ Extract file size dynamically
        let file_size = match &file {
            PathType::File(f) => f.metadata.size(),
            _ => panic!("Unexpected PathType when retrieving file size"),
        };
    
        let result = parent_folder.remove_child(&file);
        assert!(result.is_ok());
    
        // Ensure size updates correctly
        assert_eq!(parent_folder.metadata.size(), initial_size - file_size);
        assert_eq!(parent_folder.metadata.children().unwrap_or(0), 0);
    }
    
    
    #[test]
    fn test_folder_state_consistency() {
        let test_id = "[FOLDER_CONSISTENCY]";
        println!("\n\n\n{} Testing folder state consistency...", test_id);
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
        let mut parent_folder = create_test_folder(&temp_dir, "parent");
    
        // Add multiple files and verify size accumulation.
        let parent_path = parent_folder.metadata.path();
        let files: Vec<_> = (0..3)
            .map(|i| create_test_file_in(&parent_path, &format!("file{}.txt", i), b"content"))
            .collect();
    
        let mut expected_size = 0;
        let mut file_objects = Vec::new();
    
        for file_path in &files {
            //Create `File` instance from each file path
            let file = PathType::File(File::new(&file_path, Some(parent_path.to_path_buf())));
            let file_size = match &file {
                PathType::File(f) => f.metadata.size(),
                _ => panic!("Unexpected PathType"),
            };
            
            expected_size += file_size;
            parent_folder.add_child(file.clone()).expect("Add should succeed");
            file_objects.push(file);
        }
    
        assert_eq!(parent_folder.metadata.size(), expected_size);
        assert_eq!(parent_folder.metadata.children().unwrap_or(0), 3);
    
        // Remove files in reverse and verify size reduction.
        for file in file_objects.into_iter().rev() {
            let file_size = match &file {
                PathType::File(f) => f.metadata.size(),
                _ => panic!("Unexpected PathType"),
            };
            expected_size -= file_size;
            parent_folder
                .remove_child(&file)
                .expect("Remove should succeed");
            assert_eq!(parent_folder.metadata.size(), expected_size);
        }
    }
    

}
