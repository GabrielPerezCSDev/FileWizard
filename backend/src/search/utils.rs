use crate::search::path_map::PathMap;
use crate::model::path_type::PathType;
use crate::model::folder::Folder;
use std::sync::{Mutex, Arc};

#[derive(Debug)]
pub struct Search {
    curr_directory: Option<String>,
    search_root: Option<Arc<Mutex<Folder>>>, // Root node for the search tree
    path_map: Mutex<PathMap>,

}

impl Search {
    pub fn new() -> Self {
        println!("[Search] created");
        Self { 
            curr_directory: None,
            search_root: None,
            path_map: Mutex::new(PathMap::new()),
        }
    }

    pub fn initialize_search(&mut self) {
        // Ensure that a directory is set
        let directory: String = match &self.curr_directory {
            Some(dir) => dir.clone(),
            None => {
                println!("[Search] No directory set. Initialization aborted.");
                return;
            }
        };

        // Lock the path_map for modification
        let mut map = self.path_map.lock().unwrap();

        // Check if the directory exists in the path_map
        if map.contains(&directory) {
            println!("[Search] Entry found in path_map. Setting as root.");
            // Retrieve the folder from path_map and set as search_root
            self.search_root = map.get_folder(&directory);
        } else {
            println!("[Search] Entry not found. Adding to path_map.");
            // Create a new Folder instance
            let new_folder = Folder::new(
                std::path::Path::new(&directory),
                None, // No parent since this is the root
                0,    // Index for root
            );
            
            // Add the new folder to the path_map
            map.add(PathType::Folder(Arc::clone(&new_folder)));
            
            // Set the new folder as search_root
            self.search_root = Some(new_folder);
        }

        // Trigger the execute_search method
        self.execute_search();
    }



    pub fn execute_search(&self) {
        if let Some(search_root) = &self.search_root {
            let folder = search_root.lock().unwrap(); // Safely access the Folder
            println!("[Search] Executing a search with search root: {}", folder.to_string()); 
        } else {
            println!("[Search] No search root defined.");
        }

        //sert current_node = search_root if null

        //while frontier node list is not null

        //find all of current nodes children
        
        //add files staright to the file map

        //add folders to frontier node list
        
        //order them (for now randomly)

        //remove first entry

        //add it to the map for folders

        //update current_node to eb the popped one


    }

    pub fn add_path(&self, path: PathType) {
        let mut map = self.path_map.lock().unwrap();
        map.add(path);
    }

    /// Overwrite the current directory
    pub fn set_curr_directory(&mut self, curr_directory: String) {
        self.curr_directory = Some(curr_directory);
    }

    /// Get the current directory (immutable)
    pub fn get_curr_directory(&self) -> Option<String> {
        self.curr_directory.clone() // Clone to avoid ownership issues
    }
}