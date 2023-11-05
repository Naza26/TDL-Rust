use std::collections::hash_map::RandomState;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{mpsc, MutexGuard};
use std::time::Duration;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    thread::JoinHandle,
};
use crate::client::ClientMap;

pub struct Server {
    pub name: String,
    pub port: String,
    pub address: String,
    pub sender: Arc<Mutex<mpsc::Sender<()>>>,
    pub receiver: Arc<Mutex<mpsc::Receiver<()>>>,
    pub receiver_thread: Option<JoinHandle<()>>,
    pub registered_clients: ClientMap,
    pub socket: Option<TcpStream>,
}

/// Implement Drop trait for server. This way, when the server
/// is dropped, the server thread will be closed
impl Drop for Server {
    fn drop(&mut self) {
        println!("Droping server {}...", self.port);
        if let Some(thread) = self.receiver_thread.take() {
            if thread.join().is_err() {
                println!("Error while closing receiver thread from {}", self.name);
            }
        }
    }
}

impl Server {

    pub fn new(
        address: &str,
        clients: &ClientMap,
    ) -> Result<Server, ()> {
        let (sender, receiver) = mpsc::channel();
        let name = format!("server_{}", address);
        let mut registered_clients = Arc::clone(clients);

        Ok(Server {
            name,
            port: address.to_owned(),
            address: address.to_string(),
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
            receiver_thread: None,
            registered_clients,
            socket: None,
        })
    }

    /// This function will execute the reading thread of the receiver end point
    /// of the server channel. It will handle each server signal.
    pub fn read_from_recv(&mut self) {
        let receiver = Arc::clone(&self.receiver);
        let registered_clients = Arc::clone(&self.registered_clients);
        self.receiver_thread = Some(thread::spawn(move || loop {
            let receiver = receiver.lock().unwrap();
            if let Ok(msg) = receiver.recv() {
                match msg {
                    _ => println!("Only server signals are accepted!"),
                }
            }
        }));
    }

}