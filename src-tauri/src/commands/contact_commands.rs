use crate::models::contact::contact::Contact;
use crate::utils::file_error::FileError;
use crate::utils::file_handler::{load_json, save_json};
use crate::utils::image_handler::generate_image_formats;

const CONTACT_FILE_PATH: &str = "./data/contact.json";

#[tauri::command]
pub fn load_contact() -> Result<Contact, FileError> {
    load_json(CONTACT_FILE_PATH).map_err(|e| {
        eprintln!("Failed to load contact data from {}: {:?}", CONTACT_FILE_PATH, e);
        e
    })
}

#[tauri::command]
pub fn save_contact(mut contact: Contact) -> Result<(), FileError> {
    process_social_media_icons(&mut contact)?;
    save_json(CONTACT_FILE_PATH, &contact).map_err(|e| {
        eprintln!("Failed to save contact data to {}: {:?}", CONTACT_FILE_PATH, e);
        e
    })
}

fn process_social_media_icons(contact: &mut Contact) -> Result<(), FileError> {
    contact.social_media.iter_mut().try_for_each(|media| {
        generate_image_formats(&mut media.icon).map_err(|_| FileError::ProcessingError {
            message: format!("Failed to generate image formats for social media: {}", media.name),
        })
    })
}