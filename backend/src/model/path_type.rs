// Import necessary types
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::model::file::File; // Import File from the file module
use crate::model::folder::Folder; // Import Folder from the folder module

/// PathType enum to differentiate between Files, Folders, and None
#[derive(Clone, Debug)]
pub enum PathType {
    File(File),                           // Represents a file
    Folder(Arc<Mutex<Folder>>),          // Represents a folder wrapped in Rc<RefCell> for shared mutable ownership
    None,                                 // Represents no valid path (e.g., when something is missing or inaccessible)
}

// Implement Display for PathType
impl fmt::Display for PathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathType::File(file) => write!(f, "File: {}", file.name), // Assuming `File` has a `name` field
            PathType::Folder(folder) => {
                let folder = folder.lock().unwrap();
                write!(f, "Folder: {}", folder.name) // Assuming `Folder` has a `name` field
            }
            PathType::None => write!(f, "None"),
        }
    }
}