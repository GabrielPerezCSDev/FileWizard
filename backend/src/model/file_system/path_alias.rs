#[cfg(windows)]
pub type FileType = crate::model::file::windows_file::WindowsFile;

#[cfg(not(windows))]
pub type FileType = crate::model::file::base_file::File;


// same pattern for Folders:
#[cfg(windows)]
pub type FolderType = crate::model::folder::windows_folder::WindowsFolder;

#[cfg(not(windows))]
pub type FolderType = crate::model::folder::base_folder::Folder;
