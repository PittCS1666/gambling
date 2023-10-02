use bevy::prelude::*;
use crate::AppState;

mod components;
mod systems;
use systems::*;


pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Credits), setup_credits)
        .add_systems(Update, next_slide.run_if(in_state(AppState::Credits)));
    }
}