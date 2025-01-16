use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::models::category::Category;

const SERVICES_FILE_PATH: &str = "./data/services.json";
const BACKUP_FILE_SUFFIX: &str = "bak";

/// Custom error type for better error handling.
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("File operation error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

/// Implement `Serialize` for `ServiceError` to ensure compatibility with Tauri commands.
impl Serialize for ServiceError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the error as a string using its `Display` implementation (`to_string`).
        serializer.serialize_str(&self.to_string())
    }
}

/// Wrapper struct for handling categories JSON serialization internally.
#[derive(Serialize, Deserialize)]
struct CategoriesWrapper {
    categories: Vec<Category>,
}

/// Create a backup of the file if it exists.
/// The backup file will have the `.bak` suffix.
fn create_backup(file_path: &Path) -> Result<(), ServiceError> {
    if file_path.exists() {
        let backup_path = file_path.with_extension(BACKUP_FILE_SUFFIX);
        fs::copy(file_path, &backup_path)?;
        println!("Backup created: {}", backup_path.display());
    }
    Ok(())
}

/// Read the contents of a file into a string.
fn read_file_to_string(file_path: &Path) -> Result<String, ServiceError> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

/// Load categories directly as a `Vec<Category>`, abstracting away the internal wrapper logic.
#[tauri::command]
pub fn load_categories() -> Result<Vec<Category>, ServiceError> {
    let file_path = PathBuf::from(SERVICES_FILE_PATH);

    // Ensure a backup is created before reading the file.
    create_backup(&file_path)?;

    // Read the file's content.
    let data = read_file_to_string(&file_path)?;

    // Deserialize the content into `CategoriesWrapper` and return the inner categories.
    let wrapper: CategoriesWrapper = serde_json::from_str(&data)?;
    Ok(wrapper.categories)
}

/// Save categories directly from a `Vec<Category>`.
#[tauri::command]
pub fn save_categories(categories: Vec<Category>) -> Result<(), ServiceError> {
    let file_path = PathBuf::from(SERVICES_FILE_PATH);

    // Wrap the categories in the `CategoriesWrapper` struct internally.
    let wrapper = CategoriesWrapper { categories };

    // Serialize the wrapper to a JSON string.
    let json_content = serde_json::to_string_pretty(&wrapper)?;

    // Write the serialized JSON to the file.
    let mut file = File::create(&file_path)?;
    file.write_all(json_content.as_bytes())?;
    Ok(())
}