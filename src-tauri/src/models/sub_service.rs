use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize)]
pub struct SubService {
    id: u32,
    name: String,
    price: Option<Decimal>
}