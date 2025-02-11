use serde::{Deserialize, Serialize};
use crate::models::service::category::Category;
use crate::utils::file_error::FileError;
use crate::utils::file_handler::{load_json, save_json};


#[derive(Serialize, Deserialize)]
struct CategoriesWrapper {
    categories: Vec<Category>,
}

#[tauri::command]
pub fn load_categories() -> Result<Vec<Category>, FileError> {
    let wrapper: CategoriesWrapper = load_json("./data/services.json")?;
    Ok(wrapper.categories)
}

#[tauri::command]
pub fn save_categories(categories: Vec<Category>) -> Result<(), FileError> {
    let wrapper = CategoriesWrapper { categories };
    save_json("./data/services.json", &wrapper)
}