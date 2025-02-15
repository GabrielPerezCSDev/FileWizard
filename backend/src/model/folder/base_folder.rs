use std::path::Path;
use std::sync::{Arc, Mutex};
use std::fmt;
use crate::model::path_type::PathType;
use crate::model::metadata::base::{CommonMetadata, MetadataError};
// The Folder struct definition using the new metadata type.
#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub url: String,
    pub children: Vec<PathType>, // Children can be files or folders.
    pub parent: Option<Arc<Mutex<Folder>>>,
    pub metadata: CommonMetadata,
    pub index: i32,
    pub num_children: i32,
}

impl Folder {
    // Constructor to create a new Folder using the CommonMetadata type.
    pub fn new(
        path: &Path,
        parent: Option<Arc<Mutex<Folder>>>,
        pwd_index: i32
    ) -> Arc<Mutex<Self>> {
        let name = if path.parent().is_none() {
            // Root folder: use the full path string.
            path.to_str().unwrap_or("").to_string()
        } else {
            path.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("")
                .to_string()
        };
        let url = path.to_str().unwrap_or("").to_string();

        // Attempt to create metadata using the new CommonMetadata::new.
        // If it fails, create a default metadata instance with exists set to false.
        let metadata = match CommonMetadata::new(path) {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("Error retrieving metadata for {}: {:?}", name, e);
                CommonMetadata::default_instance(path, &name)
            }
        };

        // Create and return the Folder wrapped in Arc<Mutex>.
        let folder = Folder {
            name,
            url,
            children: Vec::new(),
            parent,
            metadata,
            index: pwd_index,
            num_children: 0,
        };

        Arc::new(Mutex::new(folder))
    }
}