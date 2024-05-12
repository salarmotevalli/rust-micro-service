pub mod repository;

use mongodb::Client;

pub type Conn = Client;

pub async fn client() -> Conn {
    let uri = std::env::var("MONGODB_URI").expect("The \"MONGODB_URI\" variable is not set.");

    Client::with_uri_str(uri).await.expect("failed to connect")
}
