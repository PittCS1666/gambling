pub mod resource;
mod start_screen;
mod wait_screen;

pub(crate) use super::AppState;
use bevy::prelude::*;
use bevy_egui::*;
pub(crate) use resource::{Interaction, User,Users};
pub struct OnlineScreenPlugin;

impl Plugin for OnlineScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Interaction>().init_resource::<Users>()
            .add_plugins(EguiPlugin)
            .add_systems(
                Update,
                start_screen::start_screen_update.run_if(in_state(AppState::OnlinePlay)),
            )
            .add_systems(
                Update,
                wait_screen::wait_screen_update.run_if(in_state(AppState::OnlineClient)),
            );
    }
}
