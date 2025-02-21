// path_type.rs
use std::fmt;
use crate::model::file_system::path_alias::{FileType, FolderType};
use crate::model::metadata::base::BaseMetadata;
use crate::model::file::base_file::BaseFileOps;
use crate::model::folder::base_folder::BaseFolderOps;

#[derive(Clone, Debug)]
pub enum PathType {
    File(FileType),
    Folder(FolderType),
    None,
}

impl fmt::Display for PathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathType::File(file) => {
                // Because both WindowsFile and base File
                // implement the same trait(s) or have the same methods,
                // we can call e.g. file.name(), file.size(), etc.
                write!(
                    f,
                    "File: {} (size: {} bytes)",
                    file.get_metadata().name(),
                    file.get_metadata().size()
                )
            }
            PathType::Folder(folder) => {
                // Similarly, both WindowsFolder and base Folder
                // implement the same trait methods: name(), size(), children() ...
                write!(
                    f,
                    "Folder: {} (size: {} bytes, children: {})",
                    folder.get_metadata().name(),
                    folder.get_metadata().size(),
                    folder.get_metadata().children().unwrap_or(0)
                )
            }
            PathType::None => {
                write!(f, "None")
            }
        }
    }
}