use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::domain::service::device;
use crate::repository::room::Repository;

#[derive(Deserialize)]
pub struct AddDeviceRequest {
    pub device_name: String,
    pub address: String,
    pub device_type: String,
}

#[derive(Serialize)]
pub struct AddDeviceResponse {
    pub room_name: String,
    pub device_name: String,
    pub address: String,
    pub device_type: String,
}

impl From<device::Response> for AddDeviceResponse {
    fn from(inner: device::Response) -> Self {
        Self {
            room_name: inner.room_name,
            device_name: inner.device_name,
            address: inner.address,
            device_type: inner.device_type,
        }
    }
}

pub async fn add_device<R: Repository>(
    room_id: web::Path<String>,
    req: web::Json<AddDeviceRequest>,
    repo: web::Data<R>,
) -> HttpResponse {
    let req = req.into_inner();
    let service_req = device::AddRequest {
        room_name: room_id.into_inner(),
        device_name: req.device_name,
        address: req.address,
        device_type: req.device_type,
    };

    match device::add_device(repo.into_inner(), service_req) {
        Ok(res) => HttpResponse::Ok().json(web::Json(AddDeviceResponse::from(res))),
        Err(device::Error::BadRequest) => HttpResponse::BadRequest().body("Wrong device format"),
        Err(device::Error::Conflict) => {
            HttpResponse::Conflict().body("device with this name or IP address already exists")
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn fetch_device<R: Repository>(
    param: web::Path<(String, String)>,
    repo: web::Data<R>,
) -> HttpResponse {
    let (room_name, device_name) = param.into_inner();
    let service_req = device::FetchRequest {
        room_name,
        device_name,
    };

    match device::fetch_device(repo.into_inner(), service_req) {
        Ok(res) => HttpResponse::Ok().json(web::Json(AddDeviceResponse::from(res))),
        Err(device::Error::BadRequest) => HttpResponse::BadRequest().body("Wrong device format"),
        Err(device::Error::NotFound) => {
            HttpResponse::NotFound().body("requested device or room were not found")
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_device<R: Repository>(
    param: web::Path<(String, String)>,
    repo: web::Data<R>,
) -> HttpResponse {
    let (room_name, device_name) = param.into_inner();
    let service_req = device::FetchRequest {
        room_name,
        device_name,
    };
    match device::delete_device(repo.into_inner(), service_req) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(device::Error::BadRequest) => HttpResponse::BadRequest().body("Wrong device format"),
        Err(device::Error::NotFound) => {
            HttpResponse::NotFound().body("requested device or room were not found")
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}
