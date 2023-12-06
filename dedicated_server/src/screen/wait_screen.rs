

use std::{thread::sleep, time::Duration};

use crate::options::components::OptionsResult;

use super::{AppState, Message, User, UserInfo, Users, UiInfo, UiInfoString, GameSigned};
use bevy::prelude::*;
use bevy_egui::{egui::RichText, *};


/// when game enter wait state,the function will run,every time show users infomation
pub(super) fn wait_screen_update(
    mut command:Commands,
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    users: Res<Users>,
    mut ui_info:ResMut<UiInfo>,
    signed: Res<GameSigned>,
    option:Res<OptionsResult>,
) {
    egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
        });
    });
    egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
        if ui.button(RichText::new("Back").size(16.0)).clicked() {
            // When click back,will send everyone server is close.

            state.set(AppState::GameEnd);
        }
    });
}
