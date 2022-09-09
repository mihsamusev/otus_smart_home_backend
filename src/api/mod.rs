


use actix_web::dev::Server;
use actix_web::web::{self, Data};
use actix_web::{App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use std::net::TcpListener;
use std::sync::Arc;
use crate::repository::room::Repository;
pub mod room;



async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn spawn<R: Repository>(listener: TcpListener, repo: Arc<R>) -> Result<Server, std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let server = HttpServer::new(move || {
    App::new()
    .wrap(Logger::default())
    .app_data(Data::new(repo.clone()))
    .route("/healthcheck", web::get().to(healthcheck))
    .route("/room", web::post().to(room::add_room::<R>))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
    