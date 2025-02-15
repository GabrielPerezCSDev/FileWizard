use crate::search::path_map::PathMap;
use crate::model::path_type::PathType;
use crate::model::folder::base_folder::Folder; // Folder is defined in model/folder/base_folder.rs
use crate::model::file::base_file::File;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::path::PathBuf; // Bring PathBuf into scope
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Search {
    /// The root search directory as a PathBuf.
    pub root_search_directory: Option<PathBuf>,
    /// The root Folder node for the search tree.
    pub search_root: Option<Arc<Mutex<Folder>>>,
    /// The current Folder node for the search tree.
    pub current_dir: Option<Arc<Mutex<Folder>>>,
    /// A mapping of URLs to File/Folder objects (with updated metadata).
    pub path_map: Mutex<PathMap>,
    /// A mapping used to track frontier folders during the search.
    pub frontier_map: Mutex<HashMap<String, Vec<Arc<Mutex<Folder>>>>>,
}

impl Search {
    
}
