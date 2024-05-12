use crate::{error::HandlerError, ordergrpc::{*, CreateOrder}};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct Service {}

impl From<HandlerError> for Status {
    fn from(value: HandlerError) -> Self {
        Status::new(500.into(), value.kind().to_string())
    }
}

impl From<crate::domain::entity::order::Order> for Order {
    fn from(value: crate::domain::entity::order::Order) -> Self {
        Order {
            id: value._id.to_string(),
            address: value.address,
            price: value.price,
        }
    }
}

impl From<CreateOrder> for crate::domain::entity::order::CreateOrder {
    fn from(value: CreateOrder) -> Self {
        crate::domain::entity::order::CreateOrder {
            address: value.address,
            price: value.price,
        }
    }
}


#[tonic::async_trait]
impl order_service_server::OrderService for Service {
    async fn get_order(
        &self,
        request: Request<GetOrderRequest>,
    ) -> Result<Response<GetOrderResponse>, Status> {
        let service = crate::container::Container::new()
            .await
            .order_service
            .clone();

        let order = service.get_order_by_id(request.into_inner().id).await?;

        if let Some(o) = order {
            return Ok(Response::new(GetOrderResponse {
                order: Some(o.into()),
            }));
        }

        Ok(Response::new(GetOrderResponse { order: None }))
    }

    async fn create_order(
        &self,
        request: Request<CreateOrderRequest>,
    ) -> Result<Response<Empty>, Status> {
        let service = crate::container::Container::new()
            .await
            .order_service
            .clone();

        let create_order =request.into_inner().create_order;

        if let Some(co) = create_order {
            service.create_order(co.into()).await?;
        } else {
            return Err(Status::new(500.into(), "no data provided"));
        }

        let reply = Empty {};

        Ok(Response::new(reply))
    }
}
