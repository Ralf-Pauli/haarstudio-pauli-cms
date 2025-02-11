use serde::{Deserialize, Serialize};
use crate::models::contact::image::Image;

#[derive(Serialize, Deserialize)]
pub struct ImageVariants {
    pub thumbnail: Option<Image>,
    pub large: Option<Image>,
    pub medium: Option<Image>,
    pub small: Option<Image>,
}