#[cfg(test)]
mod folder_tests {
    use crate::model::file::base_file::File;
    use crate::model::folder::Folder;
    use crate::model::path_type::PathType;
    use crate::model::metadata::base::BaseMetadata;
    use std::sync::{Arc, Mutex};
    use tempfile::TempDir;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    // Helper function to create a test folder.
    fn create_test_folder(dir: &TempDir, name: &str) -> Arc<Mutex<Folder>> {
        let folder_path = dir.path().join(name);
        println!("[TEST_HELPER] Creating folder at: {:?}", folder_path);
        fs::create_dir(&folder_path).expect("Failed to create folder");
        Folder::new(&folder_path, None, 0)
    }

    // Helper function to create a test file.
    fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> Arc<Mutex<File>> {
        let file_path = dir.path().join(name);
        println!("[TEST_HELPER] Creating file at: {:?}", file_path);
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        file.write_all(content).expect("Failed to write content");
        Arc::new(Mutex::new(File::new(&file_path, None)))
    }

    #[test]
    fn test_folder_creation() {
        let test_id = "[FOLDER_CREATION]";
        println!("\n\n\n{} Testing folder creation...", test_id);
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        
        // Use the temp directory itself as the folder.
        let folder = Folder::new(temp_dir.path(), None, 0);
        let folder_locked = folder.lock().unwrap();
        println!("{} Folder metadata:\n{}", test_id, folder_locked.get_formatted_metadata());
        
        // For a new folder, the children count should be 0.
        assert_eq!(folder_locked.metadata.children().unwrap_or(0), 0);
        // The folder's size is initially 0.
        assert_eq!(folder_locked.metadata.size(), 0);
    }

    #[test]
    fn test_folder_rename() {
        let test_id = "[FOLDER_RENAME]";
        println!("\n\n\n{} Testing folder rename...", test_id);
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        
        // Create a test folder inside the temp directory.
        let folder = create_test_folder(&temp_dir, "folder1");
        {
            let mut folder_locked = folder.lock().unwrap();
            let original_url = folder_locked.url.clone();
            println!("{} Original folder metadata:\n{}", test_id, folder_locked.get_formatted_metadata());
            
            folder_locked.modify_name("renamed_folder".to_string()).expect("Failed to rename folder");
            println!("{} Folder metadata after rename:\n{}", test_id, folder_locked.get_formatted_metadata());
            assert_eq!(folder_locked.name, "renamed_folder");
            assert!(Path::new(&folder_locked.url).exists());
            assert!(!Path::new(&original_url).exists());
        }
    }

    #[test]
    fn test_add_and_remove_child() {
        let test_id = "[FOLDER_CHILD]";
        println!("\n\n\n{} Testing adding and removing children...", test_id);
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        
        // Create a parent folder.
        let folder = create_test_folder(&temp_dir, "parent");
        // Create a child file.
        let file_child = create_test_file(&temp_dir, "child.txt", b"Child content");
        
        {
            let mut folder_locked = folder.lock().unwrap();
            let initial_children = folder_locked.metadata.children().unwrap_or(0);
            let initial_size = folder_locked.metadata.size();

            println!("{} Initial folder metadata:\n{}", test_id, folder_locked.get_formatted_metadata());

            // Add the child.
            folder_locked.add_child(PathType::File(Arc::clone(&file_child)));
            println!("{} Folder metadata after adding child:\n{}", test_id, folder_locked.get_formatted_metadata());
            assert_eq!(folder_locked.metadata.children().unwrap_or(0), initial_children + 1);
            
            let child_size = {
                let file = file_child.lock().unwrap();
                file.metadata.size()
            };
            assert_eq!(folder_locked.metadata.size(), initial_size + child_size);

            // Now remove the child.
            let removed = folder_locked.remove_child(&PathType::File(Arc::clone(&file_child)));
            println!("{} Folder metadata after removing child:\n{}", test_id, folder_locked.get_formatted_metadata());
            assert!(removed.is_some());
            assert_eq!(folder_locked.metadata.children().unwrap_or(0), initial_children);
            assert_eq!(folder_locked.metadata.size(), initial_size);
        }
    }

    #[test]
    fn test_get_formatted_metadata() {
        let test_id = "[FOLDER_METADATA]";
        println!("\n\n\n{} Testing formatted metadata...", test_id);
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let folder = create_test_folder(&temp_dir, "folder_meta");
        let folder_locked = folder.lock().unwrap();
        let meta_str = folder_locked.get_formatted_metadata();
        println!("{} Formatted metadata:\n{}", test_id, meta_str);
        assert!(meta_str.contains(&folder_locked.name));
    }
}
