use super::{AppState, Message, User, UserInfo, Users};
use bevy::prelude::*;
use bevy_egui::{egui::RichText, *};

pub(super) fn wait_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    users: Res<Users>,
) {
    egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
        });
    });
    let Users { ref users } = users.as_ref();
    let users = users.blocking_read();
    let users_list = users
        .iter()
        .map(|User { ip, name, .. }| {
            let (ip, name) = (ip.to_string(), name.to_string());
            UserInfo { ip, name }
        })
        .collect::<Vec<UserInfo>>();

    egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
        if ui.button(RichText::new("Start").size(16.0)).clicked() {
            users.iter().for_each(|e| {
                e.send_message.send(Message::Start);
            });
            state.set(AppState::GameRunning);
        }

        if ui.button(RichText::new("Back").size(16.0)).clicked() {
            // When click back,will send everyone server is close.

            state.set(AppState::GameEnd);
        }
    });

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("Login")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for UserInfo { ip, name } in users_list.iter() {
                        ui.label(RichText::new(name).size(24.0))
                            .on_hover_text(format!("target:{ip:?}"));

                        if ui.button(RichText::new("kick").size(16.0).weak()).clicked() {
                            users.iter().for_each(|e| {
                                e.send_message.blocking_send(Message::Kick(ip.to_string()));
                            });
                        }
                        ui.end_row();
                    }
                });
        })
    });
}
