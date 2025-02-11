use rust_decimal::Decimal;
use super::sub_service::SubService;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Service {
    pub id: u32,
    pub name: String,
    pub price: Option<Decimal>,
    pub sub_services: Vec<SubService>,
}
