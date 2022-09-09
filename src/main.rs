use smart_home_backend::api;
use smart_home_backend::repository::room::InMemoryRepository;
use std::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let repo = Arc::new(InMemoryRepository::new());
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Undable to bind to port");
    api::spawn(listener, repo)?.await
}
