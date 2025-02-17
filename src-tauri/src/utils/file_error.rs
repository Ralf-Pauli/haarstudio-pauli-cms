use std::fmt::write;
use std::io; // Already public since it's from the standard library
use serde_json;
use tauri::ipc::InvokeError;

#[derive(Debug)]
pub enum FileError {  // Now public
    FileNotFound(String),
    IOError(io::Error),
    SerdeError(serde_json::Error),
    Other(String),
    ProcessingError {
        message: String,
    },
    ValidationError {
        message: String,
    },
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            FileError::IOError(err) => write!(f, "IO error: {}", err),
            FileError::SerdeError(err) => write!(f, "Serde JSON error: {}", err),
            FileError::Other(msg) => write!(f, "Other error: {}", msg),
            FileError::ProcessingError { message, .. } => {
                write!(f, "Processing error: {}", message)
            }
            FileError::ValidationError { message } => {
                write!(f, "Validation error: {}", message)
            }
        }
    }
}

impl std::error::Error for FileError {}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::IOError(err)
    }
}

impl From<serde_json::Error> for FileError {
    fn from(err: serde_json::Error) -> Self {
        FileError::SerdeError(err)
    }
}

impl Into<InvokeError> for FileError {
    fn into(self) -> InvokeError {
        InvokeError::from(self.to_string())
    }
}
