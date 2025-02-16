use std::path::Path;
use std::sync::{Arc, Mutex};
use std::io;
use std::fs;
use crate::model::metadata::base::{BaseMetadata, CommonMetadata};
use crate::model::path_type::PathType;

pub trait BaseFolderOps {
    /// Renames the folder to the new name.
    fn modify_name(&mut self, new_name: String) -> io::Result<()>;
    
    /// Adds a child (either a File or Folder) to the folder.
    fn add_child(&mut self, child: crate::model::path_type::PathType);
    
    /// Removes a child based on its URL.
    fn remove_child(&mut self, child_url: &str) -> Option<crate::model::path_type::PathType>;
    
    /// return formatted metadata
    fn get_formatted_metadata(&self) -> String;
    
    fn add_size(&mut self, size: u64);

    fn subtract_size(&mut self, size: u64);
    
}

// The Folder struct definition using the new metadata type.
#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub url: String,
    pub children: Vec<PathType>, // Children can be files or folders.
    pub parent: Option<Arc<Mutex<Folder>>>,
    pub metadata: CommonMetadata,
    pub index: i32,
}

impl Folder {
    // Constructor to create a new Folder using the CommonMetadata type.
    pub fn new(
        path: &Path,
        parent: Option<Arc<Mutex<Folder>>>,
        pwd_index: i32
    ) -> Arc<Mutex<Self>> {
        let name = if path.parent().is_none() {
            // Root folder: use the full path string.
            path.to_str().unwrap_or("").to_string()
        } else {
            path.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("")
                .to_string()
        };
        let url = path.to_str().unwrap_or("").to_string();

        // Attempt to create metadata using the new CommonMetadata::new.
        // If it fails, create a default metadata instance with exists set to false.
        let metadata = match CommonMetadata::new(path) {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("Error retrieving metadata for {}: {:?}", name, e);
                CommonMetadata::default_instance(path, &name)
            }
        };

        // Create and return the Folder wrapped in Arc<Mutex>.
        let folder = Folder {
            name,
            url,
            children: Vec::new(),
            parent,
            metadata,
            index: pwd_index,
        };

        Arc::new(Mutex::new(folder))
    }

    pub fn modify_name(&mut self, new_name: String) -> io::Result<()> {
        let current_path = Path::new(&self.url);
        let parent_dir = current_path.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "No parent directory available")
        })?;
        let new_path = parent_dir.join(&new_name);
        fs::rename(current_path, &new_path)?;
        
        self.name = new_name.clone();
        self.url = new_path.to_string_lossy().into_owned();
        self.metadata.set_name(new_name);
        Ok(())
    }

    pub fn add_size(&mut self, size: u64) {
        let current_size = self.metadata.size();
        self.metadata.set_size(current_size + size);
    }

    pub fn subtract_size(&mut self, size: u64){
        let current_size = self.metadata.size();
        self.metadata.set_size(current_size - size);
    }

    pub fn add_child(&mut self, child: PathType) {
        // Increase the number of children.
        self.children.push(child.clone());
        self.metadata.set_children(self.metadata.children().unwrap_or(0) + 1);

        // Determine the size of the child from its metadata.
        let child_size = match child {
            PathType::File(ref file_arc) => {
                let file = file_arc.lock().unwrap();
                file.metadata.size()
            }
            PathType::Folder(ref folder_arc) => {
                let folder = folder_arc.lock().unwrap();
                folder.metadata.size()
            }
            PathType::None => 0,
        };

        // Increase folder's size by the child's size.
        self.add_size(child_size);
    }

    pub fn remove_child(&mut self, target: &PathType) -> Option<PathType> {
        // First, determine the URL from the target PathType.
        let target_url = match target {
            PathType::File(file_arc) => {
                let file = file_arc.lock().unwrap();
                file.metadata.path().to_string_lossy().into_owned()
            }
            PathType::Folder(folder_arc) => {
                let folder = folder_arc.lock().unwrap();
                folder.metadata.path().to_string_lossy().into_owned()
            }
            PathType::None => return None,
        };
    
        if let Some(pos) = self.children.iter().position(|child| {
            match child {
                PathType::File(file_arc) => {
                    let file = file_arc.lock().unwrap();
                    file.metadata.path().to_string_lossy() == target_url
                }
                PathType::Folder(folder_arc) => {
                    let folder = folder_arc.lock().unwrap();
                    folder.metadata.path().to_string_lossy() == target_url
                }
                PathType::None => false,
            }
        }) {
            let removed_child = self.children.remove(pos);
            // Update children count and size based on the removed child.
            self.metadata.set_children(self.metadata.children().unwrap_or(0).saturating_sub(1));
    
            let child_size = match &removed_child {
                PathType::File(file_arc) => {
                    let file = file_arc.lock().unwrap();
                    file.metadata.size()
                }
                PathType::Folder(folder_arc) => {
                    let folder = folder_arc.lock().unwrap();
                    folder.metadata.size()
                }
                PathType::None => 0,
            };
            self.subtract_size(child_size);
            Some(removed_child)
        } else {
            None
        }
    }

    pub fn get_formatted_metadata(&self) -> String {
        self.metadata.formatted_metadata()
    }


}