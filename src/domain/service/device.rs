use crate::domain::entity::{DeviceInfo, DeviceName, DeviceType, RoomName};
use crate::repository::room::{FetchError, InsertError, Repository};
use std::convert::TryFrom;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

pub struct AddRequest {
    pub room_name: String,
    pub device_name: String,
    pub address: String,
    pub device_type: String,
}
pub struct FetchRequest {
    pub room_name: String,
    pub device_name: String,
}
pub struct Response {
    pub room_name: String,
    pub device_name: String,
    pub address: String,
    pub device_type: String,
}

pub enum AddDeviceError {
    BadRequest,
    Conflict,
    Unknown,
}

pub enum FetchDeviceError {
    BadRequest,
    NotFound,
    Unknown,
}

pub fn add_device<R: Repository>(
    repo: Arc<R>,
    request: AddRequest,
) -> Result<Response, AddDeviceError> {
    let room_name = RoomName::try_from(request.room_name).map_err(|_| AddDeviceError::BadRequest)?;

    match (
        DeviceName::try_from(request.device_name),
        SocketAddr::from_str(&request.address),
        DeviceType::try_from(request.device_type),
    ) {
        (Ok(name), Ok(address), Ok(device_type)) => {
            let device_info = DeviceInfo {
                name,
                address,
                device_type,
            };
            match repo.add_device(room_name.clone(), device_info) {
                Ok(device_info) => Ok(Response {
                    room_name: room_name.into(),
                    device_name: device_info.name.into(),
                    address: device_info.address.to_string(),
                    device_type: device_info.device_type.into(),
                }),
                Err(InsertError::Conflict) => Err(AddDeviceError::Conflict),
                Err(InsertError::Unknown) => Err(AddDeviceError::Unknown),
            }
        }
        _ => Err(AddDeviceError::BadRequest),
    }
}

pub fn fetch_device<R: Repository>(
    repo: Arc<R>,
    request: FetchRequest,
) -> Result<Response, FetchDeviceError> {
    let device_name =DeviceName::try_from(request.device_name)
        .map_err(|_| FetchDeviceError::BadRequest)?;
    let room_name = RoomName::try_from(request.room_name.clone())
        .map_err(|_| FetchDeviceError::BadRequest)?;

    match repo.fetch_device(room_name, device_name) {
        Ok(device_info) => Ok(Response {
            room_name: request.room_name.into(), // room_name.into(), TODO: WTF, device_info should have room_name?
            device_name: device_info.name.into(),
            address: device_info.address.to_string(),
            device_type: device_info.device_type.into(),
        }),
        Err(FetchError::Unknown) => Err(FetchDeviceError::Unknown),
        Err(FetchError::NotFound) => Err(FetchDeviceError::NotFound),
    }
}

impl AddRequest {
    pub fn new(room_name: &str, device_name: &str, address: &str, device_type: &str) -> Self {
        Self {
            room_name: room_name.into(),
            device_name: device_name.into(),
            address: address.into(),
            device_type: device_type.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::room::InMemoryRepository;

    #[test]
    fn add_device_returns_bad_request_on_invalid_input() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        // empty device name
        let request = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::empty().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo.clone(), request) {
            Err(AddDeviceError::BadRequest) => {}
            _ => unreachable!(),
        }

        // incorrect ip adress
        let request = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo.clone(), request) {
            Err(AddDeviceError::BadRequest) => {}
            _ => unreachable!(),
        }

        // incorrect device type
        let request = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: "dumb_socket".to_string(),
        };

        match add_device(repo, request) {
            Err(AddDeviceError::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_not_found_error_if_target_room_not_found() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        // empty device name
        let request = AddRequest {
            room_name: RoomName::bathroom().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo, request) {
            Err(AddDeviceError::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_conflict_if_device_already_exists() {
        // in the same room_name or in general?

        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        add_device(repo.clone(), request).ok();

        let request_again = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0.1:9999".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        match add_device(repo, request_again) {
            Err(AddDeviceError::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_conflict_if_new_device_is_on_the_same_address() {
        // in the same room_name or in general?

        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        add_device(repo.clone(), request).ok();

        let request_again = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::thermo().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        match add_device(repo, request_again) {
            Err(AddDeviceError::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_unknown_error_if_repo_errors_unexpectidly() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        repo.add_room(RoomName::kitchen()).ok();

        // empty device name
        let request = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo, request) {
            Err(AddDeviceError::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_device_info_on_success() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo, request) {
            Ok(result) => {
                assert_eq!(result.device_name, String::from(DeviceName::socket()));
                assert_eq!(result.address, String::from("127.0.0.1:8888"));
                assert_eq!(result.device_type, String::from(DeviceType::TcpSocket));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn fetch_device_returns_unknown_error_if_repo_errors_unexpectidly() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        repo.add_room(RoomName::kitchen()).ok();

        let request = FetchRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
        };

        match fetch_device(repo, request) {
            Err(FetchDeviceError::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn fetch_device_returns_not_found_if_repo_doesnt_contain_device() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = FetchRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
        };

        match fetch_device(repo, request) {
            Err(FetchDeviceError::NotFound) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn fetch_device_returns_device_info_on_success() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = AddRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        add_device(repo.clone(), request).ok();

        let request = FetchRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
        };

        match fetch_device(repo, request) {
            Ok(result) => {
                assert_eq!(result.device_name, String::from(DeviceName::socket()));
                assert_eq!(result.address, String::from("127.0.0.1:8888"));
                assert_eq!(result.device_type, String::from(DeviceType::TcpSocket));
            }
            _ => unreachable!(),
        }
    }
}
