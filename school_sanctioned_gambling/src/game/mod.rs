use bevy::prelude::*;
use crate::AppState;

mod systems;
mod components;

use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::LocalPlay), load_game);
    }
}