use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpeningHours {
    begin: Option<String>,
    end: Option<String>,
    closed: bool,
    days: String,
}
