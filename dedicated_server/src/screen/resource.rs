use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{net::TcpListener, sync::Arc, thread::JoinHandle};
use tokio::sync::{mpsc, Mutex, RwLock};

#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    Kick(String),
    Start,
    Close,
    Join(UserInfo),
    BeKick,
}

pub struct User {
    /// client ip
    pub ip: String,
    /// client name
    pub name: String,
    /// when ui get message,it will call send_message function
    pub send_message: mpsc::Sender<Message>,
}

#[derive(Resource)]
pub struct GameInteraction {
    /// this is server ip
    pub server_ip: String,

    /// this is server code
    pub code: String,
}

impl Default for GameInteraction {
    fn default() -> Self {
        GameInteraction {
            server_ip: "127.0.0.1:3000".to_owned(),
            code: "TEST".to_owned(),
        }
    }
}
#[derive(Resource, Default)]
pub struct Users {
    pub users: Arc<RwLock<Vec<User>>>,
}

#[derive(Resource)]
pub struct GameSigned {
    /// if it is false,server will close
    pub sd: tokio::sync::mpsc::Sender<()>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub name: String,
    pub ip: String,
}
