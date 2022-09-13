use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::str;

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("IoError: {0}")]
    IoError(String),
    #[error("ConnectionError: {0}")]
    ConnectionError(String),
    #[error("UnknownError: {0}")]
    Unknown(String),
}

pub fn get_socket_status(address: SocketAddr) -> Result<String, ClientError> {
    // connect, send tcp, disconnect
    let mut stream =
        TcpStream::connect(address).map_err(|e| ClientError::ConnectionError(e.to_string()))?;

    // write a command to get status
    stream
        .write_all("GET".as_bytes())
        .map_err(|e| ClientError::IoError(e.to_string()))?;

    // unpack the result
    let mut buf: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(&stream);
    reader
        .read_until(b'\n', &mut buf)
        .map_err(|e| ClientError::IoError(e.to_string()))?;

    let response = str::from_utf8(&buf).unwrap_or_default();
    Ok(response.to_string())
}

pub fn get_thermo_status(_address: SocketAddr) -> Result<String, ClientError> {
    // if not connected to that address -> connect and keep athread for connection, check the thread on request
    Err(ClientError::Unknown(
        "UDP Thermomemter is not supported".into(),
    ))
}
