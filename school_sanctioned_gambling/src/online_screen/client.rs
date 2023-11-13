use std::sync::Arc;

use super::{sd::S2D, AppState, GameInteraction, GameSigned, Message, UserInfo, Users};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    runtime::Runtime,
    select,
    sync::{mpsc, Mutex, RwLock},
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TokioRuntime>()
            .add_systems(OnEnter(AppState::OnlineClient), create_client)
            // tell thread clear
            .add_systems(
                OnEnter(AppState::OnlineEnd),
                |mut command: Commands,
                 signed: Res<GameSigned>,
                 mut state: ResMut<NextState<AppState>>| {
                    signed.sd.blocking_send(());
                    command.remove_resource::<GameSigned>();
                    state.set(AppState::OnlinePlay);
                },
            )
            .add_systems(
                Update,
                (|signed: Res<GameSigned>, mut state: ResMut<NextState<AppState>>| {
                    let next_state = *signed.next_state.blocking_read();
                    if next_state != AppState::OnlineServer {
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
}
#[derive(Resource)]
struct TokioRuntime {
    rt: Runtime,
}

impl Default for TokioRuntime {
    fn default() -> Self {
        Self {
            rt: Runtime::new().unwrap(),
        }
    }
}

/// when game on enter lobby,create server
fn create_client(
    mut command: Commands,
    interaction: Res<GameInteraction>,
    users: Res<Users>,
    tokio_runtime: Res<TokioRuntime>,
) {
    let server_ip = interaction.server_ip.clone();
    let code = interaction.code.clone();
    let name = interaction.name.clone();
    let users = users.users.clone();
    let (sd, rv) = mpsc::channel::<()>(1);
    let g_next_state = Arc::new(RwLock::new(AppState::OnlineClient));
    let next_state = g_next_state.clone();
    command.insert_resource(GameSigned { sd, next_state });

    tokio_runtime.rt.spawn(async move {
        let Ok(mut client) = TcpStream::connect(server_ip).await else {
            println!("cannot connect server");
            *g_next_state.write().await = AppState::OnlineEnd;
            return;
        };
        // the first read to use verify
        let mut client = S2D::from(client);
        let mut data: Vec<u8> = bincode::serialize(&NetConnect { name, code }).unwrap();
        client.send(&data).await;

        match client.recv().await {
            Ok(data) => match bincode::deserialize::<NetReceiver>(&data) {
                Ok(NetReceiver::Success(us)) => {
                    users.write().await.extend(us);
                }
                Ok(NetReceiver::Failed) => {
                    println!("code error!");
                    *g_next_state.write().await = AppState::OnlineEnd;
                    return;
                }
                _ => {
                    println!("happend error!");
                    *g_next_state.write().await = AppState::OnlineEnd;
                    return;
                }
            },
            Err(e) => {
                println!("{e}");
                *g_next_state.write().await = AppState::OnlineEnd;
                return;
            }
        }

        *g_next_state.write().await = handle_connection(client, users.clone(), rv).await;
        users.write().await.clear();
    });
}

/// do everything for handle
async fn handle_connection(
    mut stream: S2D<TcpStream>,
    users: Arc<RwLock<Vec<UserInfo>>>,
    mut rv: mpsc::Receiver<()>,
) -> AppState {
    loop {
        select! {
            data=stream.recv()=>{
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
                                    users.write().await.retain(|user|user.ip!=ip);
                                }
                                Ok(Message::Join(user))=>{
                                    users.write().await.push(user);
                                }
                                Ok(Message::Start)=>{
                                    return AppState::OnlineGamePlaying;
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
        _=rv.recv()=>{
                break
        }
        }
    }
    AppState::OnlineEnd
}
