use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::model::file_system::path_alias::{FileType, FolderType};
//implements a map of Paths as keys for lookups to file system items 
pub struct FileSystem {
    files: HashMap<PathBuf, FileType>;
    folders: HashMap<PathBuf, FolderType>;
    other: HashSet<PathBuf>;
}

//used for file system operations 
impl FileSystem {

    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            folders: HashMap::new(),
        }
    }
    
    fn add_path(&path: PathType){
        //ensure path does not exist in any map

        //match the path to the correct type and call appropriate map ofr insertion
        
        //access each parent of the path and increment size & number or children


    }
}
