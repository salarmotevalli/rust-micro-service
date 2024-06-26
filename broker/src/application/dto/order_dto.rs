use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateOrderDTO {
    pub address: String,
    pub price: i32,
}

#[derive(Debug, Serialize, Default)]
pub struct OrderDTO {
    pub id: String,
    pub address: String,
    pub price: i32,
}

//
// impl From<Order> for OrderDTO {
//     fn from(val: Order) -> Self {
//         OrderDTO {
//             id: val.id.to_string(),
//             address: val.address,
//             price: val.price,
//         }
//     }
// }
//
// impl From<CreateOrderDTO> for CreateOrder {
//     fn from(val: CreateOrderDTO) -> Self {
//         CreateOrder {
//             address: val.address,
//             price: val.price,
//         }
//     }
// }
//
// impl From<CreateOrder> for CreateOrderDTO {
//     fn from(val: CreateOrder) -> Self {
//         CreateOrderDTO {
//             address: val.address,
//             price: val.price,
//         }
//     }
// }
