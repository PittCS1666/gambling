use bevy::{prelude::*, window::*};

mod server;
mod startscreen;
mod screen;
use options::OptionsPlugin;
use screen::{ScreenPlugin, ServerPlugin};
use server::*;
use startscreen::*;
mod options;
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    GameOptions,
    StartScreen, // The menu selection/create server button and any other user affordances
    ServerRunning, // Server is now active, right now has just an exit button
    GameRunning, // Game Running
    LoopGameRuning, // In loop Game Running
    OneGameOver, // One Game Over
    GameOver,    // Game over
    GameEnd,     // if game is over or menu is close,go to this state
}

fn main()
{
    App::new()
        .add_state::<AppState>()
        .add_plugins((

            DefaultPlugins.set(WindowPlugin
            {
                primary_window: Some(Window
                {
                    present_mode: PresentMode::Fifo,
                    resolution: WindowResolution::new(1280.0, 720.0),
                    title: "School Sanctioned Gambling Server".to_string(),
                    ..default()
                }),
                ..default()
            }),
            OptionsPlugin,
            ScreenPlugin,
            ServerPlugin,
        ))
        .run();
}