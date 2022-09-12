use std::net::SocketAddr;

pub enum ClientError {
    ConnectionError,
    Unknown
}

pub fn get_socket_status(address: SocketAddr) -> Result<String, ClientError>  {
    // connect, send tcp, disconnect
    Ok("ok_from_socket".to_string())
}

pub fn get_thermo_status(address: SocketAddr) -> Result<String, ClientError> {
    // if not connected to that address -> connect and keep athread for connection, check the thread on request
    Ok("ok_from_thermo".to_string())
}