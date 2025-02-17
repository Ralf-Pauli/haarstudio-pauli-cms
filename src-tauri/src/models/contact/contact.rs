use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::contact::address::Address;
use crate::models::contact::opening_hours::OpeningHours;
use crate::models::contact::social_media::SocialMedia;

#[derive(Serialize, Deserialize)]
pub struct Contact {
    address: Address,
    contact_information: HashMap<String, String>,
    opening_hours: Vec<OpeningHours>,
    pub social_media: Vec<SocialMedia>,
}