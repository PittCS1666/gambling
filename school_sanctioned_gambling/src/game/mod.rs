use bevy::prelude::*;
use crate::AppState;

mod cards;
mod game_setup;
mod buttons;
mod hand_evaluation;
pub mod components;
mod easy_ai_logic;

use game_setup::*;
use cards::*;
use components::*;
use buttons::*;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(init_cards_resource())
        //This line can be used to set whose turn it is
        .insert_resource(PokerTurn { current_player: 1, phase: PokerPhase::PreFlop, round_started: false, pot: 0, current_top_bet:0, bet_made: false, pot_raised: false, small_blind: 1, big_blind: 0, small_blind_val: 25, big_blind_val: 50})
        //Update this line to increase number of players for now
        .insert_resource(NumPlayers { player_count: 2 })
        .insert_resource(LastPlayerAction{ action: Some(PlayerAction::None) })
        .add_systems(OnEnter(AppState::LocalPlay), load_assets)
        .add_systems(OnEnter(AppState::LocalPlay), load_game)
        .add_systems(OnExit(AppState::LocalPlay), tear_down_game_screen)
        .add_systems(Update, turn_system.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, check_button_interaction.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, raise_button_interaction.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, fold_button_interaction.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, call_button_interaction.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, handle_keyboard.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, activate.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, make_scrolly.run_if(in_state(AppState::LocalPlay)))
        .add_systems(Update, save_buton_interaction.run_if(in_state(AppState::LocalPlay)));
    }
}