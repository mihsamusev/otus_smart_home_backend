use std::net::SocketAddr;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::domain::entity::{DeviceInfo, DeviceType, DeviceName};

use crate::repository::room::{FetchOneError, Repository};
#[derive(Deserialize)]
pub struct StatusRequest {
    pub room_id: String,
    pub device_id: String
}

#[derive(Serialize)]
pub struct StatusResponse {
    room_id: String,
    device_id: String,
    message: String
}

pub enum StatusError {
    NotFound,
    BadRequest,
    Unknown
}

pub fn get_status<R: Repository>(request: StatusRequest, repo: Arc<R>) -> Result<StatusResponse, StatusError> {
    // try pull the DeviceInfo from the repository
    let device_name = DeviceName::try_from(request.device_id.clone()).map_err(|_| StatusError::BadRequest)?;

    match repo.fetch_device(device_name) {
        Ok(device_info) => {
            let message = get_status_from_provider(device_info.address, device_info.device_type);
            Ok(StatusResponse {room_id: request.room_id, device_id: request.device_id, message})
        },
        Err(FetchOneError::Unknown) => Err(StatusError::Unknown),
        Err(FetchOneError::NotFound) => Err(StatusError::NotFound),
    }
    // try use the Device info to on a device info provider
}

fn get_status_from_provider(address: SocketAddr, device_type: DeviceType) -> String {
    "ok".to_string()
}