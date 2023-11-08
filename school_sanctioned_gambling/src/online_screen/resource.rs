use bevy::prelude::*;


pub struct User {
    /// client ip
    pub ip: String,
    /// client name
    pub name: String,
}

#[derive(Resource, Default)]
pub struct Interaction {
    /// this is you will connect server_ip
    pub server_ip: String,
    /// this is your code
    pub code:String,
    /// this is your name
    pub name:String,
}

#[derive(Resource,Default)]
pub struct Users{
    pub users: Vec<User>,
}