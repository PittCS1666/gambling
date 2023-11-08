use super::{AppState, Message, User, Users};
use bevy::prelude::*;
use bevy_egui::{egui::RichText, *};

pub(super) fn wait_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    users: Res<Users>,
) {
    let Users { ref users} = users.as_ref();
    egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
        });
    });

    egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
        if ui.button(RichText::new("Start").size(16.0)).clicked() {
            users.iter().for_each(|user| {
                (user.send_message)(Message::Start);
            });
            state.set(AppState::GameRunning);
        }

        if ui.button(RichText::new("Back").size(16.0)).clicked() {
            // When click back,will send everyone server is close.
            users.iter().for_each(|user| {
                (user.send_message)(Message::Close);
            });
            state.set(AppState::StartScreen);
        }
    });

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("Login")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for User {
                        ip,
                        name,
                        send_message,
                    } in users.iter()
                    {
                        ui.label(RichText::new(name).size(24.0))
                            .on_hover_text(format!("target:{ip:?}"));

                        if ui.label(RichText::new("kick").size(16.0).weak()).clicked() {
                            send_message(Message::Kick);
                        }
                        ui.end_row();
                    }
                });
        })
    });
}
