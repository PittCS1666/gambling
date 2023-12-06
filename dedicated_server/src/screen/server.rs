use std::collections::LinkedList;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
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
        app.add_systems(OnEnter(AppState::ServerRunning), create_center_server)
            // tell thread clear
            .add_systems(
                OnEnter(AppState::GameEnd),
                |mut command: Commands,
                 signed: Res<GameSigned>,
                 mut state: ResMut<NextState<AppState>>| {
                    signed.sd.send(GameSignType::End);
                    thread::sleep(Duration::from_millis(100));
                    command.remove_resource::<GameSigned>();
                    command.remove_resource::<GlobalServerLists>();
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

#[derive(Serialize)]
enum OpInfo{
    /// create lobby success,return server ip
    Success(String),
    /// create lobby failed
    Failed,
    /// query lobby list.first is ip,second is name
    List(Vec<(String,String)>)
}
#[derive(Deserialize,Debug)]
enum UserOp{
    /// the first is lobby name,the second is lobby code
    CreateLobby(String,String),
    QueryLobby,
}

#[derive(Clone)]
struct ServerLists{
    /// this is free ip
    allow_server_list:LinkedList<String>,
    /// the first is server ip,the second is server name,the third is server code
    server_list:Vec<(String,String,String)>,
}
#[derive(Resource,Clone)]
struct GlobalServerLists{
    server_lists:Arc<Mutex<ServerLists>>
}
impl Default for ServerLists{
    fn default() -> Self {
        let mut allow_server_list=LinkedList::new();
        for port in 3001..6000{
            allow_server_list.push_back(format!("0.0.0.0:{port}"));
        }
        
        Self { allow_server_list, server_list: Default::default() }
    }
}

fn create_center_server(mut command: Commands,option: Res<OptionsResult>,){
    let (sd, rx) = std::sync::mpsc::channel();
    let server=Arc::new(Mutex::new(ServerLists::default()));
    let server_lists=server.clone();
    command.insert_resource(GlobalServerLists{server_lists});
    command.insert_resource(GameSigned { sd });
    let number_player=option.num_players;
    let linstener_main=Arc::new(std::net::TcpListener::bind("0.0.0.0:3000").unwrap());
    linstener_main.set_nonblocking(true);
    let stop_signal = Arc::new(AtomicBool::new(false));
    
    let signed=stop_signal.clone();
    let linstener=linstener_main.clone();
    
    thread::spawn(move ||{
        
        loop{
            if signed.load(Ordering::SeqCst) {
                break;
            }
            
            let client = linstener.accept();
            let (client, ipaddr) = match client {
                Ok(s) => s,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    
                    continue
                },
                Err(e) => {
                    panic!("happend error:{e:?}");
                }
            };
            client.set_nonblocking(false);
            let mut client = MessageProto::from(client);
            
            let data=match client.recv() {
                Ok(data) => {
                    let Ok(data) = bincode::deserialize::<UserOp>(&data) else {
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
            match data{
                UserOp::CreateLobby(name,code)=>{
                    let mut server_local=server.lock().unwrap();
                    let Some(ip)=server_local.allow_server_list.pop_back()else{
                        let data=bincode::serialize(&OpInfo::Failed).expect("serde error!");
                        client.send(&data);
                        continue;
                    };
                    let data=bincode::serialize(&OpInfo::Success(ip.split(':').next_back().unwrap().to_string())).expect("serde error!");
                    client.send(&data);
                    server_local.server_list.push((ip.clone(),name,code.clone()));
                    drop(server_local);
                    create_server(server.clone(),ip.clone(),code.clone(),number_player);
                }
                UserOp::QueryLobby=>{
                    let server=server.lock().unwrap();
                    let vec:Vec<(String,String)>=server.server_list.iter().map(|i|(i.0.split(':').next_back().unwrap().to_string(),i.1.clone())).collect();
                    let data=bincode::serialize(&OpInfo::List(vec)).expect("serde error!");
                    client.send(&data);
                }
            }
        }
    });
    thread::spawn(move||{
        // exit server running
        
        rx.recv();
        stop_signal.store(true, Ordering::SeqCst);

    });
}
/// when game on enter lobby,create server
fn create_server(
    server_list:Arc<Mutex<ServerLists>>,
    server_ip:String,
    server_code:String,
    num_players:usize,
) {
    let (sd, rx) = std::sync::mpsc::channel();
    let users =Arc::new(Mutex::new(Vec::new()));
    main_server_loop(server_list,server_ip, server_code, users, num_players, sd,rx);
}

/// main server loop,will create thread:`main_server_loop_part1`,`main_server_loop_part2`
fn main_server_loop(
    server_list:Arc<Mutex<ServerLists>>,
    server_ip: String,
    interaction_code: String,
    users: Arc<Mutex<Vec<User>>>,
    num_players: usize,
    sd:Sender<GameSignType>,
    rx: Receiver<GameSignType>,
) {
    let listener = Arc::new(std::net::TcpListener::bind(server_ip.clone()).unwrap());
    listener.set_nonblocking(true);
    let stop_signal = Arc::new(AtomicBool::new(false));
    let interaction_code = Arc::new(interaction_code);
    let my_local_ip = local_ip().unwrap();
    let users_1 = users.clone();
    let listener_1 = listener.clone();
    let (send_wait,recv_wait)=std::sync::mpsc::channel::<()>();
    println!("You created a server with IP {:?}", my_local_ip);
    let send_2=sd.clone();
    thread::spawn(move||{
        loop{

            match recv_wait.recv_timeout(Duration::from_secs(120)){
                Ok(())=>{}
                Err(e) if e==mpsc::RecvTimeoutError::Timeout=>{
                    println!("wait time over 120s!");
                    break
                }
                Err(e)=>{
                    println!("happend unknown error!");
                    break
                }
            }
        }
        send_2.send(GameSignType::Exit);
    });
    let signed=stop_signal.clone();
    thread::spawn(move || {
        main_server_loop_part1(signed,listener, interaction_code, users, num_players,send_wait,sd);
    });
    thread::spawn(move || {
        main_server_loop_part2(server_list,server_ip,listener_1, users_1, rx,stop_signal);
    });
}

/// this is tcplinstener loop
/// 
/// the function will wait user connect
fn main_server_loop_part1(
    signed:Arc<AtomicBool>,
    listener: Arc<TcpListener>,
    interaction_code: Arc<String>,
    users: Arc<Mutex<Vec<User>>>,
    num_players: usize,
    sender:std::sync::mpsc::Sender<()>,
    game_global_send:Sender<GameSignType>,
) {
    
    loop {
        let game_global_send=game_global_send.clone();
        if signed.load(Ordering::SeqCst) {
            break;
        }
        let client = listener.accept();
        
        let (client, ipaddr) = match client {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => {
                panic!("happend error:{e:?}");
            }
        };
        client.set_nonblocking(false);
        sender.send(());
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
            handle_connection_1(main_client, users, ip2, sender,game_global_send,num_players);
        });
        thread::spawn(move || handle_connection_2(main_client_1, recever, ip3, usersn));

    }
}

/// this is message loop
fn main_server_loop_part2(
    server_list:Arc<Mutex<ServerLists>>,
    server_ip:String,
    listener: Arc<TcpListener>,
    users: Arc<Mutex<Vec<User>>>,
    rx: Receiver<GameSignType>,
    signed:Arc<AtomicBool>,
) {
    let game_message = rx.recv().unwrap();
    
    if game_message == GameSignType::Start {
        // continue wait
        let game_message = rx.recv().unwrap();
    }
    signed.store(true, Ordering::SeqCst);
    let mut users = users.lock().unwrap();
    for user in users.iter() {
        user.send_message.send(Message::Close);
    }
    users.clear();
    drop(users);
    let mut server_list=server_list.lock().unwrap();
    server_list.server_list.retain(|(ip,..)|ip!=&server_ip);
    server_list.allow_server_list.push_back(server_ip);
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
    game_global_send:Sender<GameSignType>,
    num_players: usize,
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
                        
                        
                        for user in users.iter() {
                            let ip = ip.clone();
                            user.send_message.send(Message::Kick(ip));
                        }
                        users.retain(|user| user.ip != ip);
                        if users.len()==0{
                            game_global_send.send(GameSignType::Exit);
                            println!("server is not have person,will close");
                        }
                        
                        drop(users);
                        println!("user exit");
                        return;
                    }
                    // Game Start
                    Message::Start=>{
                        let users=users.lock().unwrap();
                        if users.len()<num_players{
                            continue;
                        }
                        users.iter().for_each(|e| {
                            e.send_message.send(Message::Start);
                        });
                        game_global_send.send(GameSignType::Start);
                    }
                    // Kick User
                    Message::Kick(ip)=>{
                        users.lock().unwrap().iter().for_each(|e| {
                            e.send_message.send(Message::Kick(ip.to_string()));
                        });
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
                
                for user in users.iter() {
                    let ip = ip.clone();
                    user.send_message.send(Message::Kick(ip));
                }
                users.retain(|user| user.ip != ip);
                if users.len()==0{
                    game_global_send.send(GameSignType::Exit);
                    println!("server is not have person,will close");
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
