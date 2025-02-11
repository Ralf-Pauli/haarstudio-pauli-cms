use serde::{Deserialize, Serialize};
use crate::models::contact::image_variants::ImageVariants;

#[derive(Serialize, Deserialize)]
pub struct ImageDetails {
    pub name: String,
    pub alternative_text: String,
    pub width: usize,
    pub height: usize,
    pub formats: Option<ImageVariants>,
    pub size: f64,
    pub url: String,
}
