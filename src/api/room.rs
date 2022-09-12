use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::domain::entity;
use crate::domain::service::room;
use crate::repository::room::Repository;

#[derive(Deserialize)]
pub struct AddRoomRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct AddRoomResponse {
    pub name: String,
}

#[derive(Deserialize)]
pub struct FetchRoomRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct FetchRoomResponse {
    pub name: String,
    pub devices: Vec<RoomDeviceResponse>
}

#[derive(Serialize)]
pub struct RoomDeviceResponse {
    name: String,
    address: String,
    device_type: String,
}

impl Into<room::RoomRequest> for AddRoomRequest {
    fn into(self) -> room::RoomRequest {
        room::RoomRequest { name: self.name }
    }
}

impl From<room::RoomResponse> for AddRoomResponse {
    fn from(inner: room::RoomResponse) -> Self {
        Self { name: inner.name }
    }
}

impl Into<room::RoomRequest> for FetchRoomRequest {
    fn into(self) -> room::RoomRequest {
        room::RoomRequest { name: self.name }
    }
}

impl From<room::RoomResponse> for FetchRoomResponse {
    fn from(inner: room::RoomResponse) -> Self {
        Self {
            name: inner.name,
            devices: inner.devices
                .into_iter()
                .map(|res| RoomDeviceResponse {name: res.name, address: res.address, device_type: res.device_type}).collect()
        }
    }
}

pub async fn add_room<R: Repository>(
    room_id: web::Path<String>,
    repo: web::Data<R>,
) -> HttpResponse {
    let service_req = match entity::RoomName::try_from(room_id.into_inner()) {
        Ok(name) => room::RoomRequest {name: String::from(name)},
        Err(_) => {return HttpResponse::BadRequest().body("wrong format for room name");}
    };

    match room::add_room(repo.into_inner(), service_req) {
        Ok(res) => HttpResponse::Ok().json(web::Json(AddRoomResponse::from(res))),
        Err(room::Error::BadRequest) => HttpResponse::BadRequest().body("Wrong room format"),
        Err(room::Error::Conflict) => {
            HttpResponse::Conflict().body("room with this name already exists")
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn fetch_room<R: Repository>(
    room_id: web::Path<String>,
    repo: web::Data<R>,
) -> HttpResponse {
    let service_req = match entity::RoomName::try_from(room_id.into_inner()) {
        Ok(name) => room::RoomRequest {name: String::from(name)},
        Err(_) => {return HttpResponse::BadRequest().body("wrong format for room name");}
    };

    match room::fetch_room(repo.into_inner(), service_req) {
        Ok(res) => HttpResponse::Ok().json(web::Json(FetchRoomResponse::from(res))),
        Err(room::Error::BadRequest) => HttpResponse::BadRequest().body("Wrong room format"),
        Err(room::Error::NotFound) => {
            HttpResponse::NotFound().body("room not found")
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}
