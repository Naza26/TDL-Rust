use std::collections::HashMap;
const CAPACITY: u8 = 2;

#[derive(Debug)]
pub enum RoomState {
    WAITING,
    STARTED,
    ENDED
}

#[derive(Debug)]
pub struct Rooms {
    pub rooms: HashMap<u8, Room>,
    pub next_id: u8
}

impl Rooms {
    pub fn new() -> Rooms {
        Rooms {
            rooms: HashMap::new(),
            next_id: 0
        }
    }

    pub fn insert_client_to_room(&mut self, client_id: u8) -> (u8, bool) {
        for (id, room) in &mut self.rooms {
            if !room.is_full() {
                let is_full = room.add_client(client_id).unwrap();
                return (id.clone(), is_full);
            }
        }

        let id: u8 = self.next_id;
        let mut new_room = Room::new(id, CAPACITY).unwrap();
        new_room.add_client(client_id).unwrap();
        self.rooms.insert(id, new_room);
        self.next_id += 1;

        (id, false)
    }
}


#[derive(Debug)]
pub struct Room {
    pub id: u8,
    pub participants: Vec<u8>,
    pub capacity: u8,
    pub state: RoomState
}

impl Room {
    pub fn new(
        id: u8,
        capacity: u8,
    ) -> Result<Room, ()> {
        Ok(Room {
            id,
            participants: Vec::new(),
            capacity,
            state: RoomState::WAITING
        })
    }

    // returns if room is full
    pub fn add_client(
        &mut self,
        client_id: u8
    ) -> Result<bool, ()> {
        if self.participants.len() as u8 >= self.capacity {
            return Err(());
        }
        self.participants.push(client_id);

        if self.is_full() {
            self.state = RoomState::STARTED;
            return Ok(true);
        }
        Ok(false)
    }

    //NAZA DESPZ CAMBIA ESTO PARA QUE HAYA SALITAS DE CHAT ADENTRO DE LA SALA
    pub fn get_chat_client(&self, client_id: u8) -> u8 {
        for participant in &self.participants {
            if participant.clone() != client_id {
                return participant.clone();
            }
        }
        return 255;
    }

    pub fn get_rest_of_participants(&self, client_id: u8) -> Vec<u8> {
        let mut participants = Vec::new();
        for participant in &self.participants {
            if participant.clone() != client_id {
                participants.push(participant.clone());
            }
        }
        return participants;
    }

    pub fn should_finish_chat(&self) -> bool {
        true
    }

    pub fn is_full(&self) -> bool {
        self.participants.len() as u8 == self.capacity
    }
}