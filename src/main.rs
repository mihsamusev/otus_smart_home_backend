use clap::{self, App, Arg};
use smart_home_backend::api;
use smart_home_backend::repository::room::InMemoryRepository;
use std::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(Arg::with_name("http").long("http").help("Runs in HTTP server mode"))
        .get_matches();

    let listener = TcpListener::bind("127.0.0.1:8888").expect("Undable to bind to port");
    api::spawn(listener, repo)?.await
}
