use bevy::prelude::*;
use crate::AppState;

mod cards;
mod game_setup;
mod buttons;
mod hand_evaluation;
mod components;

use game_setup::*;
use buttons::*;
use cards::*;
use components::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PlayerTrigger>()
        .insert_resource(init_cards_resource())
        //This line can be used to set whose turn it is
        .insert_resource(PokerTurn { current_player: 0, phase: PokerPhase::PreFlop, round_started: false,})
        //Update this line to increase number of players for now
        .insert_resource(NumPlayers { player_count: 2 })
        .add_systems(OnEnter(AppState::LocalPlay), load_game)
        .add_systems(OnExit(AppState::LocalPlay), tear_down_game_screen)
        .add_systems(Update, turn_system.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, check_button_interaction.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, raise_button_interaction.run_if(in_state(AppState::LocalPlay)));
    }
}