use crate::{
    application::dto::order_dto::{CreateOrderDTO, OrderDTO},
    error::HandlerError,
};
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepository: Sync + Send {
    async fn get_by_id(&self, id: String) -> Result<Option<OrderDTO>, HandlerError>;
    async fn create(&self, order: CreateOrderDTO) -> Result<(), HandlerError>;
    async fn get_all(&self) -> Result<Vec<OrderDTO>, HandlerError>;
}
