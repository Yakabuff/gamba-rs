mod cards;
pub mod channel_hub;
pub mod poker_state;

use std::collections::HashMap;

use rand::Rng;
use tokio::sync::mpsc::{Receiver, Sender};

use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
///
/// Init task distribution thread
/// Init channel hub thread
///
/// Init channel hub mpsc rx, tx channel to be cloned
/// Init task distributor mpsc channel. Single rx, clone tx on demand
/// Init mpsc channel (rx, tx) for tokio connection tasks to request broadcast mpmc channel from channel hub
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);
    tx.send("sending from first handle").await.unwrap();

    let handle = task::spawn(async move {
        println!("Hello from a spawned async task!");
        while let Some(message) = rx.recv().await {
            println!("GOT = {}", message);
        }
    });
    handle.await.unwrap();
}

/// Spawns room distributor task which listens to a mpsc channel
/// If join message, it will try to find room id
/// If create message, it will spawn a room task. Create
/// If game message, find the corresponding tx channel of that room and send message
fn init_room_distributor(mut rx: Receiver<Message>) {
    task::spawn(async move {
        println!("Hello from a spawned async task!");
        let mut rooms = HashMap::<String, Sender<Message>>::new();
        while let Some(message) = rx.recv().await {
            match message {
                Message::UserJoined {
                    username,
                    uuid,
                    room_id,
                    tx,
                } => {
                    println!("User {} joined {}", room_id, username);
                    let sender = rooms.get(&room_id).unwrap();
                    let join_notification = Message::UserJoined {
                        username: username.clone(),
                        uuid,
                        room_id: room_id.clone(),
                        tx,
                    };
                    if let Err(e) = sender.send(join_notification).await {
                        println!("Failed to send join notification: {}", e);
                    }
                }
                Message::UserCreateRoom {
                    username,
                    uuid,
                    tx,
                    game_type,
                } => {
                    print!("Created room for {}", username);
                    // Generate random 4 letter room id
                    let room_id = generate_random_room_id();
                    // Create channel for room
                    let (tx, rx) = mpsc::channel::<Message>(10);
                    rooms.insert(room_id.clone(), tx.clone());
                    // Spawn task for room
                    create_room(game_type.clone(), rx, room_id);
                    let create_message = Message::UserCreateRoom {
                        username: username.clone(),
                        uuid: uuid.clone(),
                        tx: tx.clone(),
                        game_type: game_type.clone(),
                    };
                    // Connect user to new room
                    join_room(tx.clone(), create_message).await;
                }
                Message::UserLeft(_) => {
                    println!("User left room: ")
                }
            }
        }
    });
}

/// Spawn task
/// Init state machine for room
/// Create channel
/// Listen on rx for user input
///
fn create_room(game_type: GameType, mut rx: Receiver<Message>, room_id: String) {
    let _handle = task::spawn(async move {
        println!("Spawning {:?} room {}", game_type, room_id);
        // Init state machine
        while let Some(message) = rx.recv().await {
            // println!("GOT = {:?}", message.);
        }
    });
}

async fn join_room(tx: Sender<Message>, message: Message) {
    match tx.send(message).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
#[derive(Debug)]
enum Message {
    UserJoined {
        username: String,
        uuid: String,
        room_id: String,
        tx: Sender<Message>,
    },
    UserCreateRoom {
        username: String,
        uuid: String,
        tx: Sender<Message>,
        game_type: GameType,
    },
    UserLeft(u64),
}

#[derive(Debug, Clone, Copy)]
enum GameType {
    POKER,
}

fn generate_random_room_id() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const ID_LENGTH: usize = 4;

    let mut rng = rand::thread_rng();

    let room_id: String = (0..ID_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    room_id
}
