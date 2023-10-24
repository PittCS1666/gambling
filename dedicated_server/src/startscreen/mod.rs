use bevy::prelude::*;
use crate::AppState;

mod systems;
use systems::*;
mod components;

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::StartScreen), on_entry)
        .add_systems(Update, create_server_button_interaction.run_if(in_state(AppState::StartScreen)));
    }
}