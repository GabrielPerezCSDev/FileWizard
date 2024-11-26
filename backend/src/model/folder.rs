// Required imports
use std::path::Path;
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
//use crate::model::path_map::PathMap;
use crate::model::metadata::folder_specific_metadata;
use crate::model::path_type::PathType;

use std::fs;
use crate::model::file::File;
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
        let index = match &parent {
            Some(p) => p.lock().unwrap().index + 1,
            None => 0,
        };
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
        self.children.push(child);
        self.num_children += 1;
        //update meta data
        self.metadata.insert("Number of children".to_string(), self.num_children.to_string());
    }

    // Getter for metadata
    pub fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

   

    // Method to discover immediate children and add them to `children`
    pub fn discover_immediate_children(&mut self) {
        
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
    
    
    


}