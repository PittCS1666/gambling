use super::{AppState, GameInteraction};
use bevy::prelude::*;
use bevy_egui::{
    egui::{RichText, Vec2},
    *,
};

/// init wait lobby,can enter server.
pub(super) fn start_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    mut interaction: ResMut<GameInteraction>,
) {
    let GameInteraction {
        ref mut server_ip,
        ref mut name,
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
                if ui.button(RichText::new("Connect").size(16.0)).clicked() {
                    state.set(AppState::OnlineClient);
                }
                ui.end_row();

                ui.label(RichText::new("Server IP").size(16.0));
                ui.add(egui::TextEdit::singleline(server_ip).min_size(Vec2::new(128.0, 16.0)))
                    .on_hover_text(
                        "if you are client please write you connect ip and port,else write bind ip",
                    );
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
}
