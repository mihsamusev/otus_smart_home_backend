use crate::repository::room::Repository;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger::Env;
use std::net::TcpListener;
use std::sync::Arc;

pub mod room;
pub mod device;
pub mod device_query;

async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn spawn<R: Repository>(listener: TcpListener, repo: Arc<R>) -> Result<Server, std::io::Error> {
    let app_data = web::Data::from(repo);

    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .route("/", web::get().to(healthcheck))
            .route("/room/{room_id}", web::post().to(room::add_room::<R>))
            .route("/room/{room_id}", web::get().to(room::fetch_room::<R>))
            .route("/room/{room_id}/device", web::post().to(device::add_device::<R>))
            .route("/status/{room_id}/{device_id}", web::get().to(device_query::get_status::<R>))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
