// Import necessary types
use std::sync::{Arc, Mutex};
use crate::directory::file::File; // Import File from the file module
use crate::directory::folder::Folder; // Import Folder from the folder module

/// PathType enum to differentiate between Files, Folders, and None
#[derive(Clone)]
pub enum PathType {
    File(File),                           // Represents a file
    Folder(Arc<Mutex<Folder>>),          // Represents a folder wrapped in Rc<RefCell> for shared mutable ownership
    None,                                 // Represents no valid path (e.g., when something is missing or inaccessible)
}