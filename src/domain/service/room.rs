use crate::domain::entity::{RoomName};
use crate::repository::room::{Repository, InsertError, FetchOneError};
use std::sync::Arc;

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
    NotFound
}

pub struct RoomRequest {
    pub name: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct RoomResponse {
    name: String,
    devices: Vec<DeviceResponse>
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceResponse {
    name: String,
    device_type: String
}

pub fn add_room<R: Repository>(repo: Arc<R>, req: RoomRequest) -> Result<RoomResponse, Error> {
    let room_name = RoomName::try_from(req.name).map_err(|_| Error::BadRequest)?;
    match repo.add_room(room_name) {
        Ok(room_info) => Ok(RoomResponse {
            name: String::from(room_info.name),
            devices: Vec::new()
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
            devices: Vec::new()
        }),
        Err(FetchOneError::NotFound) => Err(Error::NotFound),
        Err(FetchOneError::Unknown) => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::room::ImMemoryRepository;
    #[test]
    fn add_room_returns_bad_request_error_on_invalid_input() {
        // invalid input is empty room name
        let repo = Arc::new(ImMemoryRepository::new());
        let request = RoomRequest {
            name: RoomName::empty().into()
        };
        match add_room(repo, request) {
            Err(Error::BadRequest) => {},
            _ => unreachable!()
        };
    }

    #[test]
    fn add_room_returns_conflict_error_if_room_already_exists() {
        let repo = Arc::new(ImMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();

        let request = RoomRequest {
            name: RoomName::kitchen().into()
        };
        match add_room(repo, request) {
            Err(Error::Conflict) => {},
            _ => unreachable!()
        };
    }

    #[test]
    fn add_room_returns_unknown_error_if_if_repo_errors_unexpectidly() {
        let repo = Arc::new(ImMemoryRepository::new().with_error());
        let request = RoomRequest {
            name: RoomName::kitchen().into()
        };
        match add_room(repo, request) {
            Err(Error::Unknown) => {},
            _ => unreachable!()
        };
    }

    #[test]
    fn add_room_returns_empty_room_on_success() {
        let repo = Arc::new(ImMemoryRepository::new());
        let request = RoomRequest {
            name: RoomName::kitchen().into()
        };
        match add_room(repo, request) {
            Ok(result ) => {
                assert_eq!(result.name, String::from(RoomName::kitchen()));
                assert_eq!(result.devices, Vec::new());
            },
            _ => unreachable!()
        };
    }

    #[test]
    fn fetch_room_returns_not_found_error_if_repo_doesnt_contain_room() {
        let repo = Arc::new(ImMemoryRepository::new());
        let request = RoomRequest {
            name: RoomName::kitchen().into()
        };
        match fetch_room(repo, request) {
            Err(Error::NotFound) => {},
            _ => unreachable!()
        };
    }

    #[test]
    fn fetch_room_returns_unknown_error_if_repo_errors_unexpectidly() {
        let repo = Arc::new(ImMemoryRepository::new().with_error());
        let request = RoomRequest {
            name: RoomName::kitchen().into()
        };
        match fetch_room(repo, request) {
            Err(Error::Unknown) => {},
            _ => unreachable!()
        };
    }

    #[test]
    fn fetch_room_returns_room_on_success() {
        let repo = Arc::new(ImMemoryRepository::new());
        repo.add_room(RoomName::kitchen()).ok();
        repo.add_room(RoomName::bathroom()).ok();

        let request = RoomRequest {
            name: RoomName::kitchen().into()
        };
        match fetch_room(repo, request) {
            Ok(result) => {
                assert_eq!(result.name, String::from(RoomName::kitchen()));
                assert_eq!(result.devices, Vec::new());
            },
            _ => unreachable!()
        };
    }



}