use crate::AppState;
use bevy::prelude::*;

mod systems;
use systems::*;
pub mod components;
use components::*;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AiButtonState::default())
        .add_event::<ButtonPressEvent>()
        .add_systems(OnEnter(AppState::Options), load_options)
            .add_systems(OnExit(AppState::Options), tear_down_options)
            .add_systems(
                Update,
                play_button_interaction.run_if(in_state(AppState::Options)),
            )
            .add_systems(Update, make_scrolly.run_if(in_state(AppState::Options)))
            .add_systems(Update, handle_keyboard.run_if(in_state(AppState::Options)))
            .add_systems(Update, activate.run_if(in_state(AppState::Options)))
            .add_systems(
                Update,
                load_button_interaction.run_if(in_state(AppState::Options)),
            )
            .add_systems(Update, easy_button_interaction.run_if(in_state(AppState::Options)))
            .add_systems(Update, hard_button_interaction.run_if(in_state(AppState::Options)))
            .add_systems(Update, cheating_button_interaction.run_if(in_state(AppState::Options)))
            .add_systems(Update, update_button_colors.run_if(in_state(AppState::Options)));
    }
}
