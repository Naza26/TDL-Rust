use std::io::{BufRead, stdin, Stdin};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::commons::client_state::ClientState;
use crate::server_connection::server_writer::{send_choose_participants_message, send_message, send_quit_message};

pub fn start_reading_input(mut socket: TcpStream, client_state: Arc<Mutex<ClientState>>) {
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
                    }
                }
            }
        }
    });
}