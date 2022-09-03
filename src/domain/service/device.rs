use crate::domain::entity::{DeviceInfo, DeviceName, DeviceType, RoomName};
use crate::repository::room::{InsertError, Repository};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

pub struct Request {
    room: String,
    name: String,
    address: String,
    device_type: String,
}

pub struct Response {
    pub room: String,
    pub name: String,
    pub address: String,
    pub device_type: String,
}

pub enum Error {
    BadRequest,
    Conflict,
    RoomNotFound,
    Unknown,
}

pub fn add_device<R: Repository>(repo: Arc<R>, request: Request) -> Result<Response, Error> {
    let room_name = RoomName::try_from(request.room).map_err(|_| Error::BadRequest)?;

    match (
        DeviceName::try_from(request.name),
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
                    room: room_name.into(),
                    name: device_info.name.into(),
                    address: device_info.address.to_string(),
                    device_type: device_info.device_type.into(),
                }),
                Err(InsertError::Conflict) => Err(Error::Conflict),
                Err(InsertError::Unknown) => Err(Error::Unknown),
            }
        }
        _ => Err(Error::BadRequest),
    }
}

impl Request {
    pub fn new(room: &str, name: &str, address: &str, device_type: &str) -> Self {
        Self {
            room: room.into(),
            name: name.into(),
            address: address.into(),
            device_type: device_type.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::room::ImMemoryRepository;

    #[test]
    fn add_device_returns_bad_request_on_invalid_input() {
        let repo = Arc::new(ImMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        // empty device name
        let request = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::empty().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo.clone(), request) {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }

        // incorrect ip adress
        let request = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::socket().into(),
            address: "127.0.0:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo.clone(), request) {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }

        // incorrect device type
        let request = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: "dumb_socket".to_string(),
        };

        match add_device(repo, request) {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_not_found_error_if_target_room_not_found() {
        let repo = Arc::new(ImMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        // empty device name
        let request = Request {
            room: RoomName::bathroom().into(),
            name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo.clone(), request) {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_conflict_if_device_already_exists() {
        let repo = Arc::new(ImMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        add_device(repo.clone(), request).ok();

        let request_again = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::socket().into(),
            address: "127.0.0.1:9999".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        match add_device(repo, request_again) {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_conflict_if_new_device_is_on_the_same_address() {
        let repo = Arc::new(ImMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        add_device(repo.clone(), request).ok();

        let request_again = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::thermo().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };
        match add_device(repo, request_again) {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_unknown_error_if_repo_errors_unexpectidly() {
        let repo = Arc::new(ImMemoryRepository::new().with_error());
        repo.add_room(RoomName::kitchen()).ok();

        // empty device name
        let request = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo.clone(), request) {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_device_info_on_success() {
        let repo = Arc::new(ImMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = Request {
            room: RoomName::kitchen().into(),
            name: DeviceName::socket().into(),
            address: "127.0.0.1:8888".to_string(),
            device_type: DeviceType::TcpSocket.into(),
        };

        match add_device(repo, request) {
            Ok(result) => {
                assert_eq!(result.name, String::from(DeviceName::socket()));
                assert_eq!(result.address, String::from("127.0.0.1:8888"));
                assert_eq!(result.device_type, String::from(DeviceType::TcpSocket));
            }
            _ => unreachable!(),
        }
    }
}
