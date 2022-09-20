use crate::domain::service::device_query;
use crate::repository::room::Repository;
use actix_web::{web, HttpResponse};

pub async fn get_device_status<R: Repository>(
    param: web::Path<(String, String)>,
    repo: web::Data<R>,
) -> HttpResponse {
    let (room_id, device_id) = param.into_inner();
    let service_req = device_query::StatusRequest { room_id, device_id };

    match device_query::get_device_status(service_req, repo.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(web::Json(res)),
        Err(device_query::StatusError::BadRequest) => HttpResponse::BadRequest().finish(),
        Err(device_query::StatusError::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_room_status<R: Repository>(
    room_id: web::Path<String>,
    repo: web::Data<R>,
) -> HttpResponse {
    match device_query::get_room_status(room_id.into_inner(), repo.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(web::Json(res)),
        Err(device_query::StatusError::BadRequest) => HttpResponse::BadRequest().finish(),
        Err(device_query::StatusError::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
