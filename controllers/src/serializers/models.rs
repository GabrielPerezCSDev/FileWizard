use file_wizard_backend::model::folder::Folder;
use file_wizard_backend::model::file::File;
use file_wizard_backend::model::path_type::PathType;
use serde::{Serialize, Serializer};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct SerializableFolder {
    pub name: String,
    pub url: String,
    pub children: Vec<SerializablePathType>,
    pub metadata: HashMap<String, String>,
    pub index: i32,
    pub num_children: i32,
}

impl From<&Folder> for SerializableFolder {
    fn from(folder: &Folder) -> Self {
        SerializableFolder {
            name: folder.name.clone(),
            url: folder.url.clone(),
            children: folder
                .children
                .iter()
                .map(SerializablePathType::from)
                .collect(),
            metadata: folder.metadata.clone(),
            index: folder.index,
            num_children: folder.num_children,
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SerializablePathType {
    File(SerializableFile),
    Folder(SerializableFolder),
}

impl From<&PathType> for SerializablePathType {
    fn from(path_type: &PathType) -> Self {
        match path_type {
            PathType::File(file) => {
                let file = file.lock().unwrap();
                SerializablePathType::File(SerializableFile::from(&*file))
            }
            PathType::Folder(folder) => {
                let folder = folder.lock().unwrap();
                SerializablePathType::Folder(SerializableFolder::from(&*folder))
            }
            PathType::None => {
                // Handle the None case. You can decide how to represent this in JSON.
                // For example, you might use a default "empty" structure or a special type.
                panic!("[Error] PathType::None encountered, which is not serializable.");
            }
        }
    }
}

#[derive(Serialize)]
pub struct SerializableFile {
    pub name: String,
    pub url: String,
    pub metadata: HashMap<String, String>,
}

impl From<&File> for SerializableFile {
    fn from(file: &File) -> Self {
        SerializableFile {
            name: file.name.clone(),
            url: file.url.clone(),
            metadata: file.metadata.clone(),
        }
    }
}