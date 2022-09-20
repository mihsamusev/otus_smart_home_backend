use std::net::SocketAddr;

#[derive(Clone)]
pub struct RoomInfo {
    pub name: RoomName,
    pub devices: Vec<DeviceInfo>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct RoomName(String);

impl TryFrom<String> for RoomName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<RoomName> for String {
    fn from(n: RoomName) -> Self {
        n.0
    }
}

#[derive(Clone)]
pub enum DeviceType {
    TcpSocket,
    UdpThermo,
}

impl TryFrom<String> for DeviceType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "tcp_socket" => Ok(Self::TcpSocket),
            "udp_thermo" => Ok(Self::UdpThermo),
            _ => Err(()),
        }
    }
}

impl From<DeviceType> for String {
    fn from(t: DeviceType) -> Self {
        String::from(match t {
            DeviceType::TcpSocket => "tcp_socket",
            DeviceType::UdpThermo => "udp_thermo",
        })
    }
}

#[derive(Clone)]
pub struct DeviceInfo {
    pub name: DeviceName,
    pub address: SocketAddr,
    pub device_type: DeviceType,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct DeviceName(String);

impl TryFrom<String> for DeviceName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<DeviceName> for String {
    fn from(n: DeviceName) -> Self {
        n.0
    }
}

#[cfg(test)]
impl RoomName {
    pub fn bathroom() -> Self {
        Self("bathroom".to_string())
    }

    pub fn kitchen() -> Self {
        Self("kitchen".to_string())
    }

    pub fn empty() -> Self {
        Self("".to_string())
    }
}

#[cfg(test)]
impl DeviceName {
    pub fn socket() -> Self {
        Self("socket".to_string())
    }

    pub fn thermo() -> Self {
        Self("thermo".to_string())
    }

    pub fn empty() -> Self {
        Self("".to_string())
    }
}
