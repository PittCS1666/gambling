use bevy::prelude::*;

pub enum Message {
    Kick,
    Start,
    Close,
}

pub struct User {
    /// client ip
    pub ip: String,
    /// client name
    pub name: String,
    /// when ui get message,it will call send_message function
    pub send_message: Box<dyn Fn(Message) + Send + Sync>,
}

#[derive(Resource, Default)]
pub struct Interaction {
    /// this is server ip
    pub server_ip: String,
    
    /// this is server code
    pub code:String,
}

#[derive(Resource,Default)]
pub struct Users{
    pub users: Vec<User>,
}
