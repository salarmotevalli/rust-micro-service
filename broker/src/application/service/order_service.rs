use std::sync::Arc;

use async_trait::async_trait;
use crate::application::repository::OrderRepository;

use crate::{
    domain::entity::order::{CreateOrder, Order},
    error::HandlerError,
};

#[async_trait]
pub trait OrderServiceInterface: Sync + Send {
    async fn get_order_by_id(&self, order_id: String) -> Result<Option<Order>, HandlerError>;
    async fn get_all_orders(&self) -> Result<Vec<Order>, HandlerError>;
    async fn create_order(&self, order: CreateOrder) -> Result<(), HandlerError>;
}

pub struct OrderService {
    pub order_repository: Arc<dyn OrderRepository>
}

#[async_trait]
impl OrderServiceInterface for OrderService {
    async fn get_order_by_id(&self, order_id: String) -> Result<Option<Order>, HandlerError> {
        let order = self.order_repository.get_by_id(order_id).await?;
        Ok(order)
    }

    async fn get_all_orders(&self) -> Result<Vec<Order>, HandlerError> {
        todo!()
    }

    async fn create_order(&self, order: CreateOrder) -> Result<(), HandlerError> {
        self.order_repository.create(order).await?;
        Ok(())
    }
}
