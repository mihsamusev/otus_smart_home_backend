use crate::domain::entity::{DeviceInfo, DeviceName, RoomInfo, RoomName};
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

pub trait Repository: Send + Sync + 'static {
    fn add_room(&self, name: RoomName) -> Result<RoomInfo, InsertError>;

    // fn delete_room(&mut self) -> Result<(), DeleteError>;

    fn fetch_room(&self, name: RoomName) -> Result<RoomInfo, FetchOneError>;

    // fn fetch_rooms(&self) -> Result<Vec<Room>, FetchAllError>;

    fn add_device(
        &self,
        room_name: RoomName,
        device_info: DeviceInfo,
    ) -> Result<DeviceInfo, InsertError>;

    // fn delete_device(&mut self) -> Result<(), DeleteError>;

    fn fetch_device(&self, name: DeviceName) -> Result<DeviceInfo, FetchOneError>;

    // fn fetch_devices(&self) -> Result<Vec<DeviceInfo>, FetchAllError>;
}

pub struct InMemoryRepository {
    returns_error: bool,
    rooms: Mutex<Vec<RoomInfo>>,
}

impl Default for InMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            returns_error: false,
            rooms: Mutex::new(Vec::new()),
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

impl Repository for InMemoryRepository {
    fn add_room(&self, name: RoomName) -> Result<RoomInfo, InsertError> {
        if self.returns_error {
            return Err(InsertError::Unknown);
        }

        let mut rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(InsertError::Unknown),
        };

        if rooms.iter().any(|room| room.name == name) {
            return Err(InsertError::Conflict);
        }

        let new_room = RoomInfo {
            name,
            devices: Vec::new(),
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
            _ => return Err(FetchOneError::Unknown),
        };

        match rooms.iter().find(|room| room.name == name) {
            Some(room) => Ok(room.clone()),
            _ => Err(FetchOneError::NotFound),
        }
    }

    fn add_device(
        &self,
        room_name: RoomName,
        device_info: DeviceInfo,
    ) -> Result<DeviceInfo, InsertError> {
        if self.returns_error {
            return Err(InsertError::Unknown);
        }

        let mut rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(InsertError::Unknown),
        };

        match rooms.iter_mut().find(|room| room.name == room_name) {
            Some(room) => {
                if room
                    .devices
                    .iter()
                    .any(|d| d.name == device_info.name || d.address == device_info.address)
                {
                    return Err(InsertError::Conflict);
                }
                room.devices.push(device_info.clone());
                Ok(device_info)
            }
            _ => Err(InsertError::Conflict),
        }
    }

    fn fetch_device(&self, name: DeviceName) -> Result<DeviceInfo, FetchOneError> {
        if self.returns_error {
            return Err(FetchOneError::Unknown);
        }

        let rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(FetchOneError::Unknown),
        };

        for room in rooms.iter() {
            match room.devices.iter().find(|d| d.name == name) {
                Some(device) => {
                    return Ok(device.clone());
                }
                _ => {
                    return Err(FetchOneError::NotFound);
                }
            };
        }
        Err(FetchOneError::NotFound)
    }
}
