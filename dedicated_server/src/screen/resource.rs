use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    Kick(String),
    Start,
    Close,
    Join(UserInfo),
    BeKick,
    Over,
    /// player will action
    Action,

    /// player action type
    Raise(u64),
    Call,
    Fold,
    Check,

    Reset,
}

pub struct User {
    /// client ip
    pub ip: String,
    /// client name
    pub name: String,
    /// when ui get message,it will call send_message function
    pub send_message: std::sync::mpsc::Sender<Message>,
    /// when recever message
    pub recv_message: std::sync::mpsc::Receiver<Message>,
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
    pub users: Arc<Mutex<Vec<User>>>,
}
#[derive(PartialEq, Eq)]
pub enum GameSignType {
    Start,
    End,
    Exit,
}

#[derive(Resource)]
pub struct GameSigned {
    /// if it is false,server will close
    pub sd: std::sync::mpsc::Sender<GameSignType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub name: String,
    pub ip: String,
}

pub enum UiInfoString {
    Info(String),
    Warn(String),
    Error(String),
    None,
}

#[derive(Resource)]
pub struct UiInfo {
    pub info: UiInfoString,
    timer: Timer,
    timer_wait:Timer,
}

impl Default for UiInfo {
    fn default() -> Self {
        UiInfo {
            info: UiInfoString::None,
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            timer_wait:Timer::from_seconds(120.0, TimerMode::Repeating),
        }
    }
}
impl UiInfo {
    pub fn set(&mut self, info: UiInfoString) {
        self.info = info;
        self.timer.reset();
    }
}

pub fn reset_infostring(time: Res<Time>, mut uiinfo: ResMut<UiInfo>) {
    if uiinfo.timer.tick(time.delta()).just_finished() {
        uiinfo.info = UiInfoString::None;
    }
}


// #[derive(Serialize, Deserialize, Clone, Default)]
// enum GameOperation {
//     #[default]
//     Check,
//     Call,
//     Raise(u64),
//     Fold,
// }

// use crate::game::cards::Card;
// use crate::game::components::Player;
// #[derive(Default)]
// pub struct PlayerInfos{
//     pub cards:Vec<Card>,
//     pub players:Vec<Player>,
//     pub game_pos:u64,
//     pub money:u64,
// }

// impl PlayerInfos{
//     pub fn reset(&mut self,size:usize){
//         self.game_pos=0;
//         self.money=0;
//         self.players.resize(size,Player::default());
//     }
// }

// #[derive(Resource,Default)]
// pub struct PubPlayerInfos{
//     pub player_infos:Arc<RwLock<PlayerInfos>>,
// }

// impl PubPlayerInfos{
//     pub fn reset(&mut self,size:usize){
//         self.player_infos.blocking_write().reset(size);
//     }
// }
