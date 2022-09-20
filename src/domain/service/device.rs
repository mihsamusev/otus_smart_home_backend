use crate::domain::entity::{DeviceInfo, DeviceName, DeviceType, RoomName};
use crate::repository::room::{DeleteError, FetchError, InsertError, Repository};
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

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
    NotFound,
}

pub fn add_device<R: Repository>(repo: Arc<R>, request: AddRequest) -> Result<Response, Error> {
    let room_name = RoomName::try_from(request.room_name).map_err(|_| Error::BadRequest)?;

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
                Err(InsertError::Conflict) => Err(Error::Conflict),
                Err(InsertError::Unknown) => Err(Error::Unknown),
            }
        }
        _ => Err(Error::BadRequest),
    }
}

pub fn fetch_device<R: Repository>(repo: Arc<R>, request: FetchRequest) -> Result<Response, Error> {
    let device_name = DeviceName::try_from(request.device_name).map_err(|_| Error::BadRequest)?;
    let room_name = RoomName::try_from(request.room_name.clone()).map_err(|_| Error::BadRequest)?;

    match repo.fetch_device(room_name, device_name) {
        Ok(device_info) => Ok(Response {
            room_name: request.room_name,
            device_name: device_info.name.into(),
            address: device_info.address.to_string(),
            device_type: device_info.device_type.into(),
        }),
        Err(FetchError::Unknown) => Err(Error::Unknown),
        Err(FetchError::NotFound) => Err(Error::NotFound),
    }
}

pub fn delete_device<R: Repository>(repo: Arc<R>, request: FetchRequest) -> Result<(), Error> {
    let device_name = DeviceName::try_from(request.device_name).map_err(|_| Error::BadRequest)?;
    let room_name = RoomName::try_from(request.room_name).map_err(|_| Error::BadRequest)?;

    match repo.delete_device(room_name, device_name) {
        Ok(_) => Ok(()),
        Err(DeleteError::Unknown) => Err(Error::Unknown),
        Err(DeleteError::NotFound) => Err(Error::NotFound),
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
            Err(Error::BadRequest) => {}
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
            Err(Error::BadRequest) => {}
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
            Err(Error::BadRequest) => {}
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
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_conflict_if_device_already_exists_in_the_same_room() {
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
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn add_device_returns_conflict_if_new_device_is_on_the_same_address_across_entire_home() {
        // in the same room_name or in general?

        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();
        repo.add_room(RoomName::bathroom()).ok();

        let request = AddRequest {
            room_name: RoomName::bathroom().into(),
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
            Err(Error::Conflict) => {}
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
            Err(Error::Unknown) => {}
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
            Err(Error::Unknown) => {}
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
            Err(Error::NotFound) => {}
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

    #[test]
    fn delete_device_errors_if_device_doesnt_exist() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = FetchRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
        };
        match delete_device(repo, request) {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn delete_device_errors_if_room_doesnt_exist() {
        let repo = Arc::new(InMemoryRepository::new());

        let request = FetchRequest {
            room_name: RoomName::kitchen().into(),
            device_name: DeviceName::socket().into(),
        };
        match delete_device(repo, request) {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn delete_device_success() {
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
        match delete_device(repo, request) {
            Ok(_) => {}
            _ => unreachable!(),
        }
    }
}
