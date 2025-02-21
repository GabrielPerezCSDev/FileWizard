use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use std::path::Path;

/// Custom error type for metadata operations
#[derive(Debug)]
pub enum MetadataError {
    InvalidPath(String),
    IoError(std::io::Error),
    SystemTimeError(std::time::SystemTimeError),
}

impl From<std::io::Error> for MetadataError {
    fn from(error: std::io::Error) -> Self {
        MetadataError::IoError(error)
    }
}

impl From<std::time::SystemTimeError> for MetadataError {
    fn from(error: std::time::SystemTimeError) -> Self {
        MetadataError::SystemTimeError(error)
    }
}

// Base metadata trait that all platforms must implement
pub trait BaseMetadata {
    // Getters
    fn size(&self) -> u64;
    fn children(&self) -> Option<u64>;
    fn created(&self) -> SystemTime;
    fn modified(&self) -> SystemTime;
    fn is_directory(&self) -> bool;
    fn is_file(&self) -> bool;
    fn name(&self) -> String;
    fn path(&self) -> PathBuf;
    fn extension(&self) -> String;
    fn exists(&self) -> bool;
    fn formatted_metadata(&self) -> String;

    // Add required setters
    fn set_size(&mut self, new_size: u64);
    fn set_children(&mut self, new_children: u64);
    fn set_modified(&mut self, new_modified: SystemTime);
    fn set_exists(&mut self, exists: bool);
    fn set_name(&mut self, new_name: String);
    fn set_extension(&mut self, new_extension: String);
    fn set_path(&mut self, new_path: PathBuf);
    // Add a constructor requirement
    fn new(path: &Path) -> Result<Self, MetadataError> where Self: Sized;
}

// Common metadata struct that implements BaseMetadata
#[derive(Debug, Clone)]
pub struct CommonMetadata {
    size: u64,
    children: Option<u64>,
    created: SystemTime,
    modified: SystemTime,
    is_dir: bool,
    name: String,
    path: PathBuf,
    extension: String,
    exists: bool,
}

impl CommonMetadata {
    /// Creates a new CommonMetadata instance from a path
    pub fn new(path: &Path) -> Result<Self, MetadataError> {
        // First check if path exists.
        if !path.exists() {
            return Err(MetadataError::InvalidPath(format!("Path does not exist: {:?}", path)));
        }

        let metadata = fs::metadata(path)?;

        // Get extension if it exists, else "None" for files, "" for directories.
        let extension = if metadata.is_dir() {
            String::new() // Empty string for directories.
        } else {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            if file_name.ends_with(".") {
                "None".to_string()
            } else {
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.to_string())
                    .unwrap_or_else(|| "None".to_string())
            }
        };

        Ok(Self {
            size: metadata.len(),
            children: if metadata.is_dir() { Some(0) } else { None },
            created: metadata.created()?,
            modified: metadata.modified()?,
            is_dir: metadata.is_dir(),
            name: path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
            extension,
            exists: true,
            path: path.to_path_buf(),
        })
    }

    pub fn default_instance(path: &Path, name: &str) -> Self {
        Self {
            size: 0,
            children: if path.is_dir() { Some(0) } else { None },
            created: SystemTime::now(),
            modified: SystemTime::now(),
            is_dir: path.is_dir(),
            name: name.to_string(),
            extension: String::new(),
            exists: false,
            path: path.to_path_buf(),
        }
    }

}

impl BaseMetadata for CommonMetadata {
    fn size(&self) -> u64 {
        self.size
    }

    fn children(&self) -> Option<u64> {
        self.children
    }

    fn created(&self) -> SystemTime {
        self.created
    }

    fn modified(&self) -> SystemTime {
        self.modified
    }

    fn is_directory(&self) -> bool {
        self.is_dir
    }

    fn is_file(&self) -> bool {
        !self.is_dir
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn exists(&self) -> bool {
        self.exists
    }

    fn extension(&self) -> String {
        self.extension.clone()
    }

    // setter implementations
    fn set_size(&mut self, new_size: u64) {
        self.size = new_size;
    }

    fn set_children(&mut self, new_children: u64) {
        if self.is_dir {
            self.children = Some(new_children);
        }
    }

    fn set_modified(&mut self, new_modified: SystemTime) {
        self.modified = new_modified;
    }

    fn set_exists(&mut self, exists: bool) {
        self.exists = exists;
    }

    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn set_extension(&mut self, new_extension: String) {
        self.extension = new_extension;
    }

    fn set_path(&mut self, new_path: PathBuf) {
        self.path = new_path;
    }



    //constructor implementation
    fn new(path: &Path) -> Result<Self, MetadataError> {
        CommonMetadata::new(path)
    }

    fn formatted_metadata(&self) -> String {
        format!(
            "
            CommonMetadata:
            Name: {}
            Path: {}
            Extension: {}
            Size: {} bytes
            Children: {:?}
            Created: {:?}
            Modified: {:?}
            Exists: {}
            ",
            self.name,
            self.path.display(),
            self.extension,
            self.size,
            self.children,
            self.created,
            self.modified,
            self.exists,
        )
    }
}
