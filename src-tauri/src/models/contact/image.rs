use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub size: f64,
    pub url: String,
}