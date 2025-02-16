use std::path::Path;
use std::fs;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::time::SystemTime;
use crate::model::metadata::base::BaseMetadata;
use super::base_file::File as BaseFile;
use super::base_file::BaseFileOps;
use crate::model::folder::base_folder::Folder;
use std::sync::{Arc, Mutex};
use crate::model::metadata::windows::WindowsMetadata;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WindowsFile {
    base: BaseFile,
    metadata: WindowsMetadata,
}

impl WindowsFile {
    pub fn new(path: &Path, parent: Option<Arc<Mutex<Folder>>>) -> Self {
        let base = BaseFile::new(path, parent);
        
        // Create Windows-specific metadata
        let (metadata, _error) = match WindowsMetadata::new(path) {
            Ok(meta) => (meta, None),
            Err(e) => {
                let common = WindowsMetadata::default_instance(path, &base.metadata.name());
                (common, Some(format!("{:?}", e)))
            }
        };

        WindowsFile {
            base,
            metadata,
        }
    }

    pub fn set_read_only(&mut self, read_only: bool) -> io::Result<()> {
        let path = self.base.metadata.path();
        let metadata = fs::metadata(&path)?; // Pass reference to avoid moving
        let mut perms = metadata.permissions();
        perms.set_readonly(read_only);
        fs::set_permissions(&path, perms)?; // Use reference again
        self.metadata.set_read_only(read_only);
        Ok(())
    }

    pub fn set_hidden(&mut self, hidden: bool) -> io::Result<()> {
        let path = self.base.metadata.path();
        let attrs = self.metadata.raw_attributes();
        
        let new_attrs = if hidden {
            attrs | 0x2
        } else {
            attrs & !0x2
        };
        
        unsafe {
            if winapi::um::fileapi::SetFileAttributesW(
                path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
                new_attrs
            ) == 0 {
                return Err(io::Error::last_os_error());
            }
        }
        
        self.metadata.set_hidden(hidden);
        Ok(())
    }

    pub fn set_system(&mut self, system: bool) -> io::Result<()> {
        let path = self.base.metadata.path();
        let attrs = self.metadata.raw_attributes();
        
        let new_attrs = if system {
            attrs | 0x4
        } else {
            attrs & !0x4
        };
        
        unsafe {
            if winapi::um::fileapi::SetFileAttributesW(
                path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
                new_attrs
            ) == 0 {
                return Err(io::Error::last_os_error());
            }
        }
        
        self.metadata.set_system(system);
        Ok(())
    }

    pub fn get_formatted_metadata(&self) -> String {
        self.metadata.formatted_metadata()
    }

    // Getters
    pub fn get_name(&self) -> String {
        self.metadata.name()
    }

    pub fn get_path(&self) -> PathBuf {
        self.metadata.path()
    }

    pub fn get_extension(&self) -> String {
        self.metadata.extension()
    }

    pub fn has_error(&self) -> bool {
        self.base.error.is_some()
    }

    pub fn get_error(&self) -> Option<String> {
        self.base.error.clone()
    }

    // Windows-specific getters
    pub fn is_read_only(&self) -> bool {
        self.metadata.is_read_only()
    }

    pub fn is_hidden(&self) -> bool {
        self.metadata.is_hidden()
    }

    pub fn is_system(&self) -> bool {
        self.metadata.is_system()
    }

    pub fn is_archive(&self) -> bool {
        self.metadata.is_archive()
    }
}

impl BaseFileOps for WindowsFile {

    fn is_read_only(&self) -> bool {
        let path = self.base.metadata.path();
        // If retrieving metadata fails, assume not read-only.
        fs::metadata(path)
            .map(|metadata| metadata.permissions().readonly())
            .unwrap_or(false)
    }

    fn modify_name(&mut self, new_name: String) -> io::Result<()> {
        println!("\n\n\nCalled modify_name in windows_fille.rs");
        if self.is_read_only() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Cannot modify a read-only file",
            ));
        }
        self.base.modify_name(new_name.clone())?;
        // Now update the Windows-specific metadata as well:
        self.metadata.set_name(new_name);
        self.metadata.set_path(self.base.metadata.path().clone());
        Ok(())
    }

    fn modify_extension(&mut self, new_extension: String) -> io::Result<()> {
        if self.is_read_only() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Cannot modify a read-only file",
            ));
        }
        self.base.modify_extension(new_extension.clone())?;
        self.metadata.set_extension(new_extension);
        self.metadata.set_path(self.base.metadata.path().clone());
        Ok(())
    }

    fn update_size(&mut self) -> io::Result<()> {
        self.base.update_size()?;
        Ok(())
    }
}

impl BaseMetadata for WindowsFile {
    fn size(&self) -> u64 {
        self.metadata.size()
    }

    fn children(&self) -> Option<u64> {
        self.metadata.children()
    }

    fn created(&self) -> SystemTime {
        self.metadata.created()
    }

    fn modified(&self) -> SystemTime {
        self.metadata.modified()
    }

    fn is_directory(&self) -> bool {
        self.metadata.is_directory()
    }

    fn is_file(&self) -> bool {
        self.metadata.is_file()
    }

    fn name(&self) -> String {
        self.metadata.name()
    }

    fn path(&self) -> PathBuf {
        self.metadata.path()
    }

    fn extension(&self) -> String {
        self.metadata.extension()
    }

    fn exists(&self) -> bool {
        self.metadata.exists()
    }

    fn formatted_metadata(&self) -> String {
        self.metadata.formatted_metadata()
    }
}