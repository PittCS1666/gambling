use bevy::{prelude::*, window::*};

mod screen;
mod server;
mod startscreen;
use screen::ScreenPlugin;
use server::*;
use startscreen::*;
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    StartScreen, // The menu selection/create server button and any other user affordances
    ServerRunning, // Server is now active, right now has just an exit button
    GameRunning,   // Game Running
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Fifo,
                    resolution: WindowResolution::new(1280.0, 720.0),
                    title: "School Sanctioned Gambling Server".to_string(),
                    ..default()
                }),
                ..default()
            }),
            ServerRunningPlugin,
            ScreenPlugin,
        ))
        .run();
}
