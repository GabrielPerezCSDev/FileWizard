#[cfg(test)]
mod windows_folder_tests {
    use super::*;
    use crate::model::folder::windows_folder::WindowsFolder;
    use crate::model::file::windows_file::WindowsFile;
    use tempfile::TempDir;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use crate::model::file_system::path_type::PathType;
    use crate::model::folder::base_folder::BaseFolderOps;
    use crate::model::metadata::base::BaseMetadata;
    use std::time::SystemTime;

    // Helper function to create a test Windows folder
    fn create_test_folder(dir: &TempDir, name: &str) -> WindowsFolder {
        let folder_path = dir.path().join(name);
        println!("[TEST_HELPER] Creating Windows folder at: {:?}", folder_path);
        fs::create_dir(&folder_path).expect("Failed to create folder");
        WindowsFolder::new(&folder_path, None, 0)
    }

    fn create_test_file_in(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let file_path = dir.join(name);
        println!("[TEST_HELPER] Creating Windows file at: {:?}", file_path);
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        file.write_all(content).expect("Failed to write content");
        file_path
    }

    #[test]
    fn test_windows_folder_attributes() {
        let test_id = "[WIN_FOLDER_ATTRS]";
        println!("\n\n\n{} Testing Windows folder attributes...", test_id);
        
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let mut folder = create_test_folder(&temp_dir, "test_folder");

        // Test read-only attribute
        folder.set_read_only(true).expect("Failed to set read-only");
        assert!(folder.is_read_only());
        
        // Verify can't modify read-only folder
        assert!(folder.modify_name("new_name".to_string()).is_err());
        
        folder.set_read_only(false).expect("Failed to remove read-only");
        assert!(!folder.is_read_only());

        // Test hidden attribute
        folder.set_hidden(true).expect("Failed to set hidden");
        assert!(folder.is_hidden());
        
        folder.set_hidden(false).expect("Failed to remove hidden");
        assert!(!folder.is_hidden());

        // Test system attribute
        folder.set_system(true).expect("Failed to set system");
        assert!(folder.is_system());
        
        folder.set_system(false).expect("Failed to remove system");
        assert!(!folder.is_system());
    }

    #[test]
    fn test_windows_folder_children_sync() {
        let test_id = "[WIN_FOLDER_SYNC]";
        println!("\n\n\n{} Testing Windows folder metadata synchronization...", test_id);
        
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let mut parent_folder = create_test_folder(&temp_dir, "parent");
        let parent_path = parent_folder.path();
    
        // Print out the children set (should be empty)
        println!("Children set before adding file: {:?}", parent_folder.get_children());
    
        // Create and add a child file
        let file_path = create_test_file_in(&parent_path, "child.txt", b"test content");
        let file = PathType::File(WindowsFile::new(&file_path, Some(parent_path.clone())));
        
        parent_folder.add_child(file.clone()).expect("Failed to add child");
    
        // Print out the children set (should contain one item)
        println!("Children set after adding file: {:?}", parent_folder.get_children());
        assert_eq!(parent_folder.children(), Some(1));
    
        // Remove child and verify both metadata stay in sync
        parent_folder.remove_child(file).expect("Failed to remove child");
    
        // Print out the children set (should be empty again)
        println!("Children set after removing file: {:?}", parent_folder.get_children());
        assert_eq!(parent_folder.children(), Some(0));
    }

    #[test]
    fn test_windows_folder_operations_with_attributes() {
        let test_id = "[WIN_FOLDER_OPS]";
        println!("\n\n\n{} Testing Windows folder operations with attributes...", test_id);
        
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let mut folder = create_test_folder(&temp_dir, "test_folder");

        // Set folder as read-only
        folder.set_read_only(true).expect("Failed to set read-only");

        // Attempt operations that should fail
        assert!(folder.modify_name("new_name".to_string()).is_err());
        
        let file_path = create_test_file_in(&folder.path(), "test.txt", b"content");
        let file = PathType::File(WindowsFile::new(&file_path, Some(folder.path())));
        
        assert!(folder.add_child(file.clone()).is_err());
        assert!(folder.remove_child(file.clone()).is_err());

        // Remove read-only and verify operations now succeed
        folder.set_read_only(false).expect("Failed to remove read-only");
        
        folder.modify_name("new_name".to_string()).expect("Should rename now");
        folder.add_child(file.clone()).expect("Should add child now");
        folder.remove_child(file).expect("Should remove child now");
    }

    #[test]
    fn test_windows_folder_attribute_persistence() {
        let test_id = "[WIN_FOLDER_PERSIST]";
        println!("\n\n\n{} Testing Windows folder attribute persistence...", test_id);
        
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let folder_path = temp_dir.path().join("test_folder");
        fs::create_dir(&folder_path).expect("Failed to create folder");

        // Set attributes
        {
            let mut folder = WindowsFolder::new(&folder_path, None, 0);
            folder.set_hidden(true).expect("Failed to set hidden");
            folder.set_system(true).expect("Failed to set system");
        }

        // Verify attributes persist after reopening
        {
            let folder = WindowsFolder::new(&folder_path, None, 0);
            assert!(folder.is_hidden());
            assert!(folder.is_system());
        }
    }
       
}