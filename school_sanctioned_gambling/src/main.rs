// #![feature(trivial_bounds)]
use bevy::{prelude::*, window::PresentMode};

mod credits;
mod game;
mod game_over;
mod menu;

mod online_screen;
mod options;
use credits::*;
use game::*;
use game_over::*;
use menu::*;
use online_screen::*;
use options::*;

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
    OnlineEnd,
    OnlineGamePlaying,
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
            GameOverPlugin,
            OnlineScreenPlugin,
        ))
        .run();
}
