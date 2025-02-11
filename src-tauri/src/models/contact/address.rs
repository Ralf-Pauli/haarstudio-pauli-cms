use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Address {
    city: String,
    zip_code: String,
    street: String,
    house_number: String,
}