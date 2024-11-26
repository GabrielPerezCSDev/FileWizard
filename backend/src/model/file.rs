// Required imports
use std::path::Path;
use std::collections::HashMap;
use crate::directory::path_map::PathMap;
use crate::directory::folder::Folder; // Import Folder since File references Folder
use std::sync::{Arc, Mutex};
// Import the function that handles file-specific metadata
use crate::directory::metadata::file_specific_metadata;

// The File struct definition
#[derive(Clone)]
pub struct File {
    pub name: String,
    pub url: String,
    pub parent: Option<Arc<Mutex<Folder>>>,
    pub metadata: HashMap<String, String>,
}

// Implement the File struct
impl File {
    // Constructor that accepts a Path and creates a File instance
    pub fn new(path: &Path, parent: Option<Arc<Mutex<Folder>>>) -> Self {
        // Extract the file name from the path
        let name = path.file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("")
            .to_string();

        // Convert the path to a string (URL-like representation)
        let url = path.to_str().unwrap_or("").to_string();

        // Initialize metadata
        let mut metadata = HashMap::new();
        file_specific_metadata(&mut metadata, path);

        // Create and return the file with metadata
        let file = File {
            name,
            url: url.clone(),
            parent,
            metadata,
        };

        file
    }

    pub fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
}