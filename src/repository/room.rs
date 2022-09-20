use crate::domain::entity::{DeviceInfo, DeviceName, RoomInfo, RoomName};
use std::sync::Mutex;

pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchError {
    Unknown,
    NotFound,
}

pub enum DeleteError {
    NotFound,
    Unknown,
}

pub trait Repository: Send + Sync + 'static {
    fn add_room(&self, name: RoomName) -> Result<RoomInfo, InsertError>;

    fn delete_room(&self, name: RoomName) -> Result<(), DeleteError>;

    fn fetch_room(&self, name: RoomName) -> Result<RoomInfo, FetchError>;

    fn fetch_rooms(&self) -> Result<Vec<RoomInfo>, FetchError>;

    fn add_device(
        &self,
        room_name: RoomName,
        device_info: DeviceInfo,
    ) -> Result<DeviceInfo, InsertError>;

    fn delete_device(
        &self,
        room_name: RoomName,
        device_name: DeviceName,
    ) -> Result<(), DeleteError>;

    fn fetch_device(
        &self,
        room_name: RoomName,
        device_name: DeviceName,
    ) -> Result<DeviceInfo, FetchError>;

    fn fetch_devices(&self, room_name: RoomName) -> Result<Vec<DeviceInfo>, FetchError>;
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

    fn fetch_room(&self, name: RoomName) -> Result<RoomInfo, FetchError> {
        if self.returns_error {
            return Err(FetchError::Unknown);
        }

        let rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(FetchError::Unknown),
        };

        match rooms.iter().find(|room| room.name == name) {
            Some(room) => Ok(room.clone()),
            _ => Err(FetchError::NotFound),
        }
    }

    fn fetch_rooms(&self) -> Result<Vec<RoomInfo>, FetchError> {
        if self.returns_error {
            return Err(FetchError::Unknown);
        }

        let rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(FetchError::Unknown),
        };

        Ok(rooms.to_vec())
    }

    fn delete_room(&self, name: RoomName) -> Result<(), DeleteError> {
        if self.returns_error {
            return Err(DeleteError::Unknown);
        }

        let mut rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(DeleteError::Unknown),
        };

        let del_idx = match rooms.iter().position(|r| r.name == name) {
            Some(idx) => idx,
            None => return Err(DeleteError::NotFound),
        };

        rooms.remove(del_idx);
        Ok(())
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

        // check device with the same address cant in the same house
        let conflict_addresses = rooms
            .iter()
            .filter(|room| {
                room.devices
                    .iter()
                    .any(|d| d.address == device_info.address)
            })
            .count();
        if conflict_addresses > 0 {
            return Err(InsertError::Conflict);
        }

        // check device with the same name cant be in the same room
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

    fn fetch_device(
        &self,
        room_name: RoomName,
        device_name: DeviceName,
    ) -> Result<DeviceInfo, FetchError> {
        if self.returns_error {
            return Err(FetchError::Unknown);
        }

        let rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(FetchError::Unknown),
        };

        match rooms.iter().find(|r| r.name == room_name) {
            Some(room) => match room.devices.iter().find(|d| d.name == device_name) {
                Some(device) => Ok(device.clone()),
                None => Err(FetchError::NotFound),
            },
            None => Err(FetchError::NotFound),
        }
    }

    fn fetch_devices(&self, room_name: RoomName) -> Result<Vec<DeviceInfo>, FetchError> {
        if self.returns_error {
            return Err(FetchError::Unknown);
        }

        let rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(FetchError::Unknown),
        };

        match rooms.iter().find(|room| room.name == room_name) {
            Some(room) => Ok(room.devices.to_vec()),
            _ => Err(FetchError::NotFound),
        }
    }

    fn delete_device(
        &self,
        room_name: RoomName,
        device_name: DeviceName,
    ) -> Result<(), DeleteError> {
        if self.returns_error {
            return Err(DeleteError::Unknown);
        }

        let mut rooms = match self.rooms.lock() {
            Ok(rooms) => rooms,
            _ => return Err(DeleteError::Unknown),
        };

        match rooms.iter_mut().find(|r| r.name == room_name) {
            Some(room) => match room.devices.iter().position(|d| d.name == device_name) {
                Some(idx) => room.devices.remove(idx),
                None => return Err(DeleteError::NotFound),
            },
            None => return Err(DeleteError::NotFound),
        };
        Ok(())
    }
}
