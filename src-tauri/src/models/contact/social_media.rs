use serde::{Deserialize, Serialize};
use crate::models::contact::image_details::ImageDetails;

#[derive(Serialize, Deserialize)]
pub struct SocialMedia {
    name: String,
    url: String,
    icon: ImageDetails,
}