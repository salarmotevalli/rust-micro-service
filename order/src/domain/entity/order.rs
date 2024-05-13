use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: ObjectId,
    pub address: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrder {
    pub address: String,
    pub price: i32,
}
