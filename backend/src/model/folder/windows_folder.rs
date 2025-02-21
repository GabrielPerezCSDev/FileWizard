use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::collections::HashSet;
use std::os::windows::ffi::OsStrExt;
use winapi::um::fileapi::SetFileAttributesW;
use crate::model::metadata::windows::WindowsMetadata;
use crate::model::metadata::base::BaseMetadata;
use crate::model::file_system::path_type::PathType;
use std::time::SystemTime;
use crate::model::metadata::base::MetadataError;
use crate::model::folder::base_folder::{ Folder, BaseFolderOps };

#[derive(Clone, Debug)]
pub struct WindowsFolder {
    folder: Folder<WindowsMetadata>,
}

impl WindowsFolder {
    /// Creates a new WindowsFolder instance
    pub fn new(path: &Path, parent: Option<PathBuf>, index: i32) -> Self {
        let metadata = match WindowsMetadata::new(path) {
            Ok(m) => m,
            Err(e) => {
                let name = path
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                    .unwrap_or("")
                    .to_string();
                eprintln!("WindowsMetadata error for '{}': {:?}", name, e);
                WindowsMetadata::default_instance(path, &name)
            }
        };

        WindowsFolder {
            folder: Folder::new(parent, index, metadata),
        }
    }

    // ----------------------------------------------------------------
    // Windows-specific methods
    // ----------------------------------------------------------------

    pub fn set_read_only(&mut self, read_only: bool) -> io::Result<()> {
        let path = self.get_metadata().path();
        let metadata = fs::metadata(&path)?;
        let mut perms = metadata.permissions();
        perms.set_readonly(read_only);
        fs::set_permissions(&path, perms)?;

        self.get_metadata_mut().set_read_only(read_only);
        Ok(())
    }

    pub fn set_hidden(&mut self, hidden: bool) -> io::Result<()> {
        let path = self.get_metadata().path();
        let attrs = self.get_metadata().raw_attributes();

        let new_attrs = if hidden { attrs | 0x2 } else { attrs & !0x2 };

        unsafe {
            if
                SetFileAttributesW(
                    path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
                    new_attrs
                ) == 0
            {
                return Err(io::Error::last_os_error());
            }
        }

        self.get_metadata_mut().set_hidden(hidden);
        Ok(())
    }

    pub fn set_system(&mut self, system: bool) -> io::Result<()> {
        let path = self.get_metadata().path();
        let attrs = self.get_metadata().raw_attributes();

        let new_attrs = if system { attrs | 0x4 } else { attrs & !0x4 };

        unsafe {
            if
                SetFileAttributesW(
                    path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
                    new_attrs
                ) == 0
            {
                return Err(io::Error::last_os_error());
            }
        }

        self.get_metadata_mut().set_system(system);
        Ok(())
    }

    // Attribute getters
    pub fn is_read_only(&self) -> bool {
        self.get_metadata().is_read_only()
    }

    pub fn is_hidden(&self) -> bool {
        self.get_metadata().is_hidden()
    }

    pub fn is_system(&self) -> bool {
        self.get_metadata().is_system()
    }

    pub fn is_archive(&self) -> bool {
        self.get_metadata().is_archive()
    }

    // Delegated getters
    pub fn get_children(&self) -> &HashSet<PathBuf> {
        self.folder.get_children()
    }

    pub fn get_parent(&self) -> Option<&PathBuf> {
        self.folder.get_parent()
    }

    pub fn get_index(&self) -> i32 {
        self.folder.get_index()
    }
}

impl BaseFolderOps<WindowsMetadata> for WindowsFolder {
    fn modify_name(&mut self, new_name: String) -> io::Result<()> {
        if self.is_read_only() {
            return Err(
                io::Error::new(io::ErrorKind::PermissionDenied, "Cannot rename a read-only folder")
            );
        }
        self.folder.modify_name(new_name)
    }

    fn add_child(&mut self, child: PathType) -> io::Result<()> {
        if self.is_read_only() {
            return Err(
                io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "Cannot add child to a read-only folder"
                )
            );
        }
        self.folder.add_child(child)
    }

    fn remove_child(&mut self, child: PathType) -> Result<PathBuf, io::Error> {
        if self.is_read_only() {
            return Err(
                io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "Cannot remove child from a read-only folder"
                )
            );
        }
        self.folder.remove_child(child)
    }

    fn set_name(&mut self, name: String) {
        self.folder.set_name(name)
    }

    fn set_path(&mut self, path: PathBuf) {
        self.folder.set_path(path)
    }

    fn set_size(&mut self, size: u64) {
        self.folder.set_size(size)
    }

    fn set_children(&mut self, count: u64) {
        self.folder.set_children(count)
    }
    
    fn get_formatted_metadata(&self) -> String {
        self.folder.get_metadata().formatted_metadata()
    }

    fn get_metadata(&self) -> &WindowsMetadata {
        self.folder.get_metadata()
    }

    fn get_metadata_mut(&mut self) -> &mut WindowsMetadata {
        self.folder.get_metadata_mut()
    }

    fn update_size(&mut self, delta: i64) {
        self.folder.update_size(delta)
    }
}

impl BaseMetadata for WindowsFolder {
    fn size(&self) -> u64 {
        self.folder.get_metadata().size()
    }
    fn children(&self) -> Option<u64> {
        self.folder.get_metadata().children()
    }
    fn created(&self) -> SystemTime {
        self.folder.get_metadata().created()
    }
    fn modified(&self) -> SystemTime {
        self.folder.get_metadata().modified()
    }
    fn is_directory(&self) -> bool {
        self.folder.get_metadata().is_directory()
    }
    fn is_file(&self) -> bool {
        self.folder.get_metadata().is_file()
    }
    fn name(&self) -> String {
        self.folder.get_metadata().name()
    }
    fn path(&self) -> PathBuf {
        self.folder.get_metadata().path()
    }
    fn extension(&self) -> String {
        self.folder.get_metadata().extension()
    }
    fn exists(&self) -> bool {
        self.folder.get_metadata().exists()
    }
    fn formatted_metadata(&self) -> String {
        self.folder.get_metadata().formatted_metadata()
    }

    fn set_size(&mut self, new_size: u64) {
        self.folder.get_metadata_mut().set_size(new_size);
    }

    fn set_children(&mut self, new_children: u64) {
        self.folder.get_metadata_mut().set_children(new_children);
    }

    fn set_modified(&mut self, new_modified: SystemTime) {
        self.folder.get_metadata_mut().set_modified(new_modified);
    }

    fn set_exists(&mut self, exists: bool) {
        self.folder.get_metadata_mut().set_exists(exists);
    }

    fn set_name(&mut self, new_name: String) {
        self.folder.get_metadata_mut().set_name(new_name);
    }

    fn set_extension(&mut self, new_extension: String) {
        self.folder.get_metadata_mut().set_extension(new_extension);
    }

    fn set_path(&mut self, new_path: PathBuf) {
        self.folder.get_metadata_mut().set_path(new_path);
    }

    // Add constructor
    fn new(path: &Path) -> Result<Self, MetadataError> {
        let metadata = WindowsMetadata::new(path)?;
        Ok(WindowsFolder {
            folder: Folder::new(None, 0, metadata)
        })
    }
}
