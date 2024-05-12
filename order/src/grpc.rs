use order::ordergrpc::order_service_server::OrderServiceServer;
use order::presentation::grpc::service::Service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = Service::default();

    Server::builder()
        .add_service(OrderServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
