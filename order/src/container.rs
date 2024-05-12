use crate::application::repository::OrderRepository;
use crate::application::service::{OrderService, OrderServiceInterface};
use crate::infra::mongo;
use std::sync::Arc;

pub struct Container {
    pub order_service: Arc<dyn OrderServiceInterface>,
}

impl Container {
    pub async fn new() -> Self {
        let order_repository: Arc<dyn OrderRepository> = Arc::new(
            mongo::repository::order::Repository::new(Arc::new(mongo::client().await)),
        );

        let order_service = Arc::new(OrderService {
            repository: order_repository,
        });

        Self { order_service }
    }
}
