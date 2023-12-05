use super::{AppState, UserInfo, Users, GameSigned};
use bevy::prelude::*;
use bevy_egui::{egui::RichText, *};

/// wait screen every second update
pub(super) fn wait_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    users: Res<Users>,
    signed: Res<GameSigned>,
) {
    egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
        });
    });

    egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
        if ui.button(RichText::new("Back").size(16.0)).clicked() {

            signed.sd.send(());
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
