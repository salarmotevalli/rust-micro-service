use async_trait::async_trait;

use crate::{
    domain::entity::order::{CreateOrder, Order},
    error::HandlerError,
};

#[async_trait]
pub trait OrderRepository: Sync + Send {
    async fn get_by_id(&self, id: String) -> Result<Option<Order>, HandlerError>;
    async fn create(&self, order: CreateOrder) -> Result<(), HandlerError>;
    async fn get_all(&self) -> Result<Vec<Order>, HandlerError>;
}
