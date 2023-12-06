use super::{AppState, UserInfo, Users, GameSigned, GameInteraction};
use bevy::prelude::*;
use bevy_egui::{egui::RichText, *};

/// wait screen every second update
pub(super) fn wait_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    users: Res<Users>,
    signed: Res<GameSigned>,
    interaction: Res<GameInteraction>,
) {
    if interaction.is_master{
egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
            });
        });
    
        egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
            if ui.button(RichText::new("Start").size(16.0)).clicked() {
    
                signed.sd.send(Some(super::Message::Start));
            }
            if ui.button(RichText::new("Back").size(16.0)).clicked() {
    
                signed.sd.send(None);
            }
            
        });
    
        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("Login")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
    
                        for UserInfo { ip, name } in users.users.lock().unwrap().iter() {
                            
                            ui.label(RichText::new(name).size(24.0))
                                .on_hover_text(format!("target:{ip:?}"));
                            if ui.button(RichText::new("kick").size(16.0).weak()).clicked() {
                                signed.sd.send(Some(super::Message::Kick(ip.to_string())));
                            }
                            ui.end_row();
                        }
                    });
            })
        });
    }
    else{
        egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
            });
        });
    
        egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
            if ui.button(RichText::new("Back").size(16.0)).clicked() {
    
                signed.sd.send(None);
            }
        });
    
        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("Login")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
    
                        for UserInfo { ip, name } in users.users.lock().unwrap().iter() {
                            
                            ui.label(RichText::new(name).size(24.0))
                                .on_hover_text(format!("target:{ip:?}"));
                            ui.end_row();
                        }
                    });
            })
        });
    }
    
}
