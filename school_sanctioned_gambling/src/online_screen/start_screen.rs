use std::net::TcpStream;

use super::{AppState, GameInteraction, resource::ServerList, sd::MessageProto};
use bevy::prelude::*;
use bevy_egui::{
    egui::{RichText, Vec2},
    *,
};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
enum OpInfo{
    /// create lobby success,return server ip
    Success(String),
    /// create lobby failed
    Failed,
    /// query lobby list.first is ip,second is name
    List(Vec<(String,String)>)
}
#[derive(Serialize)]
enum UserOp{
    /// the first is lobby name,the second is lobby code
    CreateLobby(String,String),
    QueryLobby,
}


/// init wait lobby,can enter server.
pub(super) fn start_screen_update(
    mut command: Commands,
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    mut interaction: ResMut<GameInteraction>,
    server_list:Option<Res<ServerList>>,
) {
    let GameInteraction {
        ref mut server_ip,
        ref mut name,
        ref mut code,
        ref mut is_master,
        ref mut connect_ip,
        ref mut lobby_name,
    } = interaction.as_mut();

    egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
        });
    });

    egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
        egui::Grid::new("Panel")
            .num_columns(2)
            .spacing([10.0, 10.0])
            .striped(false)
            .show(ui, |ui| {
                if ui.button(RichText::new("Connect").size(16.0)).clicked() {
                    let Ok(main_client)=TcpStream::connect(format!("{server_ip}:3000")) else{
                        println!("cannot connect server!");
                        return;
                    };
                    let mut client=MessageProto::from(main_client);
                    let data=bincode::serialize(&UserOp::QueryLobby).unwrap();
                    client.send(&data);
                    match client.recv(){
                        Ok(data)=>{
                            let Ok(list)=bincode::deserialize::<OpInfo>(&data) else{
                                println!("bincode serde error!");
                                return
                            };
                            let OpInfo::List(list)=list else{
                                return
                            };
                            if server_list.is_some(){
                                command.remove_resource::<ServerList>();
                            }
                            command.insert_resource(ServerList{list});
                        }
                        Err(e)=>{
                            println!("{e}");
                            return
                        }
                    }
                    
                }
                ui.add(egui::TextEdit::singleline(server_ip).min_size(Vec2::new(128.0, 16.0)))
                    .on_hover_text(
                        "if you are client please write you connect ip and port,else write bind ip",
                    );
                ui.end_row();
                if ui.button(RichText::new("Create").size(16.0)).clicked() {
                    let Ok(main_client)=TcpStream::connect(format!("{server_ip}:3000")) else{
                        println!("cannot connect server!");
                        return;
                    };
                    let mut client=MessageProto::from(main_client);
                    let data=bincode::serialize(&UserOp::CreateLobby(lobby_name.to_string(), code.to_string())).unwrap();
                    client.send(&data);
                    match client.recv(){
                        Ok(data)=>{
                            let Ok(data)=bincode::deserialize::<OpInfo>(&data) else{
                                println!("bincode serde error!");
                                return
                            };
                            let OpInfo::Success(ip)=data else{
                                return
                            };
                            *connect_ip=format!("{server_ip}:{ip}");
                            *is_master=true;
                            state.set(AppState::OnlineClient);
                            
                        }
                        Err(e)=>{
                            println!("{e}");
                            
                            return
                        }
                    }
                }
                ui.end_row();
                ui.label(RichText::new("Lobby name").size(16.0));
                ui.add(egui::TextEdit::singleline(lobby_name).min_size(Vec2::new(128.0, 16.0)))
                    .on_hover_text("Please write your lobby name,if you are not master,do not write it.");
                ui.end_row();
                ui.label(RichText::new("Name").size(16.0));
                ui.add(egui::TextEdit::singleline(name).min_size(Vec2::new(128.0, 16.0)))
                    .on_hover_text("Please write your name");
                ui.end_row();

                ui.label(RichText::new("Code").size(16.0));
                ui.add(egui::TextEdit::singleline(code).min_size(Vec2::new(128.0, 16.0)))
                    .on_hover_text("Please write your code");
                ui.end_row();
            })
    });
    let Some(server_list)=server_list else{
        return;
    };
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("Login")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    
                    for (ip,name) in &server_list.list {
                        
                        ui.label(RichText::new(name).size(24.0))
                            .on_hover_text(format!("target:{ip:?}"));
                        if ui.button(RichText::new("Join").size(16.0).weak()).clicked(){
                            *connect_ip=format!("{server_ip}:{ip}");
                            *is_master=false;
                            state.set(AppState::OnlineClient);
                        }
                        ui.end_row();
                    }
                });
        })
    });
}
