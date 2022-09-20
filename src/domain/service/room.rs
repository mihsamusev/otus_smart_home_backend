use crate::domain::entity::{self, RoomName};
use crate::repository::room::{DeleteError, FetchError, InsertError, Repository};
use std::sync::Arc;

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
    NotFound,
}

#[derive(Debug)]
pub struct RoomRequest {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RoomResponse {
    pub name: String,
    pub devices: Vec<DeviceResponse>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceResponse {
    pub name: String,
    pub address: String,
    pub device_type: String,
}

impl From<entity::RoomInfo> for RoomResponse {
    fn from(inner: entity::RoomInfo) -> Self {
        Self {
            name: String::from(inner.name),
            devices: inner
                .devices
                .into_iter()
                .map(DeviceResponse::from)
                .collect(),
        }
    }
}
impl From<entity::DeviceInfo> for DeviceResponse {
    fn from(inner: entity::DeviceInfo) -> Self {
        Self {
            name: String::from(inner.name),
            address: inner.address.to_string(),
            device_type: String::from(inner.device_type),
        }
    }
}

pub fn add_room<R: Repository>(repo: Arc<R>, req: RoomRequest) -> Result<RoomResponse, Error> {
    let room_name = RoomName::try_from(req.name).map_err(|_| Error::BadRequest)?;
    match repo.add_room(room_name) {
        Ok(room_info) => Ok(RoomResponse {
            name: String::from(room_info.name),
            devices: Vec::new(),
        }),
        Err(InsertError::Conflict) => Err(Error::Conflict),
        Err(InsertError::Unknown) => Err(Error::Unknown),
    }
}

pub fn fetch_room<R: Repository>(repo: Arc<R>, req: RoomRequest) -> Result<RoomResponse, Error> {
    let room_name = RoomName::try_from(req.name).map_err(|_| Error::BadRequest)?;
    match repo.fetch_room(room_name) {
        Ok(room_info) => Ok(RoomResponse {
            name: String::from(room_info.name),
            devices: room_info
                .devices
                .into_iter()
                .map(DeviceResponse::from)
                .collect(),
        }),
        Err(FetchError::NotFound) => Err(Error::NotFound),
        Err(FetchError::Unknown) => Err(Error::Unknown),
    }
}

pub fn fetch_rooms<R: Repository>(repo: Arc<R>) -> Result<Vec<RoomResponse>, Error> {
    match repo.fetch_rooms() {
        Ok(room_infos) => Ok(room_infos.into_iter().map(RoomResponse::from).collect()),
        Err(FetchError::NotFound) => Err(Error::NotFound),
        Err(FetchError::Unknown) => Err(Error::Unknown),
    }
}

pub fn delete_room<R: Repository>(repo: Arc<R>, req: RoomRequest) -> Result<(), Error> {
    let room_name = RoomName::try_from(req.name).map_err(|_| Error::BadRequest)?;
    match repo.delete_room(room_name) {
        Ok(()) => Ok(()),
        Err(DeleteError::NotFound) => Err(Error::NotFound),
        Err(_) => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::room::InMemoryRepository;
    #[test]
    fn add_room_returns_bad_request_error_on_invalid_input() {
        // invalid input is empty room name
        let repo = Arc::new(InMemoryRepository::new());
        let request = RoomRequest {
            name: RoomName::empty().into(),
        };
        match add_room(repo, request) {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn add_room_returns_conflict_error_if_room_already_exists() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = RoomRequest {
            name: RoomName::kitchen().into(),
        };
        match add_room(repo, request) {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn add_room_returns_unknown_error_if_if_repo_errors_unexpectidly() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let request = RoomRequest {
            name: RoomName::kitchen().into(),
        };
        match add_room(repo, request) {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn add_room_returns_empty_room_on_success() {
        let repo = Arc::new(InMemoryRepository::new());
        let request = RoomRequest {
            name: RoomName::kitchen().into(),
        };
        match add_room(repo, request) {
            Ok(result) => {
                assert_eq!(result.name, String::from(RoomName::kitchen()));
                assert_eq!(result.devices, Vec::new());
            }
            _ => unreachable!(),
        };
    }

    #[test]
    fn fetch_room_returns_not_found_error_if_repo_doesnt_contain_room() {
        let repo = Arc::new(InMemoryRepository::new());
        let request = RoomRequest {
            name: RoomName::kitchen().into(),
        };
        match fetch_room(repo, request) {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn fetch_room_returns_unknown_error_if_repo_errors_unexpectidly() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let request = RoomRequest {
            name: RoomName::kitchen().into(),
        };
        match fetch_room(repo, request) {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn fetch_room_returns_room_on_success() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();
        repo.add_room(RoomName::bathroom()).ok();

        let request = RoomRequest {
            name: RoomName::kitchen().into(),
        };
        match fetch_room(repo, request) {
            Ok(result) => {
                assert_eq!(result.name, String::from(RoomName::kitchen()));
                assert_eq!(result.devices, Vec::new());
            }
            _ => unreachable!(),
        };
    }

    #[test]
    fn fetch_rooms_returns_zero_rooms() {
        let repo = Arc::new(InMemoryRepository::new());

        match fetch_rooms(repo) {
            Ok(result) => assert_eq!(result, vec![]),
            _ => unreachable!(),
        };
    }
    #[test]
    fn fetch_rooms_returns_two_rooms() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();
        repo.add_room(RoomName::bathroom()).ok();

        match fetch_rooms(repo) {
            Ok(result) => assert_eq!(
                result,
                vec![
                    RoomResponse {
                        name: "kitchen".to_string(),
                        devices: vec![]
                    },
                    RoomResponse {
                        name: "bathroom".to_string(),
                        devices: vec![]
                    }
                ]
            ),
            _ => unreachable!(),
        };
    }

    #[test]
    fn delete_room_errors_if_room_doesnt_exist() {
        let repo = Arc::new(InMemoryRepository::new());
        let request = RoomRequest {
            name: RoomName::kitchen().into(),
        };
        match delete_room(repo, request) {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn delete_room_deletes_room() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();
        let request = RoomRequest {
            name: RoomName::kitchen().into(),
        };
        delete_room(repo.clone(), request).ok();

        match fetch_rooms(repo) {
            Ok(result) => assert_eq!(result, vec![]),
            _ => unreachable!(),
        };
    }
}
