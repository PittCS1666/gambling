use bevy::{prelude::*, window::PresentMode};

mod credits;
mod menu;
mod game;
mod options;
mod networking;
mod game_over;

use game::*;
use menu::*;
use credits::*;
use options::*;
use networking::*;
use game_over::*;


const TITLE: &str = "School Sanctioned Gambling";
const WIN_WIDTH: f32 = 1280.;
const WIN_HEIGHT: f32 = 720.;


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    LocalPlay,
    OnlinePlay,
    OnlineServer,
    OnlineClient,
    Credits,
    Options,
    GameOver,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Fifo,
                    resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                    title: TITLE.into(),
                    ..default()
                }),
                ..default()
            }),
            MenuPlugin,
            CreditsPlugin,
            GamePlugin,
            OptionsPlugin,
            NetworkingPlugin,
            GameOverPlugin,
        ))
        .run();
}
