use bevy::prelude::*;
use crate::AppState;

mod systems;
use systems::*;
mod components;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::OnlinePlay), on_entry)
        .add_systems(Update, fill_textboxes.run_if(in_state(AppState::OnlinePlay)))
        .add_systems(Update, join_server_button_interaction.run_if(in_state(AppState::OnlinePlay)))
        .add_systems(Update, ip_textbox_button_interaction.run_if(in_state(AppState::OnlinePlay)))
        .add_systems(OnExit(AppState::OnlinePlay), remove_gui)

        //.add_systems(OnEnter(AppState::OnlineServer), remove_gui)
        //.add_systems(OnEnter(AppState::OnlineServer), server_on_enter)
        //.add_systems(Update, server_on_update.run_if(in_state(AppState::OnlineServer)))

        //.add_systems(OnEnter(AppState::OnlineClient), remove_gui)
        .add_systems(OnEnter(AppState::OnlineClient), client_on_enter)
        .add_systems(Update, client_on_update.run_if(in_state(AppState::OnlineClient)));
    }
}
