// src/model/validation/mod.rs
use std::io;

#[derive(Debug)]
pub enum FileNameError {
    InvalidCharacters(String),
    ReservedName(String),
    TooLong(usize),
    Empty,
    // Add more specific error types as needed
}

impl From<FileNameError> for io::Error {
    fn from(error: FileNameError) -> Self {
        match error {
            FileNameError::InvalidCharacters(msg) => {
                io::Error::new(io::ErrorKind::InvalidInput, msg)
            }
            FileNameError::ReservedName(msg) => {
                io::Error::new(io::ErrorKind::InvalidInput, msg)
            }
            FileNameError::TooLong(len) => {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("File name too long: {} characters (max 255)", len)
                )
            }
            FileNameError::Empty => {
                io::Error::new(io::ErrorKind::InvalidInput, "File name cannot be empty")
            }
        }
    }
}

pub fn validate_path_name(name: &str) -> Result<(), FileNameError> {
    #[cfg(windows)]
    {
        const INVALID_CHARS: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        const RESERVED_NAMES: &[&str] = &[
            "CON", "PRN", "AUX", "NUL",
            "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
            "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
        ];

        if name.is_empty() {
            return Err(FileNameError::Empty);
        }

        if name.len() > 255 {
            return Err(FileNameError::TooLong(name.len()));
        }

        if name.ends_with(' ') || name.ends_with('.') {
            return Err(FileNameError::InvalidCharacters(
                "Path name cannot end with a space or period".to_string()
            ));
        }

        if name.starts_with(' ') {
            return Err(FileNameError::InvalidCharacters(
                "Path name cannot start with a space".to_string()
            ));
        }

        if let Some(invalid_char) = name.chars().find(|c| INVALID_CHARS.contains(c)) {
            return Err(FileNameError::InvalidCharacters(
                format!("Character '{}' is not allowed in Windows path names", invalid_char)
            ));
        }

        let name_upper = name.to_uppercase();
        for reserved in RESERVED_NAMES {
            if name_upper == *reserved || name_upper.starts_with(&format!("{}.", reserved)) {
                return Err(FileNameError::ReservedName(
                    format!("'{}' is a reserved name in Windows", name)
                ));
            }
        }
    }

    #[cfg(unix)]
    {
        const INVALID_CHARS: &[char] = &['/']; 

        if name.is_empty() {
            return Err(FileNameError::Empty);
        }

        if name.len() > 255 {
            return Err(FileNameError::TooLong(name.len()));
        }

        if let Some(invalid_char) = name.chars().find(|c| INVALID_CHARS.contains(c)) {
            return Err(FileNameError::InvalidCharacters(
                format!("Character '{}' is not allowed in Unix path names", invalid_char)
            ));
        }
    }

    Ok(())
}

