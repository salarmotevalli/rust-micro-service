use appproto::order_service_client::OrderServiceClient;
use appproto::{GetOrderRequest, CreateOrderRequest};
use tonic::Status;
use async_trait::async_trait;
use crate::domain::entity::order::{Order, CreateOrder};
use crate::error::HandlerError;
use self::appproto::CreateOrder as CreateOrderGrpc;
pub mod appproto{
    tonic::include_proto!("app");
}

pub struct OrderRepository;

impl From<HandlerError> for Status {
    fn from(value: HandlerError) -> Self {
        Status::new(500.into(), value.kind().to_string())
    }
}
impl From<Status> for HandlerError {
    fn from(value: Status) -> Self {
        HandlerError::internal(Some(value.to_string()))
    }
}

impl From<tonic::transport::Error> for HandlerError
{
    fn from(value: tonic::transport::Error) -> Self {
        HandlerError::internal(Some(value.to_string()))
    }
}

#[async_trait]
impl crate::application::repository::OrderRepository for OrderRepository
{
    async fn get_by_id(&self, id: String) -> Result<Option<Order>, HandlerError> {
        let mut client = OrderServiceClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(GetOrderRequest {
            id
        });

        let response = client.get_order(request).await?;
        let responsed_order = response.into_inner().order; 

        if let Some(o) = responsed_order {
            return Ok(Some(Order{
                id: o.id,
                address: o.address,
                price: o.price
            }))
        }

        Ok(None)    
    }

    async fn create(&self, order: CreateOrder) -> Result<(), HandlerError> {
        
    let mut client = OrderServiceClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(CreateOrderRequest {
            create_order: Some(CreateOrderGrpc {
                address: order.address,
                price: order.price,
            })
        });

        client.create_order(request).await?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Order>, HandlerError> {
        todo!()
    }
}
