use actix_web::{guard, web};

use super::handlers::{create, show};

pub fn order_scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order")
            .guard(guard::Header("content-type", "application/json"))
            .service(create)
            // .service(index)
            .service(show),
    );
}
