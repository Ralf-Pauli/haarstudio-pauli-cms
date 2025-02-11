use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::utils::file_error::FileError;

const BACKUP_FILE_SUFFIX: &str = "bak";
/// Generic function to create a backup of the file if it exists.
fn create_backup(file_path: &Path) -> Result<(), FileError> {
    if file_path.exists() {
        let backup_path = file_path.with_extension(BACKUP_FILE_SUFFIX);
        fs::copy(file_path, &backup_path)?;
        println!("Backup created: {}", backup_path.display());
    }
    Ok(())
}

/// Generic function to read the contents of a file into a string.
fn read_file_to_string(file_path: &Path) -> Result<String, FileError> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

/// Generic function to load data from a JSON file.
/// The data type must implement `Deserialize`.
pub fn load_json<T>(file_path: &str) -> Result<T, FileError>
where
    T: DeserializeOwned,
{
    let path = PathBuf::from(file_path);

    // Ensure a backup is created before reading the file.
    create_backup(&path)?;

    // Read the file's content.
    let data = read_file_to_string(&path)?;

    // Deserialize the content into the specified type.
    let result: T = serde_json::from_str(&data)?;

    Ok(result)
}

/// Generic function to save data to a JSON file.
/// The data type must implement `Serialize`.
pub fn save_json<T>(file_path: &str, data: &T) -> Result<(), FileError>
where
    T: Serialize,
{
    let path = PathBuf::from(file_path);
    // Serialize the data to a JSON string.
    let json_content = serde_json::to_string_pretty(data)?;

    // Write the serialized JSON to the file.
    let mut file = File::create(&path)?;
    file.write_all(json_content.as_bytes())?;
    Ok(())
}