use crate::domain::client;
use crate::domain::entity::{DeviceInfo, DeviceName, DeviceType, RoomName};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::repository::room::{FetchError, Repository};
#[derive(Deserialize, Debug)]
pub struct StatusRequest {
    pub room_id: String,
    pub device_id: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    room_id: String,
    device_id: String,
    message: String,
}

pub enum StatusError {
    NotFound,
    BadRequest,
    Unknown,
}

pub fn get_status<R: Repository>(
    request: StatusRequest,
    repo: Arc<R>,
) -> Result<StatusResponse, StatusError> {
    // try pull the DeviceInfo from the repository
    let device_name =DeviceName::try_from(request.device_id.clone())
        .map_err(|_| StatusError::BadRequest)?;
    let room_name = RoomName::try_from(request.room_id.clone())
        .map_err(|_| StatusError::BadRequest)?;
    

    match repo.fetch_device(room_name, device_name) {
        Ok(DeviceInfo {name: _name, address, device_type}) => {
            let message = get_device_status_message(address, device_type);
            Ok(StatusResponse {
                room_id: request.room_id,
                device_id: request.device_id,
                message,
            })
        }
        Err(FetchError::Unknown) => Err(StatusError::Unknown),
        Err(FetchError::NotFound) => Err(StatusError::NotFound),
    }
    // try use the Device info to on a device info provider
}

fn get_device_status_message(address: SocketAddr, device_type: DeviceType) -> String {
    let result = match device_type {
        DeviceType::TcpSocket => client::get_socket_status(address),
        DeviceType::UdpThermo => client::get_thermo_status(address),
    };
    result.unwrap_or_else(|e| e.to_string())
}
