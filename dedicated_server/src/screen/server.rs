use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::screen::resource::GameSignType;
use crate::{options::components::OptionsResult, screen::MessageProto};

use super::{AppState, GameInteraction, GameSigned, Message, User, UserInfo, Users};
use bevy::prelude::*;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ServerRunning), create_server)
            // tell thread clear
            .add_systems(
                OnEnter(AppState::GameEnd),
                |mut command: Commands,
                 signed: Res<GameSigned>,
                 mut state: ResMut<NextState<AppState>>| {
                    signed.sd.send(GameSignType::End);
                    thread::sleep(Duration::from_millis(100));
                    command.remove_resource::<GameSigned>();
                    state.set(AppState::StartScreen)
                },
            );
    }
}

#[derive(Deserialize)]
struct NetConnect {
    name: String,
    code: String,
}

#[derive(Serialize)]
enum NetReceiver {
    Success(Vec<UserInfo>),
    Failed,
    Full,
}
/// when game on enter lobby,create server
fn create_server(
    mut command: Commands,
    interaction: Res<GameInteraction>,
    users: Res<Users>,
    option: Res<OptionsResult>,
) {
    let server_ip = interaction.server_ip.clone();
    let interaction_code = interaction.code.clone();
    let num_players = option.num_players;
    let (sd, rx) = std::sync::mpsc::channel();
    let sd_new=sd.clone();
    command.insert_resource(GameSigned { sd });
    let users = users.users.clone();
    main_server_loop(server_ip, interaction_code, users, num_players, sd_new,rx);
}

/// main server loop,will create thread:`main_server_loop_part1`,`main_server_loop_part2`
fn main_server_loop(
    server_ip: String,
    interaction_code: String,
    users: Arc<Mutex<Vec<User>>>,
    num_players: usize,
    sd:Sender<GameSignType>,
    rx: Receiver<GameSignType>,
) {
    let listener = Arc::new(std::net::TcpListener::bind(server_ip).unwrap());
    let interaction_code = Arc::new(interaction_code);
    let my_local_ip = local_ip().unwrap();
    let users_1 = users.clone();
    let listener_1 = listener.clone();
    let (send_wait,recv_wait)=std::sync::mpsc::channel::<()>();
    println!("You created a server with IP {:?}", my_local_ip);
    thread::spawn(move||{
        loop{
            match recv_wait.recv_timeout(Duration::from_secs(120)){
                _=>{}
                Err(e) if e==mpsc::RecvTimeoutError::Timeout=>{
                    println!("wait time over 120s!");
                    break
                }
                Err(e)=>{
                    println!("happend unknown error!");
                    break
                }
            }
            sd.send(GameSignType::Exit);
        }
    });
    thread::spawn(move || {
        main_server_loop_part1(listener, interaction_code, users, num_players,send_wait);
    });
    thread::spawn(move || {
        main_server_loop_part2(listener_1, users_1, rx);
    });
}

/// this is tcplinstener loop
/// 
/// the function will wait user connect
fn main_server_loop_part1(
    listener: Arc<TcpListener>,
    interaction_code: Arc<String>,
    users: Arc<Mutex<Vec<User>>>,
    num_players: usize,
    sender:std::sync::mpsc::Sender<()>
) {
    
    loop {
        let client = listener.accept();
        sender.send(());
        let (client, ipaddr) = match client {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(e) => {
                panic!("happend error:{e:?}");
            }
        };
        let main_client = Arc::new(client);

        let mut client = MessageProto::from(main_client.as_ref());
        let ip = ipaddr.to_string();
        let interaction_code = interaction_code.clone();
        let users = users.clone();

        let NetConnect { name, code } = match client.recv() {
            Ok(data) => {
                let Ok(data) = bincode::deserialize::<NetConnect>(&data) else {
                    println!("data error!");
                    continue;
                };
                data
            }
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        if code != *interaction_code {
            let encoded: Vec<u8> = bincode::serialize(&NetReceiver::Failed).expect("serde error!");
            client.send(&encoded);
            continue;
        }
        let mut users2 = users.lock().unwrap();
        if users2.len() >= num_players {
            let encoded: Vec<u8> = bincode::serialize(&NetReceiver::Full).expect("serde error!");
            client.send(&encoded);
        }
        let (send_message, recever) = mpsc::channel::<Message>();
        let (sender, recv_message) = mpsc::channel::<Message>();
        // for every send
        for user in users2.iter() {
            let (ip, name) = (ip.to_string(), name.to_string());
            user.send_message.send(Message::Join(UserInfo { name, ip }));
        }
        let ip2 = ip.clone();
        users2.push(super::User {
            ip,
            name,
            send_message,
            recv_message,
        });
        let users_list = users2
            .iter()
            .map(|User { ip, name, .. }| {
                let (ip, name) = (ip.to_string(), name.to_string());
                UserInfo { ip, name }
            })
            .collect::<Vec<UserInfo>>();
        drop(users2);

        // send ok
        let encoded: Vec<u8> =
            bincode::serialize(&NetReceiver::Success(users_list)).expect("serde error!");

        client.send(&encoded);
        let main_client_1 = main_client.clone();
        let ip3 = ip2.clone();
        let usersn = users.clone();
        thread::spawn(move || {
            handle_connection_1(main_client, users, ip2, sender);
        });
        thread::spawn(move || handle_connection_2(main_client_1, recever, ip3, usersn));

    }
}

/// this is message loop
fn main_server_loop_part2(
    listener: Arc<TcpListener>,
    users: Arc<Mutex<Vec<User>>>,
    rx: Receiver<GameSignType>,
) {
    let game_message = rx.recv().unwrap();
    listener.set_nonblocking(true).expect("setting error!");
    if game_message == GameSignType::Start {
        // continue wait
        let game_message = rx.recv().unwrap();
    }
    let mut users = users.lock().unwrap();
    for user in users.iter() {
        user.send_message.send(Message::Close);
    }
    users.clear();
    drop(users);
}

/// loop for `TcpStream` await
/// 
/// wait client send message
/// 
/// if message is `Message::Close`,retain user,and send all user this message
/// 
/// if message is oprate,tell all user and tell server how to do
/// 
/// else message will send signed check
fn handle_connection_1(
    stream: Arc<TcpStream>,
    users: Arc<Mutex<Vec<User>>>,
    ip: String,
    sender: mpsc::Sender<Message>,
) {
    let mut stream = MessageProto::from(stream.as_ref());
    loop {
        let result = stream.recv();
        match result {
            Ok(data) => {
                let message: Message = bincode::deserialize(&data).expect("serde error!");
                match message {
                    // Game Exit
                    Message::Close => {
                        let mut users = users.lock().unwrap();
                        users.retain(|user| user.ip != ip);
                        for user in users.iter() {
                            let ip = ip.clone();
                            user.send_message.send(Message::Kick(ip));
                        }
                        drop(users);
                        println!("user exit");
                        return;
                    }
                    // oprate
                    Message::Raise(_) | Message::Call | Message::Fold | Message::Check => {
                        let users = users.lock().unwrap();
                        for user in users.iter() {
                            if user.ip != ip {
                                user.send_message.send(message.clone());
                            }
                        }
                        drop(users);
                    }
                    else_message => {
                        sender.send(else_message);
                    }
                }
            }
            Err(_e) => {
                let mut users = users.lock().unwrap();
                users.retain(|user| user.ip != ip);
                for user in users.iter() {
                    let ip = ip.clone();
                    user.send_message.send(Message::Kick(ip));
                }
                drop(users);
                println!("user exit");
                return;
            }
        }
    }
}
/// do signed check,
/// 
/// loop check `recever.recv()`
/// 
/// if message is kick,or message is close,the function will close,and connect will shutdown
/// 
/// if recever is close,the function will close
fn handle_connection_2(
    main_stream: Arc<TcpStream>,
    recever: mpsc::Receiver<Message>,
    ip: String,
    users: Arc<Mutex<Vec<User>>>,
) {
    let mut stream = MessageProto::from(main_stream.as_ref());
    loop {
        let message = recever.recv();
        let Ok(message) = message else { break };
        match message {
            Message::Kick(this_ip) => {
                if this_ip == ip {
                    let encoded: Vec<u8> =
                        bincode::serialize(&Message::BeKick).expect("serde error!");
                    stream.send(&encoded);
                    break;
                } else {
                    let encoded: Vec<u8> =
                        bincode::serialize(&Message::Kick(this_ip)).expect("serde error!");
                    stream.send(&encoded);
                }
            }
            Message::Close => break,
            tell => {
                let encode: Vec<u8> = bincode::serialize(&tell).expect("serde error!");
                stream.send(&encode);
            }
        }
    }
    // delete user
    let mut users = users.lock().unwrap();
    users.retain(|user| user.ip != ip);

    main_stream.shutdown(std::net::Shutdown::Both);
}
