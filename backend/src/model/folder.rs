// Required imports
use std::path::Path;
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
//use crate::model::path_map::PathMap;

use crate::model::metadata::folder_specific_metadata;
use crate::model::metadata::update_size as update_metadata_size;
use crate::model::path_type::PathType;


// The Folder struct definition
#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub url: String,
    pub children: Vec<PathType>, // Children can be files or folders
    pub parent: Option<Arc<Mutex<Folder>>>, // Use Rc<RefCell> for parent reference
    pub metadata: HashMap<String, String>,
    pub index: i32,
    pub num_children: i32,
}

impl Default for Folder {
    fn default() -> Self {
        Folder {
            name: String::from("default"),
            url: String::from("default_url"),
            children: Vec::new(),
            parent: None,
            metadata: HashMap::new(),
            index: 0,
            num_children: 0,
        }
    }
}

impl Folder {
    // Constructor to create a new Folder
    pub fn new(
        path: &Path,
        parent: Option<Arc<Mutex<Folder>>>,
        pwd_index: i32
    ) -> Arc<Mutex<Self>> {
        let name = if path.parent().is_none() {
            // If there is no parent, it is the root directory.
            path.to_str().unwrap_or("").to_string()
        } else {
            path.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("")
                .to_string()
        };
        let url = path.to_str().unwrap_or("").to_string();
        let cloned_url = url.clone();

        // Borrow the index from the parent folder if it exists
        let index = pwd_index;
        
        let num_children = 0;
        let mut folder = Folder {
            name,
            url: cloned_url.clone(),
            children: Vec::new(),
            parent,
            metadata: HashMap::new(),
            index,
            num_children,
        };

        // Populate folder-specific metadata
        folder_specific_metadata(&mut folder.metadata, path);

        // Wrap the folder in Arc<Mutex> for shared ownership and thread-safe mutability
        let folder_arc = Arc::new(Mutex::new(folder));

        folder_arc
    }

    // Add a child to the folder's children
    pub fn add_child(&mut self, child: PathType) {
        self.children.push(child.clone());
        self.num_children += 1;
        //update meta data
        self.metadata.insert("Number of children".to_string(), self.num_children.to_string());
        // Add the raw size of the child to the folder's size
        let child_size = match &child {
        PathType::File(file_arc) => {
            // Lock the file and extract the raw_size
            if let Ok(file) = file_arc.lock() {
                file.get_raw_size()
            } else {
                0
            }
        }
        PathType::Folder(folder_arc) => {
            // Lock the folder and extract the raw_size
            if let Ok(folder) = folder_arc.lock() {
                folder.get_raw_size()
            } else {
                0
            }
        }, &PathType::None => todo!()
    };
        //println!("[Folder] floating value to this folder {}: {}",self.name, child_size);
        self.update_size(child_size);
        

        // Propagate the size to all parent folders
    let mut current_parent = self.parent.clone();
    while let Some(parent_arc) = current_parent {
        if let Ok(mut parent) = parent_arc.lock() {
            //println!("[Folder] Updating parent folder '{}'", parent.name);
            parent.update_size(child_size);
            current_parent = parent.parent.clone(); // Move up to the next parent
        } else {
            //println!("[Folder] Failed to lock parent folder.");
            break;
        }
    }

    }

    // Getter for metadata
    pub fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn update_size(&mut self, size: u64) {
        let current_size = self
        .metadata
        .get("raw_size")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

        // Calculate the new raw size
        let new_size = current_size + size;
        update_metadata_size(&mut self.metadata, new_size);
        // Debugging output
        /*
        println!(
        "[Folder] Updated size for folder '{}' after adding {}: raw_size={:?} (formatted={:?})",
        self.name,
        size,
        self.metadata.get("raw_size"),
        self.metadata.get("size")
        );
        */
    }

    
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str("\n====================\n");
        result.push_str(&format!("Folder Name: {}\n", self.name));
        result.push_str(&format!("URL: {}\n", self.url));
        result.push_str(&format!("Index: {}\n", self.index));
        result.push_str(&format!("Number of Children: {}\n", self.num_children));
        
        if let Some(parent) = &self.parent {
            let parent_name = parent.lock().unwrap().name.clone();
            result.push_str(&format!("Parent: {}\n", parent_name));
        } else {
            result.push_str("Parent: None\n");
        }
        
        result.push_str("Metadata:\n");
        for (key, value) in &self.metadata {
            result.push_str(&format!("  {}: {}\n", key, value));
        }

        result.push_str("Children:\n");
        for (i, child) in self.children.iter().enumerate() {
            result.push_str(&format!("  [{}] {}\n", i + 1, child.to_string()));
        }

        result.push_str("====================\n");

        result
    }
    
    
    pub fn get_raw_size(&self) -> u64 {
        self.metadata
            .get("raw_size")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0)
    }


}