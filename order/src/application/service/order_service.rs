use async_trait::async_trait;

use crate::{
    application::repository::OrderRepository,
    domain::entity::order::{CreateOrder, Order},
    error::HandlerError,
};
use std::sync::Arc;

#[async_trait]
pub trait OrderServiceInterface: Sync + Send {
    async fn get_order_by_id(&self, order_id: String) -> Result<Option<Order>, HandlerError>;
    async fn get_all_orders(&self) -> Result<Vec<Order>, HandlerError>;
    async fn create_order(&self, order: CreateOrder) -> Result<(), HandlerError>;
}

pub struct OrderService {
    pub repository: Arc<dyn OrderRepository>,
}

#[async_trait]
impl OrderServiceInterface for OrderService {
    async fn get_order_by_id(&self, order_id: String) -> Result<Option<Order>, HandlerError> {
        self.repository.get_by_id(order_id).await
    }

    async fn get_all_orders(&self) -> Result<Vec<Order>, HandlerError> {
        self.repository.get_all().await
    }

    async fn create_order(&self, order: CreateOrder) -> Result<(), HandlerError> {
        self.repository.create(order).await
    }
}
