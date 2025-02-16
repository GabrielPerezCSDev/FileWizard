// path_map.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Import File and Folder structs
use crate::model::file::base_file::File;
use crate::model::folder::base_folder::Folder;
use crate::model::metadata::base::BaseMetadata;

// Import PathType enum
use crate::model::path_type::PathType;

/// `PathMap` is a custom data structure that manages mappings between URLs and their corresponding `Folder` and `File` entities.
#[derive(Debug)]
pub struct PathMap {
    /// HashMap storing Folder instances with their URLs as keys.
    folders: HashMap<String, Arc<Mutex<Folder>>>,
    /// HashMap storing File instances with their URLs as keys.
    files: HashMap<String, Arc<Mutex<File>>>,
    /// HashMap to track files or folders that had errors during metadata retrieval.
    bad_paths: HashMap<String, (Arc<Mutex<PathType>>, String)>,
}

impl PathMap {
    /// Creates a new instance of `PathMap` with empty HashMaps.
    pub fn new() -> Self {
        PathMap {
            folders: HashMap::new(),
            files: HashMap::new(),
            bad_paths: HashMap::new(),
        }
    }

    /// Adds a PathType to the map. If the item has an error in its metadata,
    /// it is also tracked in the `bad_paths` map.
    pub fn add(&mut self, p: PathType) {
        match p {
            PathType::File(file_arc) => {
                let file = file_arc.lock().unwrap();
                let url = file.metadata.path().to_string_lossy().into_owned();
                // Check if file has an error based on the new metadata design.
                if let Some(err) = &file.error {
                    self.bad_paths.insert(
                        url.clone(),
                        (Arc::new(Mutex::new(PathType::File(Arc::clone(&file_arc)))), err.clone())
                    );
                }
                self.files.insert(url, Arc::clone(&file_arc)); // Shared ownership
            }
            PathType::Folder(folder_arc) => {
                let folder = folder_arc.lock().unwrap();
                let url = folder.url.clone();
                // For folders, you might check for errors similarly if needed.
                self.folders.insert(url, Arc::clone(&folder_arc));
            }
            PathType::None => {
                println!("[PathMap] Warning: Attempted to add PathType::None.");
            }
        }
    }

    pub fn contains(&self, url: &str) -> bool {
        self.folders.contains_key(url) || self.files.contains_key(url)
    }

    pub fn contains_folder(&self, url: &str) -> bool {
        self.folders.contains_key(url)
    }

    pub fn contains_file(&self, url: &str) -> bool {
        self.files.contains_key(url)
    }

    /// Returns an Option containing the requested PathType (wrapped in Arc/Mutex).
    pub fn get(&self, url: &str) -> Option<Arc<Mutex<PathType>>> {
        if let Some(folder) = self.folders.get(url) {
            return Some(Arc::new(Mutex::new(PathType::Folder(Arc::clone(folder)))));
        }
        if let Some(file) = self.files.get(url) {
            return Some(Arc::new(Mutex::new(PathType::File(Arc::clone(file)))));
        }
        None
    }

    pub fn get_folder(&self, url: &str) -> Option<Arc<Mutex<Folder>>> {
        self.folders.get(url).cloned()
    }

    pub fn get_file(&self, url: &str) -> Option<Arc<Mutex<File>>> {
        self.files.get(url).cloned()
    }

    /// Retrieves all items that encountered errors during metadata retrieval.
    pub fn get_bad_paths(&self) -> &HashMap<String, (Arc<Mutex<PathType>>, String)> {
        &self.bad_paths
    }

    /// Removes an item from the map based on its URL.
    pub fn remove(&mut self, url: &str) -> Option<PathType> {
        if let Some(folder) = self.folders.remove(url) {
            return Some(PathType::Folder(folder));
        }
        if let Some(file) = self.files.remove(url) {
            return Some(PathType::File(file));
        }
        // Optionally, also remove from bad_paths if present.
        self.bad_paths.remove(url).map(|(arc, _)| arc.lock().unwrap().clone())
    }

    pub fn clear(&mut self) {
        self.folders.clear();
        self.files.clear();
        self.bad_paths.clear();
    }
}
