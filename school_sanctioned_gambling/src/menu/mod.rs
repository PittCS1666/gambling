use crate::AppState;
use bevy::prelude::*;

mod systems;
use systems::*;
mod components;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), load_menu)
            .add_systems(OnExit(AppState::MainMenu), tear_down_menu)
            .add_systems(
                Update,
                local_button_interaction.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                online_button_interaction.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                exit_button_interaction.run_if(in_state(AppState::MainMenu)),
            );
    }
}
