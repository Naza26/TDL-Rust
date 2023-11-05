use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Client {
    pub name: Option<String>,
    pub age: Option<u8>,
    pub country: Option<String>,
    pub socket: Option<TcpStream>
}

// Consider creating a client map composed of an ID and the Client data
pub type ClientMap = Arc<Mutex<HashMap<String, Client>>>;

impl Clone for Client {
    fn clone(&self) -> Self {
        let mut socket = None;
        if let Some(stream) = &self.socket {
            if let Ok(s) = stream.try_clone() {
                socket = Some(s);
            }
        }
        Self {
            socket,
            age: Some(1),
            country: Some("arg".to_string()),
            name: Some("pepe".to_string())
        }
    }
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket: Some(socket),
            age: Some(1),
            country: Some("arg".to_string()),
            name: Some("pepe".to_string())

        }
    }
}