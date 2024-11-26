use crate::search::path_map::PathMap;
use crate::model::path_type::PathType;
use crate::model::folder::Folder;
use std::sync::{Mutex, Arc};

#[derive(Debug)]
pub struct Search {
    root_search_directory: Option<String>, //current directory being employed for search 
    search_root: Option<Arc<Mutex<Folder>>>, // Root node for the search tree
    current_dir: Option<Arc<Mutex<Folder>>>, // current node for the search tree
    path_map: Mutex<PathMap>,
}

impl Search {
    pub fn new() -> Self {
        println!("[Search] created");
        Self { 
            root_search_directory: None,
            search_root: None,
            current_dir: None,
            path_map: Mutex::new(PathMap::new()),
        }
    }

    pub fn initialize_search(&mut self) {
        // Ensure that a directory is set
        let directory: String = match &self.root_search_directory {
            Some(dir) => dir.clone(),
            None => {
                println!("[Search] No directory set. Initialization aborted.");
                return;
            }
        };

        {
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
        }

        // Trigger the execute_search method
        self.execute_search();
    }



    pub fn execute_search(&mut self) {
        if let Some(search_root) = &self.search_root {
            let folder = search_root.lock().unwrap(); // Safely access the Folder
            println!("[Search] Executing a search with search root: {}", folder.to_string()); 
        } else {
            println!("[Search] No search root defined.");
        }

        if self.current_dir.is_none() {
            // Ensure that search_root is not None
            if let Some(search_root) = &self.search_root {
                // Set current_node to point to the same Arc<Mutex<Folder>> as search_root
                self.current_dir = Some(Arc::clone(search_root));
                println!("[Search] Current node initialized to the search root.");
            } else {
                println!("[Search] Search root is None; cannot initialize current node.");
            }
        } else {
            println!("[Search] Current node is already initialized.");
        }
    
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
    pub fn set_root_search_directory(&mut self, root_search_directory: String) {
        self.root_search_directory = Some(root_search_directory);
    }

    /// Get the current directory (immutable)
    pub fn get_root_search_directory(&self) -> Option<String> {
        self.root_search_directory.clone() // Clone to avoid ownership issues
    }
}