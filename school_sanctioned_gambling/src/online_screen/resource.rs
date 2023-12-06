use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Resource)]
pub struct GameInteraction {
    /// this is you will connect server_ip
    pub server_ip: String,
    /// this is your code
    pub code: String,
    /// this is your name
    pub name: String,
    /// is lobby master
    pub is_master:bool,
    /// connect ip
    pub connect_ip:String,
    /// lobby name
    pub lobby_name:String,
}
#[derive(Resource)]
pub struct ServerList{
    pub list:Vec<(String,String)>,
}
#[derive(Resource)]
pub struct UserOperater{
    /// when ui get message,it will call send_message function
    pub send_message: Arc<Mutex<std::sync::mpsc::Sender<Message>>>,
    /// when recever message
    pub recv_message:Arc<Mutex<std::sync::mpsc::Receiver<Message>>>,
}
impl Default for GameInteraction {
    fn default() -> Self {
        GameInteraction {
            server_ip: "127.0.0.1".to_string(),
            code: "TEST".to_string(),
            name: "XX".to_string(),
            is_master:false,
            connect_ip:String::new(),
            lobby_name:"TEST".to_string(),
        }
    }
}
#[derive(Resource, Default)]
pub struct Users {
    pub users: Arc<Mutex<Vec<UserInfo>>>,
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
    pub sd: std::sync::mpsc::Sender<Option<Message>>,

    /// next state
    pub next_state: Arc<Mutex<AppState>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub name: String,
    pub ip: String,
}
