// Import necessary types
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::model::metadata::base::BaseMetadata;
use crate::model::file::base_file::File; // Import File from the file module
use crate::model::folder::base_folder::Folder; // Import Folder from the folder module
use serde::{Serialize, Deserialize};
/// PathType enum to differentiate between Files, Folders, and None
#[derive(Clone, Debug)]
pub enum PathType {
    File(Arc<Mutex<File>>),    // Represents a file
    Folder(Arc<Mutex<Folder>>), // Represents a folder
    None,                      // Represents no valid path (e.g., when something is missing or inaccessible)
}

impl fmt::Display for PathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathType::File(file_arc) => {
                let file = file_arc.lock().unwrap();
                write!(
                    f,
                    "File: {} (size: {} bytes)",
                    file.name,
                    file.metadata.size()
                )
            }
            PathType::Folder(folder_arc) => {
                let folder = folder_arc.lock().unwrap();
                write!(
                    f,
                    "Folder: {} (size: {} bytes)",
                    folder.name,
                    folder.metadata.size()
                )
            }
            PathType::None => write!(f, "None"),
        }
    }
}