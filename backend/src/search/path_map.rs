// path_map.rs

// Required Imports
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Import File and Folder structs
use crate::model::file::File;
use crate::model::folder::Folder;

// Import PathType enum
use crate::model::path_type::PathType;

/// `PathMap` is a custom data structure that manages mappings between URLs and their corresponding `Folder` and `File` entities.
#[derive(Debug)]
pub struct PathMap {
    /// HashMap storing Folder instances with their URLs as keys.
    folders: HashMap<String, Arc<Mutex<Folder>>>,

    /// HashMap storing File instances with their URLs as keys.
    files: HashMap<String, Arc<Mutex<File>>>,
}

impl PathMap {
    /// Creates a new instance of `PathMap` with empty HashMaps.
    pub fn new() -> Self {
        PathMap {
            folders: HashMap::new(),
            files: HashMap::new(),
        }
    }

   
    pub fn add(&mut self, p: PathType) {
        match p {
            PathType::File(file_arc) => {
                let url = file_arc.lock().unwrap().url.clone();
                self.files.insert(url, Arc::clone(&file_arc)); // Shared ownership
            }
            PathType::Folder(folder_arc) => {
                let folder = folder_arc.lock().unwrap();
                let url = folder.url.clone();
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

    
    pub fn get(&self, url: &str) -> Option<Arc<Mutex<PathType>>> {
        if let Some(folder) = self.folders.get(url) {
            return Some(Arc::new(Mutex::new(PathType::Folder(Arc::clone(folder))))); // Wrap Folder
        }
        if let Some(file) = self.files.get(url) {
            return Some(Arc::new(Mutex::new(PathType::File(Arc::clone(file))))); // Wrap File
        }
        None
    }

    pub fn get_folder(&self, url: &str) -> Option<Arc<Mutex<Folder>>> {
        self.folders.get(url).cloned()
    }

    pub fn get_file(&self, url: &str) -> Option<Arc<Mutex<File>>> {
        self.files.get(url).cloned()
    }

    
    pub fn remove(&mut self, url: &str) -> Option<PathType> {
        if let Some(folder) = self.folders.remove(url) {
            return Some(PathType::Folder(folder));
        }
        if let Some(file) = self.files.remove(url) {
            return Some(PathType::File(file));
        }
        None
    }

    /*
    pub fn persist_to_db(&self) {
        // Implementation goes here
    }

    
    pub fn load_from_db(&mut self) {
        // Implementation goes here
    }
    */
    
    pub fn clear(&mut self) {
        self.folders.clear();
        self.files.clear();
    }
    
}