use std::sync::Arc;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};

use crate::AppState;

#[derive(Resource)]
pub struct GameInteraction {
    /// this is you will connect server_ip
    pub server_ip: String,
    /// this is your code
    pub code: String,
    /// this is your name
    pub name: String,
}

#[derive(Resource)]
pub struct UserOperater{
    /// when ui get message,it will call send_message function
    pub send_message: mpsc::Sender<Message>,
    /// when recever message
    pub recv_message:mpsc::Receiver<Message>,
}
impl Default for GameInteraction {
    fn default() -> Self {
        GameInteraction {
            server_ip: "127.0.0.1:3000".to_string(),
            code: "TEST".to_string(),
            name: "XX".to_string(),
        }
    }
}
#[derive(Resource, Default)]
pub struct Users {
    pub users: Arc<RwLock<Vec<UserInfo>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    Kick(String),
    Start,
    Close,
    Join(UserInfo),
    BeKick,

    /// player will action
    Action,

    /// player action type
    Raise(u64),
    Call,
    Fold,
    Check,
        
    Reset,
}

#[derive(Resource)]
pub struct GameSigned {
    /// if it is false,server will close
    pub sd: mpsc::Sender<()>,

    /// next state
    pub next_state: Arc<RwLock<AppState>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub name: String,
    pub ip: String,
}
