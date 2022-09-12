use clap::Parser;
use rand;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(thiserror::Error, Debug)]
pub enum DeviceError {
    #[error("SmartSocket error: {0}")]
    SocketError(String),
}

#[derive(Serialize, Deserialize)]
pub struct SmartSocket {
    enabled: bool,
    power: f32,
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartSocket {
    pub fn new() -> Self {
        Self {
            enabled: false,
            power: 0.0,
        }
    }

    pub fn update(&mut self) {
        if self.enabled {
            self.power = rand::random::<f32>();
        }
    }

    pub fn get_power_usage(&self) -> f32 {
        self.power
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_on(&mut self) {
        self.enabled = true
    }

    pub fn set_off(&mut self) {
        self.enabled = false;
        self.power = 0.0
    }

    pub fn get_status(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn status(&self) -> Result<String, DeviceError> {
        let state = if self.enabled { "on" } else { "off" };
        Ok(format!(
            "SmartSocket is {} and consumes 0 W {}",
            state,
            self.get_power_usage()
        ))
    }

    fn execute(&mut self, query: &str) -> Result<String, DeviceError> {
        self.update();
        match query {
            "SET1" => {
                self.set_on();
                Ok(self.get_status())
            }
            "SET0" => {
                self.set_off();
                Ok(self.get_status())
            }
            "GET" => Ok(self.get_status()),
            _ => Err(DeviceError::SocketError(format!(
                "Unrecognized command {}",
                query
            ))),
        }
    }
}

// socket server maps Tcp requests from multiple users to
// socket device commandsto simulate the device
//
pub struct SmartSocketServer {
    pub device: Arc<Mutex<SmartSocket>>,
    pub listener: TcpListener,
}

impl SmartSocketServer {
    pub fn new(listener: TcpListener) -> Self {
        let device = Arc::new(Mutex::new(SmartSocket::new()));
        Self { device, listener }
    }

    pub fn listen(&mut self) {
        println!(
            "[SmartSocket] listening on {}",
            &self.listener.local_addr().expect("Couldnt get local addr")
        );
        for stream in self.listener.incoming() {
            match stream {
                Err(e) => {
                    eprintln!("fail: {}", e)
                }
                Ok(stream) => {
                    let client_addr = stream.peer_addr().unwrap();
                    let socket_ref = self.device.clone();
                    thread::spawn(move || {
                        handle_smart_device(stream, socket_ref)
                            .unwrap_or_else(|_| eprintln!("{} disconnected", client_addr));
                    });
                }
            }
        }
    }
}

fn handle_smart_device(
    mut stream: TcpStream,
    device: Arc<Mutex<SmartSocket>>,
) -> Result<(), io::Error> {
    let client_addr = &stream.peer_addr()?;
    println!("[SmartDevice] {} connected", client_addr);

    loop {
        let mut buf: [u8; 10] = [0; 10];
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            println!("[SmartDevice] {} disconnected", client_addr);
            return Ok(());
        }

        let mut device = device.lock().unwrap();
        let command = std::str::from_utf8(&buf)
            .unwrap_or_default()
            .trim_matches(char::from(0))
            .trim();
        let mut response = match device.execute(command) {
            Ok(ok_resp) => ok_resp,
            Err(err_resp) => format!("{:?}", err_resp),
        };

        println!("[SmartDevice] {}: {}", client_addr, &response);

        // send response back to the strem
        response.push('\n');
        stream.write_all(response.as_bytes())?;
    }
}

/// Simple TCP socket device emulator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// IP:PORT
    #[clap(short, long, value_parser, default_value = "127.0.0.1:8080")]
    address: String,
}
fn main() {
    let args = Args::parse();
    let listener = TcpListener::bind(args.address).expect("Could not bind to given address");
    let mut socket = SmartSocketServer::new(listener);
    socket.listen();
}
