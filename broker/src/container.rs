use std::sync::Arc;

use crate::{
    application::service::{OrderService, OrderServiceInterface},
    infra::order_repository::OrderRepository,
};

pub struct Container {
    pub order_service: Arc<dyn OrderServiceInterface>,
}

impl Container {
    pub async fn new() -> Self {
        let order_service = OrderService {
            order_repository: Arc::new(OrderRepository),
        };

        Self {
            order_service: Arc::new(order_service),
        }
    }
}
