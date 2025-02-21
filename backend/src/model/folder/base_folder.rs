use std::path::Path;
use std::io;
use std::fs;
use std::collections::HashSet;
use crate::model::metadata::base::BaseMetadata;
use crate::model::file_system::path_type::PathType;
use crate::model::validation;
use std::path::PathBuf;
use crate::model::file::base_file::BaseFileOps;

pub trait BaseFolderOps<T: BaseMetadata> {
    /// Renames the folder to a new name on disk
    fn modify_name(&mut self, new_name: String) -> io::Result<()>;

    /// Adds a child (file or folder) to this folder
    fn add_child(&mut self, child: PathType) -> io::Result<()>;

    /// Removes a child, returning the child's path if found
    fn remove_child(&mut self, child: PathType) -> Result<PathBuf, io::Error>;

    /// Returns a formatted string with the folder's metadata
    fn get_formatted_metadata(&self) -> String;

    /// Returns the metadata implementation
    fn get_metadata(&self) -> &T;
    fn get_metadata_mut(&mut self) -> &mut T;

    /// Increase or decrease the folder's total size
    fn update_size(&mut self, delta: i64);  // Combined add/subtract with signed integer

    //setters
    fn set_name(&mut self, name: String);
    fn set_path(&mut self, path: PathBuf);
    fn set_size(&mut self, size: u64);
    fn set_children(&mut self, count: u64);
}

#[derive(Clone, Debug)]
pub struct Folder<T: BaseMetadata> {
    children: HashSet<PathBuf>,
    parent: Option<PathBuf>,
    metadata: T,
    index: i32,
}

#[derive(Debug)]
pub enum FolderError {
    DuplicateChild(String),    // Child with this name already exists
    CircularReference(String), // Would create a circular reference
    InvalidPath(String),       // Path is not a valid child of this folder
}

#[derive(Debug)]
pub enum RemoveError {
    NotFound(String),
    InvalidChild(String),
    PermissionDenied(String),
}


impl From<FolderError> for io::Error {
    fn from(error: FolderError) -> Self {
        match error {
            FolderError::DuplicateChild(msg) => io::Error::new(io::ErrorKind::AlreadyExists, msg),
            FolderError::CircularReference(msg) => io::Error::new(io::ErrorKind::InvalidInput, msg),
            FolderError::InvalidPath(msg) => io::Error::new(io::ErrorKind::InvalidInput, msg),
        }
    }
}

impl From<RemoveError> for io::Error {
    fn from(error: RemoveError) -> Self {
        match error {
            RemoveError::NotFound(msg) => io::Error::new(io::ErrorKind::NotFound, msg),
            RemoveError::InvalidChild(msg) => io::Error::new(io::ErrorKind::InvalidInput, msg),
            RemoveError::PermissionDenied(msg) => io::Error::new(io::ErrorKind::PermissionDenied, msg),
        }
    }
}


impl<T: BaseMetadata> Folder<T> {
    /// Creates a new Folder<T> instance
    pub fn new(
        parent: Option<PathBuf>,
        pwd_index: i32,
        metadata: T
    ) -> Self {
        Folder {
            parent,
            children: HashSet::new(),
            metadata,          // Single metadata instance
            index: pwd_index,
        }
    }

    // Simple getters
    pub fn get_parent(&self) -> Option<&PathBuf> {
        self.parent.as_ref()
    }

    pub fn get_children(&self) -> &HashSet<PathBuf> {
        &self.children
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }
    
    /// Access to the metadata
    pub fn get_metadata(&self) -> &T {
        &self.metadata
    }

    pub fn get_metadata_mut(&mut self) -> &mut T {
        &mut self.metadata
    }
}

impl<T: BaseMetadata> BaseFolderOps<T> for Folder<T> {
    fn modify_name(&mut self, new_name: String) -> io::Result<()> {
        validation::validate_path_name(&new_name)?;
        let current_path = self.metadata.path();

        let parent_dir = current_path
            .parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "No parent directory available"))?;

        let new_path = parent_dir.join(&new_name);
        fs::rename(&current_path, &new_path)?;

        self.set_name(new_name);
        self.set_path(new_path);
        Ok(())
    }

    fn add_child(&mut self, child: PathType) -> io::Result<()> {
        // Extract child's path + size
        let (child_path, child_size) = match &child {
            PathType::File(file) => {
                (file.get_metadata().path().to_path_buf(), file.get_metadata().size())
            },
            PathType::Folder(folder) => {
                (folder.get_metadata().path().to_path_buf(), folder.get_metadata().size())
            },
            PathType::None => return Ok(()),
        };

        // For duplicate detection
        let child_name = child_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

        // Validate path and check constraints
        self.validate_child_path(&child_path)?;
        self.check_duplicate(child_name)?;
        self.check_circular_reference(&child_path)?;

        // Insert path into the children set
        if !self.children.insert(child_path) {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Child already exists"));
        }

        // Update metadata
        if let Some(current_children) = self.metadata.children() {
            self.metadata.set_children(current_children + 1);
        }
        self.update_size(child_size as i64);

        Ok(())
    }

    fn remove_child(&mut self, child: PathType) -> Result<PathBuf, io::Error> {
        let (child_path, child_size) = match &child {
            PathType::File(file) => {
                (file.get_metadata().path().to_path_buf(), file.get_metadata().size())
            },
            PathType::Folder(folder) => {
                (folder.get_metadata().path().to_path_buf(), folder.get_metadata().size())
            },
            PathType::None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cannot remove None PathType")),
        };
    
        if self.children.remove(&child_path) {
            if let Some(current_children) = self.metadata.children() {
                self.metadata.set_children(current_children.saturating_sub(1));
            }
            
            // Update size using the size from PathType's metadata
            self.update_size(-(child_size as i64));
            
            Ok(child_path)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Child not found"))
        }
    }

    fn get_formatted_metadata(&self) -> String {
        self.metadata.formatted_metadata()
    }

     // New setter implementations that delegate to metadata
     fn set_name(&mut self, name: String) {
        self.metadata.set_name(name);
    }

    fn set_path(&mut self, path: PathBuf) {
        self.metadata.set_path(path);
    }

    fn set_size(&mut self, size: u64) {
        self.metadata.set_size(size);
    }

    fn set_children(&mut self, count: u64) {
        self.metadata.set_children(count);
    }

    fn get_metadata(&self) -> &T {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut T {
        &mut self.metadata
    }

    fn update_size(&mut self, delta: i64) {
        let current_size = self.metadata.size();
        let new_size = if delta >= 0 {
            current_size + delta as u64
        } else {
            current_size.saturating_sub((-delta) as u64)
        };
        self.set_size(new_size);
    }
}

impl<T: BaseMetadata> Folder<T> {
    /// Validates if a path is a valid child of this folder
    fn validate_child_path(&self, child_path: &Path) -> Result<(), FolderError> {
        let parent_path = self.metadata.path();
        if !child_path.starts_with(&parent_path) {
            return Err(FolderError::InvalidPath(
                format!("Child path {:?} is not within parent folder {:?}", child_path, parent_path)
            ));
        }
        if child_path.parent().map_or(false, |p| p != parent_path) {
            return Err(FolderError::InvalidPath(
                format!("Child path {:?} is not an immediate child of {:?}", child_path, parent_path)
            ));
        }
        Ok(())
    }

    /// Checks for duplicate children
    fn check_duplicate(&self, child_name: &str) -> Result<(), FolderError> {
        if self.children.iter().any(|child_path| {
            child_path.file_name()
                .and_then(|os_str| os_str.to_str())
                .map(|n| n == child_name)
                .unwrap_or(false)
        }) {
            return Err(FolderError::DuplicateChild(
                format!("Child with name '{}' already exists", child_name)
            ));
        }
        Ok(())
    }

    /// Checks for circular references in folder structure
    fn check_circular_reference(&self, child_path: &Path) -> Result<(), FolderError> {
        if child_path == self.metadata.path() {
            return Err(FolderError::CircularReference(
                "Child path is the same as parent folder".to_string()
            ));
        }
        Ok(())
    }
}

