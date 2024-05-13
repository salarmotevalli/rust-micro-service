use crate::application::dto::order_dto::CreateOrderDTO;
use crate::{application::service::OrderServiceInterface, error::HandlerError};
use actix_web::{get, post, web, HttpResponse};

#[post("/")]
pub(super) async fn create(
    service: web::Data<dyn OrderServiceInterface>,
    post_data: web::Json<CreateOrderDTO>,
) -> Result<HttpResponse, HandlerError> {
    service.create_order(post_data.into_inner()).await?;

    Ok(HttpResponse::Created().into())
}

#[get("/{order_id}")]
pub(super) async fn show(
    info: web::Path<String>,
    service: web::Data<dyn OrderServiceInterface>,
) -> Result<HttpResponse, HandlerError> {
    let order = service.get_order_by_id(info.into_inner()).await?;

    if let Some(o) = order {
        return Ok(HttpResponse::Ok().json(o));
    }

    Ok(HttpResponse::NotFound().finish())
}

// #[get("/")]
// pub(super) async fn index(
//     service: web::Data<dyn OrderServiceInterface>,
// ) -> Result<web::Json<Vec<OrderDTO>>, HandlerError> {
//     let orders = service.get_all_orders().await?;
//
//     Ok(web::Json(orders.into()))
// }
