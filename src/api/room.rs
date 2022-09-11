use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

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

pub async fn add_room<R: Repository>(
    req: web::Json<AddRoomRequest>,
    repo: web::Data<R>,
) -> HttpResponse {
    let service_req = room::RoomRequest::from(req.into_inner().into());

    match room::add_room(repo.into_inner(), service_req) {
        Ok(res) => HttpResponse::Ok().json(web::Json(AddRoomResponse::from(res))),
        Err(room::Error::BadRequest) => HttpResponse::BadRequest().body("Wrong room format"),
        Err(room::Error::Conflict) => {
            HttpResponse::Conflict().body("room with this name already exists")
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}
