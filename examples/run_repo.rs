use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

struct AppState {
    data: Mutex<String>,
}

async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct AppendRequest {
    data: String,
}

#[derive(Serialize)]
struct AppendResponse {
    data: String,
}

async fn append_to_state(
    req: web::Json<AppendRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let mut state = match state.data.lock() {
        Ok(state) => state,
        _ => return HttpResponse::InternalServerError().finish(),
    };
    state.push_str(req.data.as_str());

    HttpResponse::Ok().json(web::Json(AppendResponse {
        data: state.clone(),
    }))
}

fn spawn(listener: TcpListener) -> Result<Server, std::io::Error> {
    let data = AppState {
        data: Mutex::new(String::with_capacity(1000)),
    };
    let data = Arc::new(data);
    let state_data = web::Data::from(data);

    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state_data.clone())
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/append", web::post().to(append_to_state))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Undable to bind to port");
    spawn(listener)?.await
}
