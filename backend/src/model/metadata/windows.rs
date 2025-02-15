use std::time::SystemTime;
use std::path::Path;
use std::fs;
use std::os::windows::fs::MetadataExt; // Windows-specific metadata traits

use super::base::{ BaseMetadata, CommonMetadata, MetadataError };

/// Windows-specific metadata implementation
#[derive(Debug, Clone)]
pub struct WindowsMetadata {
    common: CommonMetadata,
    read_only: bool,
    hidden: bool,
    system: bool,
    archive: bool,
    raw_attributes: u32,
}

impl WindowsMetadata {
    pub fn new(path: &Path) -> Result<Self, MetadataError> {
        let common = CommonMetadata::new(path)?;
        let metadata = fs::metadata(path)?;

        // Get Windows-specific file attributes
        let attributes = metadata.file_attributes();

        Ok(Self {
            common,
            read_only: (attributes & 0x1) != 0, // FILE_ATTRIBUTE_READONLY
            hidden: (attributes & 0x2) != 0, // FILE_ATTRIBUTE_HIDDEN
            system: (attributes & 0x4) != 0, // FILE_ATTRIBUTE_SYSTEM
            archive: (attributes & 0x20) != 0, // FILE_ATTRIBUTE_ARCHIVE
            raw_attributes: attributes,
        })
    }

    // Windows-specific getters
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn is_system(&self) -> bool {
        self.system
    }

    pub fn is_archive(&self) -> bool {
        self.archive
    }

    pub fn raw_attributes(&self) -> u32 {
        self.raw_attributes
    }

    // Windows-specific setters
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
        if read_only {
            self.raw_attributes |= 0x1;
        } else {
            self.raw_attributes &= !0x1;
        }
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
        if hidden {
            self.raw_attributes |= 0x2;
        } else {
            self.raw_attributes &= !0x2;
        }
    }

    pub fn set_system(&mut self, system: bool) {
        self.system = system;
        if system {
            self.raw_attributes |= 0x4;
        } else {
            self.raw_attributes &= !0x4;
        }
    }

    pub fn set_archive(&mut self, archive: bool) {
        self.archive = archive;
        if archive {
            self.raw_attributes |= 0x20;
        } else {
            self.raw_attributes &= !0x20;
        }
    }

    pub fn set_raw_attributes(&mut self, attributes: u32) {
        self.raw_attributes = attributes;
        self.read_only = (attributes & 0x1) != 0;
        self.hidden = (attributes & 0x2) != 0;
        self.system = (attributes & 0x4) != 0;
        self.archive = (attributes & 0x20) != 0;
    }

    // Delegated setters for common metadata fields
    pub fn set_name(&mut self, new_name: String) {
        self.common.set_name(new_name);
    }

    pub fn set_extension(&mut self, new_extension: String) {
        self.common.set_extension(new_extension);
    }

    pub fn set_size(&mut self, new_size: u64) {
        self.common.set_size(new_size);
    }

    pub fn set_children(&mut self, new_children: u64) {
        self.common.set_children(new_children);
    }

    pub fn set_modified(&mut self, new_modified: SystemTime) {
        self.common.set_modified(new_modified);
    }

    pub fn set_exists(&mut self, exists: bool) {
        self.common.set_exists(exists);
    }


    pub fn formatted_metadata(&self) -> String {

        let children_str = if self.is_directory() {
            self.common.children().unwrap_or(0).to_string()
        } else {
            "N/A".to_string()
        };

        format!(
            "File Metadata:
            Name: {}
            Extension: {}
            Size: {} bytes
            Children: {}
            Created: {:?}
            Modified: {:?}
            Directory: {}
            File: {}
            Exists: {}
            Read-Only: {}
            Hidden: {}
            System: {}
            Archive: {}
            Raw Attributes: {:#010X}",
            self.name(),
            self.extension(),
            self.size(),
            children_str,
            self.created(),
            self.modified(),
            self.is_directory(),
            self.is_file(),
            self.exists(),
            self.is_read_only(),
            self.is_hidden(),
            self.is_system(),
            self.is_archive(),
            self.raw_attributes()
        )
    }
}

// Implement BaseMetadata by delegating to common
impl BaseMetadata for WindowsMetadata {
    fn size(&self) -> u64 {
        self.common.size()
    }

    fn children(&self) -> Option<u64> {
        self.common.children()
    }

    fn created(&self) -> SystemTime {
        self.common.created()
    }

    fn modified(&self) -> SystemTime {
        self.common.modified()
    }

    fn is_directory(&self) -> bool {
        self.common.is_directory()
    }

    fn is_file(&self) -> bool {
        self.common.is_file()
    }

    fn name(&self) -> String {
        self.common.name()
    }

    fn extension(&self) -> String {
        self.common.extension()
    }

    fn exists(&self) -> bool {
        self.common.exists()
    }
}
