use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub address: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrder {
    pub address: String,
    pub price: i32,
}
