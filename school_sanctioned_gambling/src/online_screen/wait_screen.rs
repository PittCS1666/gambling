use super::{AppState, Interaction, User, Users};
use bevy::prelude::*;
use bevy_egui::{egui::RichText, *};

pub(super) fn wait_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    users: Res<Users>,
) {
    let Users { ref users } = users.as_ref();
    egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
        });
    });

    egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
        if ui.button(RichText::new("Back").size(16.0)).clicked() {

            state.set(AppState::OnlinePlay);
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
                    } in users.iter()
                    {
                        ui.label(RichText::new(name).size(24.0))
                            .on_hover_text(format!("target:{ip:?}"));

                        ui.end_row();
                    }
                });
        })
    });
}
