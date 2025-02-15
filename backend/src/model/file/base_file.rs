// Required imports
use std::path::Path;
use std::fs;
use std::fmt;
use std::time::SystemTime;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::model::folder::base_folder::Folder; // Import Folder since File references Folder
use std::sync::{Arc, Mutex};
// Import the function that handles file-specific metadata
use crate::model::metadata::base::{CommonMetadata, MetadataError};


#[derive(Clone, Debug)]
pub struct File {
    pub name: String,
    pub url: String,
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
        let url = path.to_str().unwrap_or("").to_string();
    
        // Attempt to create metadata using the new CommonMetadata::new
        let (metadata, error) = match CommonMetadata::new(path) {
            Ok(meta) => (meta, None),
            Err(e) => {
                (CommonMetadata::default_instance(path, &name), Some(format!("{:?}", e)))
            }
        };
    
        println!("Final metadata for {}: {:?}", name, metadata);
        File {
            name,
            url,
            parent,
            metadata,
            error,
        }
    }
}