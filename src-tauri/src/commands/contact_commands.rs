use crate::models::contact::contact::Contact;
use crate::utils::file_error::FileError;
use crate::utils::file_handler::{load_json, save_json};

#[tauri::command]
pub fn load_contact() -> Result<Contact, FileError> {
    load_json("./data/contact.json")
}

#[tauri::command]
pub fn save_contact(contact: Contact) -> Result<(), FileError> {


    save_json("./data/contact.json", &contact)
}