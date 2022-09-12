use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::domain::service::device;
use crate::repository::room::Repository;

#[derive(Deserialize)]
pub struct AddDeviceRequest {
    pub name: String,
    pub address: String,
    pub device_type: String
}

#[derive(Serialize)]
pub struct AddDeviceResponse {
    pub room: String,
    pub name: String,
    pub address: String,
    pub device_type: String
}

impl From<device::Response> for AddDeviceResponse {
    fn from(inner: device::Response) -> Self {
        Self {
            room: inner.room,
            name: inner.name,
            address: inner.address,
            device_type: inner.device_type
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
        room: room_id.into_inner(),
        name: req.name,
        address: req.address,
        device_type: req.device_type
    };

    match device::add_device(repo.into_inner(), service_req) {
        Ok(res) => HttpResponse::Ok().json(web::Json(AddDeviceResponse::from(res))),
        Err(device::AddDeviceError::BadRequest) => HttpResponse::BadRequest().body("Wrong device format"),
        Err(device::AddDeviceError::Conflict) => {
            HttpResponse::Conflict().body("device with this name or IP address already exists")
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}