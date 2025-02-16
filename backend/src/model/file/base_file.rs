// Required imports
use std::path::Path;
use std::fs;
use crate::model::folder::base_folder::Folder; // Import Folder since File references Folder
use crate::model::validation;
use std::sync::{Arc, Mutex};
// Import the function that handles file-specific metadata
use crate::model::metadata::base::{CommonMetadata, BaseMetadata};
use std::path::PathBuf;

pub trait BaseFileOps {
    fn is_read_only(&self) -> bool;
    fn update_size(&mut self) -> std::io::Result<()>;
    fn modify_name(&mut self, new_name: String) -> std::io::Result<()>;
    fn modify_extension(&mut self, new_extension: String) -> std::io::Result<()>;
}


#[derive(Clone, Debug)]
pub struct File {
    pub parent: Option<Arc<Mutex<Folder>>>,
    pub metadata: CommonMetadata,
    pub error: Option<String>,
}

/// Implement the File struct
impl File {
    /// Constructor that accepts a Path and creates a File instance
    pub fn new(path: &Path, parent: Option<Arc<Mutex<Folder>>>) -> Self {
        let name = path.file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("")
            .to_string();
    
        // Attempt to create metadata using the new CommonMetadata::new
        let (metadata, error) = match CommonMetadata::new(path) {
            Ok(meta) => (meta, None),
            Err(e) => {
                (CommonMetadata::default_instance(path, &name), Some(format!("{:?}", e)))
            }
        };
    
        println!("Final metadata for {}: {:?}", name, metadata);
        File {
            parent,
            metadata,
            error,
        }
    }

     /// Updates the metadata size by checking the current file size
     pub fn update_size(&mut self) -> std::io::Result<()> {
        let path = self.metadata.path();
        
        // Get the current metadata from the filesystem
        let metadata = fs::metadata(path)?;
        
        // Update our metadata with the new size
        self.metadata.set_size(metadata.len());
        Ok(())
    }

    pub fn modify_name(&mut self, new_name: String) -> std::io::Result<()> {
        // Validate the new name first.
        validation::validate_filename(&new_name)?;
        
        // Get the current path from metadata.
        let current_path = self.metadata.path();
        let parent_dir = current_path.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Cannot modify name of root directory"
            )
        })?;
        
        // Determine current extension.
        let current_extension = current_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        // Build the new path.
        let new_path = if !current_extension.is_empty() {
            parent_dir.join(format!("{}.{}", new_name, current_extension))
        } else {
            parent_dir.join(&new_name)
        };
        
        // Rename the file or folder.
        fs::rename(current_path, &new_path)?;
        
        // Update metadata: change name and the stored path.
        self.metadata.set_name(new_name);
        self.metadata.set_path(new_path);
        Ok(())
    }

    /// Modifies the file extension in the file system and updates metadata
    pub fn modify_extension(&mut self, new_extension: String) -> std::io::Result<()> {
        // Get the current path from metadata.
        let current_path = self.metadata.path();
        let parent_dir = current_path.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Cannot modify extension of root directory"
            )
        })?;
        
        // Build a new path using the current name from metadata.
        let new_path = if new_extension.is_empty() || new_extension == "None" {
            parent_dir.join(self.metadata.name())
        } else {
            parent_dir.join(format!("{}.{}", self.metadata.name(), new_extension))
        };
        
        // Rename the file or folder.
        fs::rename(current_path, &new_path)?;
        
        // Update metadata: change extension and stored path.
        self.metadata.set_extension(new_extension);
        self.metadata.set_path(new_path);
        
        Ok(())
    }

    pub fn path(&self) -> PathBuf {
        self.metadata.path().clone()
    }

}