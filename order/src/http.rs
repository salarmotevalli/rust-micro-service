use actix_web::middleware::Logger;
use actix_web::web::{self, scope};
use actix_web::App;
use actix_web::HttpServer;
use order::cnf::Cnf;
use order::container::Container;
use order::presentation::http::routes::order_scoped_config;

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cnf = Cnf::load();

    let container = Container::new().await;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(container.order_service.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(scope("/api").configure(order_scoped_config))
    })
    .bind(("127.0.0.1", cnf.port))?
    .run()
    .await
}
