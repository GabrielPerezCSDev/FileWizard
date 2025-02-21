use std::path::Path;
use std::fs;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::time::SystemTime;
use crate::model::metadata::base::BaseMetadata;
//use super::base_file::File as BaseFile;
use super::base_file::BaseFileOps;
use crate::model::metadata::windows::WindowsMetadata;
use std::path::PathBuf;
use crate::model::file::File;
use crate::model::metadata::base::MetadataError;

#[derive(Debug, Clone)]
pub struct WindowsFile {
    file: File<WindowsMetadata>,
}

impl WindowsFile {
    pub fn new(path: &Path, parent: Option<PathBuf>) -> Self {
        // Create the metadata in the OS-specific implementation
        let metadata = match WindowsMetadata::new(path) {
            Ok(m) => m,
            Err(e) => {
                let name = path.file_name()
                    .and_then(|os_str| os_str.to_str())
                    .unwrap_or("")
                    .to_string();
                eprintln!("WindowsMetadata error for '{}': {:?}", name, e);
                WindowsMetadata::default_instance(path, &name)
            }
        };

        WindowsFile {
            file: File::new(parent, metadata),  // Pass the created metadata
        }
    }

    // Windows-specific methods
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
            if winapi::um::fileapi::SetFileAttributesW(
                path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
                new_attrs
            ) == 0 {
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
            if winapi::um::fileapi::SetFileAttributesW(
                path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
                new_attrs
            ) == 0 {
                return Err(io::Error::last_os_error());
            }
        }

        self.get_metadata_mut().set_system(system);
        Ok(())
    }

    // Windows-specific getters
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

}

impl BaseFileOps<WindowsMetadata> for WindowsFile {
    fn get_metadata(&self) -> &WindowsMetadata {
        self.file.get_metadata()
    }

    fn get_metadata_mut(&mut self) -> &mut WindowsMetadata {
        self.file.get_metadata_mut()
    }

    fn modify_name(&mut self, new_name: String) -> io::Result<()> {
        if self.is_read_only() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Cannot modify a read-only file"
            ));
        }
        self.file.modify_name(new_name)
    }

    fn modify_extension(&mut self, new_extension: String) -> io::Result<()> {
        if self.is_read_only() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Cannot modify a read-only file"
            ));
        }
        self.file.modify_extension(new_extension)
    }

    fn update_size(&mut self) -> io::Result<()> {
        self.file.update_size()
    }

    fn set_name(&mut self, name: String) {
        self.file.set_name(name)
    }

    fn set_path(&mut self, path: PathBuf) {
        self.file.set_path(path)
    }

    fn set_size(&mut self, size: u64) {
        self.file.set_size(size)
    }
}

impl BaseMetadata for WindowsFile {
    fn size(&self) -> u64 {
        self.file.get_metadata().size()
    }

    fn children(&self) -> Option<u64> {
        self.file.get_metadata().children()
    }

    fn created(&self) -> SystemTime {
        self.file.get_metadata().created()
    }

    fn modified(&self) -> SystemTime {
        self.file.get_metadata().modified()
    }

    fn is_directory(&self) -> bool {
        self.file.get_metadata().is_directory()
    }

    fn is_file(&self) -> bool {
        self.file.get_metadata().is_file()
    }

    fn name(&self) -> String {
        self.file.get_metadata().name()
    }

    fn path(&self) -> PathBuf {
        self.file.get_metadata().path()
    }

    fn extension(&self) -> String {
        self.file.get_metadata().extension()
    }

    fn exists(&self) -> bool {
        self.file.get_metadata().exists()
    }

    fn formatted_metadata(&self) -> String {
        self.file.get_metadata().formatted_metadata()
    }

    // Add the required setters
    fn set_size(&mut self, new_size: u64) {
        self.file.get_metadata_mut().set_size(new_size);
    }

    fn set_children(&mut self, new_children: u64) {
        self.file.get_metadata_mut().set_children(new_children);
    }

    fn set_modified(&mut self, new_modified: SystemTime) {
        self.file.get_metadata_mut().set_modified(new_modified);
    }

    fn set_exists(&mut self, exists: bool) {
        self.file.get_metadata_mut().set_exists(exists);
    }

    fn set_name(&mut self, new_name: String) {
        self.file.get_metadata_mut().set_name(new_name);
    }

    fn set_extension(&mut self, new_extension: String) {
        self.file.get_metadata_mut().set_extension(new_extension);
    }

    fn set_path(&mut self, new_path: PathBuf) {
        self.file.get_metadata_mut().set_path(new_path);
    }

    // Constructor implementation
    fn new(path: &Path) -> Result<Self, MetadataError> {
        let metadata = WindowsMetadata::new(path)?;
        Ok(WindowsFile {
            file: File::new(None, metadata)
        })
    }
}