use crate::domain::entity::{RoomInfo, DeviceInfo, RoomName, DeviceName};
use std::sync::Mutex;

pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchAllError {
    Unknown,
}

pub enum FetchOneError {
    NotFound,
    Unknown,
}

pub enum DeleteError {
    NotFound,
    Unknown,
}

pub trait Repository: Send + Sync {
    fn add_room(&self, name: RoomName) -> Result<RoomInfo, InsertError>;

    // fn delete_room(&mut self) -> Result<(), DeleteError>;

    fn fetch_room(&self, name: RoomName) -> Result<RoomInfo, FetchOneError>;

    // fn fetch_rooms(&self) -> Result<Vec<Room>, FetchAllError>;

    fn add_device(&self, room: RoomName, device: DeviceName) -> Result<DeviceInfo, InsertError>;

    // fn delete_device(&mut self) -> Result<(), DeleteError>;

    fn fetch_device(&self, name: DeviceName) -> Result<DeviceInfo, FetchOneError>;

    // fn fetch_devices(&self) -> Result<Vec<DeviceInfo>, FetchAllError>;
}

pub struct ImMemoryRepository {
    returns_error: bool,
    rooms: Mutex<Vec<RoomInfo>>
}

impl ImMemoryRepository {
    pub fn new() -> Self {
        Self {
            returns_error: false,
            rooms: Mutex::new(Vec::new())
        }
    }

    #[cfg(test)]
    pub fn with_error(self) -> Self {
        Self {
            returns_error: true,
            ..self
        }
    }
}

impl Repository for ImMemoryRepository {
    fn add_room(&self, name: RoomName) -> Result<RoomInfo, InsertError> {
        if self.returns_error {
            return Err(InsertError::Unknown);
        }

        let mut rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(InsertError::Unknown)
        };

        if rooms.iter().any(|room| room.name == name) {
            return Err(InsertError::Conflict);
        }

        let new_room = RoomInfo {
            name: name,
            devices: Vec::new()
        };
        rooms.push(new_room.clone());
        
        Ok(new_room)
    }

    fn fetch_room(&self, name: RoomName) -> Result<RoomInfo, FetchOneError> {
        if self.returns_error {
            return Err(FetchOneError::Unknown);
        }

        let rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(FetchOneError::Unknown)
        };

        match rooms.iter().find(|room| room.name == name) {
            Some(room) => Ok(room.clone()),
            _ => Err(FetchOneError::NotFound)
        }

    }

    fn add_device(&self, room: RoomName, name: DeviceName) -> Result<DeviceInfo, InsertError> {
        Err(InsertError::Unknown)
    }

    fn fetch_device(&self, name: DeviceName) -> Result<DeviceInfo, FetchOneError> {
        Err(FetchOneError::Unknown)
    }
}

