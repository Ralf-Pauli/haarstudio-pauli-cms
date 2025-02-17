use serde::{Deserialize, Serialize};
use crate::models::contact::image_details::ImageDetails;

#[derive(Serialize, Deserialize)]
pub struct SocialMedia {
    pub(crate) name: String,
    url: String,
    pub icon: ImageDetails,
}