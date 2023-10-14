use bevy::prelude::*;
use crate::AppState;

mod systems;
use systems::*;
mod components;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Options), load_options)
        .add_systems(OnExit(AppState::Options), tear_down_options)
        .add_systems(Update, play_button_interaction.run_if(in_state(AppState::Options)));
    }
}