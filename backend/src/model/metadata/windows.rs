use std::time::SystemTime;
use std::path::Path;
use std::fs;
use std::os::windows::fs::MetadataExt; // Windows-specific metadata traits
use std::path::PathBuf;

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

    pub fn default_instance(path: &Path, name: &str) -> Self {
        Self {
            common: CommonMetadata::default_instance(path, name),
            read_only: false,
            hidden: false,
            system: false,
            archive: false,
            raw_attributes: 0,
        }
    }
}

impl BaseMetadata for WindowsMetadata {
    fn new(path: &Path) -> Result<Self, MetadataError> {
        let common = CommonMetadata::new(path)?;
        let metadata = fs::metadata(path)?;

        let attributes = metadata.file_attributes();

        Ok(Self {
            common,
            read_only: (attributes & 0x1) != 0,
            hidden: (attributes & 0x2) != 0,
            system: (attributes & 0x4) != 0,
            archive: (attributes & 0x20) != 0,
            raw_attributes: attributes,
        })
    }

    // Base getters - delegate to common
    fn size(&self) -> u64 { self.common.size() }
    fn children(&self) -> Option<u64> { self.common.children() }
    fn created(&self) -> SystemTime { self.common.created() }
    fn modified(&self) -> SystemTime { self.common.modified() }
    fn is_directory(&self) -> bool { self.common.is_directory() }
    fn is_file(&self) -> bool { self.common.is_file() }
    fn name(&self) -> String { self.common.name() }
    fn path(&self) -> PathBuf { self.common.path() }
    fn extension(&self) -> String { self.common.extension() }
    fn exists(&self) -> bool { self.common.exists() }

    // Base setters - delegate to common
    fn set_size(&mut self, new_size: u64) { self.common.set_size(new_size) }
    fn set_children(&mut self, new_children: u64) { self.common.set_children(new_children) }
    fn set_modified(&mut self, new_modified: SystemTime) { self.common.set_modified(new_modified) }
    fn set_exists(&mut self, exists: bool) { self.common.set_exists(exists) }
    fn set_name(&mut self, new_name: String) { self.common.set_name(new_name) }
    fn set_extension(&mut self, new_extension: String) { self.common.set_extension(new_extension) }
    fn set_path(&mut self, new_path: PathBuf) { self.common.set_path(new_path) }

    fn formatted_metadata(&self) -> String {
        let children_str = if self.is_directory() {
            self.children().unwrap_or(0).to_string()
        } else {
            "N/A".to_string()
        };

        format!(
            "File Metadata:
            Name: {}
            Extension: {}
            Path: {}
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
            self.path().display(),
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
