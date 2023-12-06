mod client;
pub mod resource;
pub mod sd;
mod start_screen;
mod wait_screen;
pub(crate) use super::AppState;
use bevy::prelude::*;
use bevy_egui::*;
pub(crate) use resource::{GameInteraction, GameSigned, Message, UserInfo, Users};

pub struct OnlineScreenPlugin;

impl Plugin for OnlineScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameInteraction>()
            .init_resource::<Users>()
            .add_plugins(client::ClientPlugin)
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
