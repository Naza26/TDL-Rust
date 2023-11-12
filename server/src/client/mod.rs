use std::collections::HashMap;
use std::net::TcpStream;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ClientInfo {
    pub name: Option<String>,
    pub age: Option<String>,
    pub country: Option<String>
}


#[derive(Debug)]
pub struct Clients {
    pub clients: HashMap<u8, Client>,
    pub next_id: u8
}

impl Clients {
    pub fn new() -> Clients {
        Clients {
            clients: HashMap::new(),
            next_id: 0
        }
    }

    pub fn add_client(&mut self, client_id: u8, client_info: ClientInfo, socket: TcpStream) {
        let client: Client = Client::new(client_id, client_info, socket);
        self.clients.insert(client_id, client);
        println!("client added, clients: {:?}", &self.clients);
    }
}

#[derive(Debug)]
pub struct Client {
    pub client_info: Option<ClientInfo>,
    pub client_id: u8,
    pub socket: Option<TcpStream>
}

// Consider creating a client map composed of an ID and the Client data
//pub type ClientMap = Arc<Mutex<HashMap<String, Client>>>;

impl Clone for Client {
    fn clone(&self) -> Self {
        let mut socket = None;
        if let Some(stream) = &self.socket {
            if let Ok(s) = stream.try_clone() {
                socket = Some(s);
            }
        }
        Self {
            client_id: self.client_id,
            socket,
            client_info: None
        }
    }
}

impl Client {
    pub fn new(client_id: u8,
               client_info: ClientInfo,
               socket: TcpStream) -> Self {
        Self {
            client_id,
            socket: Some(socket),
            client_info: Some(client_info)
        }
    }
}
