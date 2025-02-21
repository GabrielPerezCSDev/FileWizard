// Required imports
use std::path::{Path, PathBuf};
use std::fs;
use crate::model::validation;
use std::io;
use crate::model::metadata::base::BaseMetadata;

pub trait BaseFileOps<T: BaseMetadata> {
    /// Returns the metadata implementation
    fn get_metadata(&self) -> &T;
    fn get_metadata_mut(&mut self) -> &mut T;

    /// File operations
    fn modify_name(&mut self, new_name: String) -> io::Result<()>;
    fn modify_extension(&mut self, new_extension: String) -> io::Result<()>;
    fn update_size(&mut self) -> io::Result<()>;

    // Metadata setters
    fn set_name(&mut self, name: String);
    fn set_path(&mut self, path: PathBuf);
    fn set_size(&mut self, size: u64);
}


#[derive(Clone, Debug)]
pub struct File<T: BaseMetadata> {
    parent: Option<PathBuf>,
    metadata: T,
}

/// Implement the File struct
impl<T: BaseMetadata> File<T> {
    /// Creates a new File<T> instance
    pub fn new(
        parent: Option<PathBuf>,
        metadata: T     // Take metadata directly like Folder does
    ) -> Self {
        File {
            parent,
            metadata,
        }
    }

    pub fn get_parent(&self) -> Option<&PathBuf> {
        self.parent.as_ref()
    }
}
impl<T: BaseMetadata> BaseFileOps<T> for File<T> {
    fn get_metadata(&self) -> &T {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut T {
        &mut self.metadata
    }

    fn modify_name(&mut self, new_name: String) -> io::Result<()> {
        validation::validate_path_name(&new_name)?;
        
        let current_path = self.metadata.path();
        let parent_dir = current_path.parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Cannot modify name of root directory"))?;
        
        let current_extension = current_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let new_path = if !current_extension.is_empty() {
            parent_dir.join(format!("{}.{}", new_name, current_extension))
        } else {
            parent_dir.join(&new_name)
        };
        
        fs::rename(current_path, &new_path)?;
        
        self.set_name(new_name);
        self.set_path(new_path);
        Ok(())
    }

    fn modify_extension(&mut self, new_extension: String) -> io::Result<()> {
        let current_path = self.metadata.path();
        let parent_dir = current_path.parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Cannot modify extension of root directory"))?;
        
        let new_path = if new_extension.is_empty() || new_extension == "None" {
            parent_dir.join(self.metadata.name())
        } else {
            parent_dir.join(format!("{}.{}", self.metadata.name(), new_extension))
        };
        
        fs::rename(current_path, &new_path)?;
        
        self.metadata.set_extension(new_extension);
        self.set_path(new_path);
        Ok(())
    }

    fn update_size(&mut self) -> io::Result<()> {
        let path = self.metadata.path();
        let metadata = fs::metadata(path)?;
        self.set_size(metadata.len());
        Ok(())
    }

    fn set_name(&mut self, name: String) {
        self.metadata.set_name(name);
    }

    fn set_path(&mut self, path: PathBuf) {
        self.metadata.set_path(path);
    }

    fn set_size(&mut self, size: u64) {
        self.metadata.set_size(size);
    }
}