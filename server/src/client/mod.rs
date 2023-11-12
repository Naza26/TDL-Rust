use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ClientInfo {
    pub name: Option<String>,
    pub age: Option<String>,
    pub country: Option<String>
}

#[derive(Debug)]
pub struct Client {
    pub client_info: Option<ClientInfo>,
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
            client_info: None
        }
    }
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket: Some(socket),
            client_info: None

        }
    }
}
