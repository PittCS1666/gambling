use bevy::prelude::*;
use super::components::*;
use super::cards::*; 
use super::buttons::*;
use std::{thread, time};

pub fn load_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("game_screen.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Background);
    
    spawn_option_buttons(&mut commands, &asset_server);
}

pub fn tear_down_game_screen(
    mut commands: Commands, 
    mut background_query: Query<Entity, With<Background>>, 
    mut node_query: Query<Entity, With<NBundle>>,) 
{
    let node = node_query.single_mut();

    commands.entity(node).despawn_recursive();

    let background = background_query.single_mut();
    
    commands.entity(background).despawn_recursive();
}

pub fn turn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<PokerTurn>,
    players: Query<&Player>,
    community_query: Query<&CommunityCards>,
    mut deck: ResMut<Deck>,
    player_count: ResMut<NumPlayers>,
    mut ev_trigger: EventReader<PlayerTrigger>,
) {
    let total_players = players.iter().count();

    match state.phase {
        PokerPhase::PreFlop => {
            let cards = &mut deck.cards;
            if players.is_empty() {
                shuffle_cards(cards);
                let players = deal_hands(player_count.player_count, cards);
                spawn_player_cards(&mut commands, &asset_server, &players);
            }
            if state.current_player != 0 {
                println!("AI {} Taking Turn!", state.current_player);
                thread::sleep(time::Duration::from_millis(2000));
                next_player_turn(&mut state, total_players);
            } else {
                for _event in ev_trigger.iter() {
                    println!("Switching from player turn!");
                    next_player_turn(&mut state, total_players);
                }
            }
        }
        PokerPhase::Flop => {
            if community_query.iter().count() < 3 {
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, &asset_server, flop, &community_query);
            }
            if state.current_player != 0 {
                println!("AI {} Taking Turn!", state.current_player);
                thread::sleep(time::Duration::from_millis(2000));
                next_player_turn(&mut state, total_players);
            } else {
                for _event in ev_trigger.iter() {
                    println!("Switching from player turn!");
                    next_player_turn(&mut state, total_players);
                }
            }
        }
        PokerPhase::Turn => {
            if community_query.iter().count() < 4 {
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, &asset_server, flop, &community_query);
            }
            if state.current_player != 0 {
                println!("AI {} Taking Turn!", state.current_player);
                thread::sleep(time::Duration::from_millis(2000));
                next_player_turn(&mut state, total_players);
            } else {
                for _event in ev_trigger.iter() {
                    println!("Switching from player turn!");
                    next_player_turn(&mut state, total_players);
                }
            }     

        }
        PokerPhase::River => {
            if community_query.iter().count() < 5 {
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, &asset_server, flop, &community_query);
            }
            if state.current_player != 0 {
                println!("AI {} Taking Turn!", state.current_player);
                thread::sleep(time::Duration::from_millis(2000));
                next_player_turn(&mut state, total_players);
            } else {
                for _event in ev_trigger.iter() {
                    println!("Switching from player turn!");
                    next_player_turn(&mut state, total_players);
                }
            }
        }
        PokerPhase::Showdown => {
            let winners = card_function(&community_query, &players);
            
            if winners.len() == 1 {
                println!("Player {} won the hand!", winners[0]);
            } else if winners.len() > 1 {
                let winners_list = winners.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                println!("Players {} have all tied and split the pot!", winners_list);
            }
            deck.cards = init_cards();
            state.phase = PokerPhase::PreFlop;
        }
    }
}

fn next_player_turn(state: &mut ResMut<PokerTurn>, total_players: usize) {
    state.current_player = (state.current_player + 1) % total_players;

    match state.phase {
        PokerPhase::PreFlop => {
            if state.current_player == 0 {
                state.phase = PokerPhase::Flop;
            }
        }
        PokerPhase::Flop => {
            if state.current_player == 0 {
                state.phase = PokerPhase::Turn;
            }
        }
        PokerPhase::Turn => {
            if state.current_player == 0 {
                state.phase = PokerPhase::River;
            }
        }
        PokerPhase::River => {
            if state.current_player == 0 {
                state.phase = PokerPhase::Showdown;
            }
        }
        PokerPhase::Showdown => {
        }
    } 
}