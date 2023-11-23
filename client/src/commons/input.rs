use std::io::{BufRead, stdin, Stdin};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use crate::commons::client_state::ClientState;
use crate::server_connection::server_writer::{send_add_room_message, send_choose_participants_message, send_message, send_quit_message};

pub fn start_reading_input(mut socket: TcpStream, client_state: Arc<Mutex<ClientState>>, tx: Sender<bool>) {
    thread::spawn(move || {
        let stdin: Stdin = stdin();
        for line in stdin.lock().lines() {
            let l = line.unwrap();
            if let Ok(client_state_locked) = client_state.lock() {
                match *client_state_locked {
                    ClientState::Waiting => {},
                    ClientState::Chatting => {
                        if l == "quit".to_string() {
                            send_quit_message(&mut socket);
                        } else {
                            send_message(l, &mut socket);
                        }
                    },
                    ClientState::ChoosingMatch => {
                        send_choose_participants_message(l, &mut socket);
                    },
                    ClientState::AddNewRoom => {
                        if l == "yes".to_string() {
                            send_add_room_message(&mut socket);
                        } else if l == "no".to_string() {
                            println!("Finishing app");
                            tx.send(true).unwrap();
                            return;
                        }
                    }
                }
            }
        }
    });
}