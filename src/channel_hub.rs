use std::collections::HashMap;

use tokio::{
    sync::{broadcast::Sender, mpsc::Receiver},
    task,
};

use crate::Message;

struct ChannelHub {
    rooms: HashMap<String, Sender<Message>>,
    users: HashMap<String, Sender<Message>>,
    rx_worker: Receiver<Message>,
    rx_sse: Receiver<Message>,
}

impl ChannelHub {
    fn new(rx_worker: Receiver<Message>, rx_sse: Receiver<Message>) -> Self {
        let rooms = HashMap::<String, Sender<Message>>::new();
        let users = HashMap::<String, Sender<Message>>::new();
        ChannelHub {
            rooms,
            users,
            rx_worker,
            rx_sse,
        }
    }

    fn spawn(self) {
        task::spawn(async move {
            let mut rooms = self.rooms;
            let mut users = self.users;
            let mut rx_worker = self.rx_worker;
            while let Some(message) = rx_worker.recv().await {
                ChannelHub::handler(&mut rooms, &mut users, message)
            }
        });
    }

    /// Channel hub receives msgs from game workers routes them to the correct channels to be consumed
    /// by sse.  
    ///
    /// Maintain hashmap of room_id -> channel and user_id -> channel
    /// room channels are for spectators and public events
    ///
    /// User channels are for private events like dealing a hand of cards
    ///
    /// Channels are broadcast channels to be consumed by multiple connections at the same time (mpmc)
    fn handler(
        rooms: &mut HashMap<String, Sender<Message>>,
        users: &mut HashMap<String, Sender<Message>>,
        message: Message,
    ) {
        match message {
            Message::UserJoined {
                username,
                uuid,
                room_id,
                tx,
            } => {
                let public_channel = rooms.get(&room_id);
                match public_channel {
                    Some(x) => {
                        let join_message = Message::UserJoined {
                            username: username.clone(),
                            uuid: uuid.clone(),
                            tx: tx.clone(),
                            room_id: room_id.clone(),
                        };
                        let res = x.send(join_message);
                        match res {
                            Ok(_) => todo!(),
                            Err(_) => todo!(),
                        }
                    }
                    None => todo!(),
                }
            }
            Message::UserCreateRoom {
                username,
                uuid,
                tx,
                game_type,
            } => todo!(),
            Message::UserLeft(_) => todo!(),
        }
    }
}
