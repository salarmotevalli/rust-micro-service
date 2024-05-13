use self::appproto::CreateOrder;
use crate::application::dto::order_dto::{CreateOrderDTO, OrderDTO};
use crate::error::HandlerError;
use appproto::order_service_client::OrderServiceClient;
use appproto::{CreateOrderRequest, GetOrderRequest};
use async_trait::async_trait;
pub mod appproto {
    tonic::include_proto!("app");
}

pub struct OrderRepository;

#[async_trait]
impl crate::application::repository::OrderRepository for OrderRepository {
    async fn get_by_id(&self, id: String) -> Result<Option<OrderDTO>, HandlerError> {
        let mut client = OrderServiceClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(GetOrderRequest { id });

        let response = client.get_order(request).await?;
        let responsed_order = response.into_inner().order;

        if let Some(o) = responsed_order {
            return Ok(Some(OrderDTO {
                id: o.id,
                address: o.address,
                price: o.price,
            }));
        }

        Ok(None)
    }

    async fn create(&self, order: CreateOrderDTO) -> Result<(), HandlerError> {
        let mut client = OrderServiceClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(CreateOrderRequest {
            create_order: Some(CreateOrder {
                address: order.address,
                price: order.price,
            }),
        });

        client.create_order(request).await?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<OrderDTO>, HandlerError> {
        todo!()
    }
}
