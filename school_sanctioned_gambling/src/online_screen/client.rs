use std::{sync::{Arc, Mutex}, thread, net::TcpStream};

use super::{sd::MessageProto, AppState, GameInteraction, GameSigned, Message, UserInfo, Users, resource::UserOperater};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::OnlineClient), create_client)
            // tell thread clear
            .add_systems(
                OnEnter(AppState::OnlineEnd),
                |mut command: Commands,
                 
                 mut state: ResMut<NextState<AppState>>| {
 
                    command.remove_resource::<GameSigned>();
                    state.set(AppState::OnlinePlay);
                },
            )
            .add_systems(
                Update,
                (|signed: Res<GameSigned>, mut state: ResMut<NextState<AppState>>| {
                    let next_state = *signed.next_state.lock().unwrap();
                    if next_state != AppState::OnlineClient {
                        state.set(next_state);
                        
                    }
                })
                .run_if(in_state(AppState::OnlineClient)),
            );
    }
}

#[derive(Serialize)]
struct NetConnect {
    name: String,
    code: String,
}

#[derive(Deserialize)]
enum NetReceiver {
    Success(Vec<UserInfo>),
    Failed,
    Full
}

/// when game on enter lobby,create server
fn create_client(
    mut command: Commands,
    interaction: Res<GameInteraction>,
    users: Res<Users>,
) {
    let server_ip = interaction.server_ip.clone();
    let code = interaction.code.clone();
    let name = interaction.name.clone();
    let users = users.users.clone();
    let (sd, mut rv) = std::sync::mpsc::channel::<()>();
    let g_next_state = Arc::new(Mutex::new(AppState::OnlineClient));
    let next_state = g_next_state.clone();
    let (send_message_1, mut recever_message_1) = std::sync::mpsc::channel();
    let (send_message_2, mut recever_message_2) = std::sync::mpsc::channel();
    command.insert_resource(UserOperater{
        send_message:Arc::new(Mutex::new(send_message_1)),
        recv_message:Arc::new(Mutex::new(recever_message_2)),
    });
    command.insert_resource(GameSigned { sd, next_state });

    thread::spawn(move|| {
        let Ok(main_client) = TcpStream::connect(server_ip) else {
            println!("cannot connect server");
            *g_next_state.lock().unwrap() = AppState::OnlineEnd;
            return;
        };
        let main_client=Arc::new(main_client);
        // the first read to use verify
        let mut client = MessageProto::from(main_client.as_ref());
        let data: Vec<u8> = bincode::serialize(&NetConnect { name, code }).unwrap();
        client.send(&data);

        match client.recv() {
            Ok(data) => match bincode::deserialize::<NetReceiver>(&data) {
                Ok(NetReceiver::Success(us)) => {
                    users.lock().unwrap().extend(us);
                    
                }
                Ok(NetReceiver::Failed) | Ok(NetReceiver::Full) => {
                    println!("connect error(server full,or code error)");
                    *g_next_state.lock().unwrap() = AppState::OnlineEnd;
                    return;
                }
                _ => {
                    println!("happend error!");
                    *g_next_state.lock().unwrap() = AppState::OnlineEnd;
                    return;
                }
            },
            Err(e) => {
                println!("{e}");
                *g_next_state.lock().unwrap() = AppState::OnlineEnd;
                return;
            }
        }

        handle_connection(main_client, users.clone(), rv,recever_message_1,send_message_2,g_next_state);
        
    });
}

/// do everything for connect stream
/// 
/// will recv and send
fn handle_connection(
    stream_main: Arc<TcpStream>,
    users: Arc<Mutex<Vec<UserInfo>>>,
    rv: std::sync::mpsc::Receiver<()>,
    recv_message_1:std::sync::mpsc::Receiver<Message>,
    send_message_2:std::sync::mpsc::Sender<Message>,
    state:Arc<Mutex<AppState>>,
) {
    let users2=users.clone();
    let state2=state.clone();
    let main=stream_main.clone();
    thread::spawn(move||{
        let mut stream=MessageProto::from(main.as_ref());
        loop{
            let data=stream.recv();
            match data{
                Ok(data) => {
                        match bincode::deserialize::<Message>(&data){
                            Ok(Message::Close)=>{
                                println!("Server is close!");
                                break
                            }
                            Ok(Message::BeKick)=>{
                                println!("You are be kicked");
                                break
                            }
                            Ok(Message::Kick(ip))=>{
                                users.lock().unwrap().retain(|user|user.ip!=ip);
                            }
                            Ok(Message::Join(user))=>{
                                users.lock().unwrap().push(user);
                            }
                            Ok(Message::Start)=>{
                                *state.lock().unwrap()= AppState::OnlineGamePlaying;
                            }
                            Ok(e)=>{
                                match e{
                                    Message::Raise(_)|Message::Call|Message::Fold|Message::Check|Message::Reset=>{
                                        send_message_2.send(e.clone());
                                    }
                                    _=>{
                                        panic!("happend!");
                                    }
                                }
                            }
                            Err(e)=>{
                                println!("{e}");
                                break
                            }
                        }


                },
                Err(e) => {
                    println!("{e}");
                    break
                }
            }
        }
        *state.lock().unwrap()=AppState::OnlineEnd;
    });
    
    let main=stream_main.clone();
    thread::spawn(move||{
        let mut stream=MessageProto::from(main.as_ref());
        loop{
            let message=recv_message_1.recv();
            let Ok(message)=message else{
                break;
            };
            let encoded: Vec<u8>=bincode::serialize(&message).expect("serde error!");
            stream.send(&encoded);
        }
    });
    
    thread::spawn(move||{
        rv.recv();
        stream_main.shutdown(std::net::Shutdown::Both);
        users2.lock().unwrap().clear();
        *state2.lock().unwrap()=AppState::OnlineEnd;
    });
}