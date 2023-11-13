use bevy::prelude::*;
use crate::AppState;

mod systems;
use systems::*;
mod components;

pub struct ServerRunningPlugin;

impl Plugin for ServerRunningPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::ServerRunning), on_entry)
        .add_systems(Update, server_on_update.run_if(in_state(AppState::ServerRunning)))
        .add_systems(Update, destroy_server_button_interaction.run_if(in_state(AppState::ServerRunning)));
    }
}