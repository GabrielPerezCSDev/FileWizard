use crate::search::path_map::PathMap;
use crate::model::path_type::PathType;
use crate::model::folder::Folder;
use crate::model::file::File;
use std::sync::{ Mutex, Arc };
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Search {
    root_search_directory: Option<String>, //current directory being employed for search
    search_root: Option<Arc<Mutex<Folder>>>, // Root node for the search tree
    current_dir: Option<Arc<Mutex<Folder>>>, // current node for the search tree
    path_map: Mutex<PathMap>,
    frontier_map: Mutex<HashMap<String, Vec<Arc<Mutex<Folder>>>>>,
}

impl Search {
    pub fn new() -> Self {
        println!("[Search] created");
        Self {
            root_search_directory: None,
            search_root: None,
            current_dir: None,
            path_map: Mutex::new(PathMap::new()),
            frontier_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn initialize_search(&mut self) {
        // Ensure that a directory is set
        let directory: String = match &self.root_search_directory {
            Some(dir) => dir.clone(),
            None => {
                println!("[Search] No root directory set. Initialization aborted.");
                return;
            }
        };

        // Check for existence in the path_map
        {
            let mut path_map = self.path_map.lock().unwrap();

            if !path_map.contains(&directory) {
                println!("[Search] Directory '{}' not found in path_map. Adding entry.", directory);

                // Create and add the root folder to the path_map
                let new_folder = Folder::new(
                    std::path::Path::new(&directory),
                    None, // Root node has no parent
                    0 // Root node index
                );

                path_map.add(PathType::Folder(Arc::clone(&new_folder)));

                // Set the root folder as `search_root`
                self.search_root = Some(new_folder);
            } else {
                println!("[Search] Directory '{}' found in path_map.", directory);

                // Retrieve the folder from path_map and set as search_root
                self.search_root = path_map.get_folder(&directory);
            }
        }

        // Assign `current_dir` to `search_root` during initialization
        if let Some(search_root) = &self.search_root {
            self.current_dir = Some(Arc::clone(search_root));
            println!(
                "[Search] Current directory set to '{}'.",
                search_root.lock().unwrap().to_string()
            );
        } else {
            println!("[Search] Failed to set current directory to search root.");
            return; // Abort if we couldn't set current_dir
        }

        // Check for the root directory's frontier list in the frontier_map
        {
            let mut frontier_map = self.frontier_map.lock().unwrap();

            if let Some(frontier_list) = frontier_map.get_mut(&directory) {
                if frontier_list.is_empty() {
                    println!("[Search] Frontier list for '{}' is empty. Search is complete.", directory);
                    return; // Fully searched, nothing more to do
                } else {
                    println!(
                        "[Search] Frontier list for '{}' has {} entries. Continuing search.",
                        directory,
                        frontier_list.len()
                    );
                }
            } else {
                println!("[Search] Frontier list for '{}' does not exist. Initializing.", directory);

                // Create and populate a new frontier list for the root directory
                let new_list = frontier_map.entry(directory.clone()).or_insert_with(Vec::new);

                if let Some(search_root) = &self.search_root {
                    // Pass the Arc<Mutex<Folder>> directly to discover_immediate_children
                    self.discover_immediate_children(search_root, new_list);
                    println!(
                        "[Search] Frontier list for '{}' initialized with {} entries.",
                        directory,
                        new_list.len()
                    );
                }
            }
        }

        // Trigger the execute_search method
        self.execute_search();
    }

    pub fn execute_search(&mut self) {
        // Step 1: Validate search root
        let search_root = match &self.search_root {
            Some(root) => Arc::clone(root), // Clone the Arc to extend its lifetime
            None => {
                println!("[Search] No search root defined.");
                return; // Exit early
            }
        };

        // Step 2: Lock the search root's folder
        //let mut folder = search_root.lock().unwrap();

        // Step 3: Get or create the frontier list for the root directory
        let root_directory_key = self.root_search_directory
            .as_ref()
            .expect("[Search] Root search directory is not set.")
            .clone();

        let mut frontier_map = self.frontier_map.lock().unwrap();
        let frontier_list = frontier_map.entry(root_directory_key.clone()).or_insert_with(Vec::new);

        // If the frontier list exists but is empty, suspend the search
        if frontier_list.is_empty() {
            //println!("[Search] Frontier list for '{}' is empty. Suspending search.", root_directory_key);
            return; // Exit early without continuing
        }

        // Print the current size of the frontier list
        /*
        println!(
            "[Search] Frontier list for '{}' now has {} entries.",
            root_directory_key,
            frontier_list.len()
        );
        */

        // Step 4: Sort the frontier list
        self.sort_frontier_list(frontier_list);

        // Step 5: Pop the next entry and assign it to `current_dir`
        if let Some(next_folder) = frontier_list.pop() {
            self.current_dir = Some(next_folder);
            if let Some(current_dir_arc) = &self.current_dir {
                if let Ok(current_folder) = current_dir_arc.lock() {
                    /*
                    println!(
                        "\n\n\n\n[Search] Current directory updated to {}",
                        current_folder.name
                    );
                    */
                } else {
                    println!("[Search] Failed to lock the current directory.");
                }
            }
        } else {
            println!("[Search] No more entries in the frontier list.");
            self.current_dir = None;
        }

        // Step 6: Discover children and add new to be discovered by frontier nodes
        if let Some(current_dir) = &self.current_dir {
            // Pass the `Arc<Mutex<Folder>>` reference directly to `discover_immediate_children`
            self.discover_immediate_children(current_dir, frontier_list);
        } else {
            println!("[Search] No current directory set. Cannot discover children.");
        }
    }

    pub fn discover_immediate_children(
        &self,
        folder: &Arc<Mutex<Folder>>,
        frontier_list: &mut Vec<Arc<Mutex<Folder>>>
    ) {
        //println!("[Search] discovering the children");
        let path_string = {
            let folder_guard = folder.lock().unwrap(); // Lock the folder to access its contents
            folder_guard.url.clone() // Clone the URL to release the lock
        };

        let path = Path::new(&path_string); // Use the cloned URL to create the path

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        //println!("[Search] adding a folder");
                        let new_folder = Folder::new(
                            entry.path().as_path(),
                            Some(Arc::clone(folder)), // Pass the actual reference
                            folder.lock().unwrap().index + 1
                        );
                        //println!("[Search] created the new folder");
                        frontier_list.push(Arc::clone(&new_folder));
                        //println!("[Search] added the new fodler to the frontier list");
                        {
                            let mut folder_guard = folder.lock().unwrap();
                            let path_type = PathType::Folder(Arc::clone(&new_folder));
                            folder_guard.add_child(path_type);
                        }
                        // println!("[Search] After adding the child");
                    } else {
                        //println!("[Search] adding a file");
                        let new_file = Arc::new(
                            Mutex::new(
                                File::new(
                                    entry.path().as_path(),
                                    Some(Arc::clone(folder)) // Pass the actual reference
                                )
                            )
                        );
                        let path_type = PathType::File(Arc::clone(&new_file));

                        // Add the new file to the current folder's children
                        {
                            let mut folder_guard = folder.lock().unwrap();
                            folder_guard.add_child(path_type.clone());
                        }
                        self.add_path(path_type);
                    }
                }
            }
        } else {
            //  println!("[Error] Unable to read directory: {}", folder.lock().unwrap().url);
        }
        //  println!("[Search] finished discovering the children swag");
    }

    pub fn sort_frontier_list(&self, frontier_list: &mut Vec<Arc<Mutex<Folder>>>) {
        //println!("[Search] Shuffled frontier list");
        //for now just shufle the entries (will impelment a better sorting algo later with heursitcs)
        // Shuffle the entries in the frontier list
        let mut rng = thread_rng();
        frontier_list.shuffle(&mut rng);
    }

    pub fn assign_next_frontier_to_current(&mut self, frontier_list: &mut Vec<Arc<Mutex<Folder>>>) {
        if let Some(next_folder) = self.pop_frontier_entry(frontier_list) {
            self.current_dir = Some(next_folder); // Assign the popped entry to current_dir
            // println!("[Search] Current directory updated to the next frontier node.");
        } else {
            // println!("[Search] Frontier list is empty. No further directories to process.");
            self.current_dir = None; // Clear current_dir if no entries remain
        }
    }

    pub fn pop_frontier_entry(
        &mut self,
        frontier_list: &mut Vec<Arc<Mutex<Folder>>>
    ) -> Option<Arc<Mutex<Folder>>> {
        if !frontier_list.is_empty() {
            // Remove and return the first element
            Some(frontier_list.remove(0))
        } else {
            None
        }
    }

    pub fn add_path(&self, path: PathType) {
        let mut map = self.path_map.lock().unwrap();
        map.add(path);
    }

    /// Overwrite the current directory
    pub fn set_root_search_directory(&mut self, root_search_directory: String) -> bool {
        // Normalize the directory path
        let normalized_directory = Self::normalize_directory_path(&root_search_directory);

        // Validate the normalized path
        if Self::is_valid_path(&normalized_directory) {
            self.root_search_directory = Some(normalized_directory);
            true // Successfully set the path
        } else {
            println!("[Error] Invalid directory path: {}", root_search_directory);
            false // Failed to set the path
        }
    }

    /// Get the current directory (immutable)
    pub fn get_root_search_directory(&self) -> Option<String> {
        self.root_search_directory.clone() // Clone to avoid ownership issues
    }

    /// Normalize a directory path based on the OS.
    fn normalize_directory_path(path: &str) -> String {
        if cfg!(target_os = "windows") {
            // Normalize Windows paths (e.g., "C:" to "C:\")
            if path.ends_with(':') {
                format!("{}\\", path)
            } else {
                path.replace('/', "\\") // Convert slashes to backslashes
            }
        } else {
            // Normalize Unix-like paths (macOS/Linux)
            path.replace('\\', "/") // Convert backslashes to slashes
        }
    }

    /// Check if a path is valid and exists.
    fn is_valid_path(path: &str) -> bool {
        Path::new(path).exists()
    }
}
