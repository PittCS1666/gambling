use bevy::prelude::*;
use crate::AppState;

mod systems;
mod components;

use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::LocalPlay), load_game)
        .add_systems(OnExit(AppState::LocalPlay), tear_down_game_screen)
        .add_systems(Update, check_button_interaction.run_if(in_state(AppState::LocalPlay)));
    }
}