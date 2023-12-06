use super::{AppState, GameInteraction};
use bevy::prelude::*;
use bevy_egui::{
    egui::{RichText, Vec2},
    *,
};

/// when game enter start state,the function will run,every time show users infomation
pub(super) fn start_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    mut interaction: ResMut<GameInteraction>,
) {
    let GameInteraction {
        ref mut server_ip,
        ref mut code,
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
                if ui.button(RichText::new("Create").size(16.0)).clicked() {
                    state.set(AppState::ServerRunning);
                }
                ui.end_row();
                if ui
                    .button(RichText::new("back options").size(16.0))
                    .clicked()
                {
                    state.set(AppState::GameOptions);
                }
                ui.end_row();
            })
    });
}
