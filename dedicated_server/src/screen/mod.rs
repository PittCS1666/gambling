pub mod resource;
pub mod sd;
mod server;
mod start_screen;
mod wait_screen;


pub(crate) use super::AppState;
use bevy::prelude::*;
use bevy_egui::*;
pub(crate) use resource::{GameInteraction, GameSigned, Message, User, UserInfo, Users};
pub use sd::S2D;
pub struct ScreenPlugin;
pub use server::ServerPlugin;
impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameInteraction>()
            .init_resource::<Users>()
            .add_plugins(EguiPlugin)
            .add_systems(
                Update,
                start_screen::start_screen_update.run_if(in_state(AppState::StartScreen)),
            )
            .add_systems(
                Update,
                wait_screen::wait_screen_update.run_if(in_state(AppState::ServerRunning)),
            );
    }
}
