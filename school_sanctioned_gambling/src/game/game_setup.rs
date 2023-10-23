use bevy::prelude::*;
use super::components::*;
use super::cards::*; 
use super::buttons::*;
use crate:: AppState

pub fn load_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(init_cards_resource());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("game_screen.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Background);
    spawn_buttons(&mut commands, &asset_server);
}

pub fn tear_down_game_screen(
    mut commands: Commands, 
    mut background_query: Query<Entity, With<Background>>, 
<<<<<<< Updated upstream
    mut node_query: Query<Entity, With<NBundle>>,) 
=======
    mut node_query: Query<Entity, With<NBundle>>,
    mut player_entity_query: Query<(Entity, &mut Player)>,
    mut player_card_query: Query<Entity, With<VisPlayerCards>>,
    mut com_entity_query: Query<Entity, With<CommunityCards>>,) 
>>>>>>> Stashed changes
{
    let node = node_query.single_mut();

    commands.entity(node).despawn_recursive();

    let background = background_query.single_mut();
    
    commands.entity(background).despawn_recursive();
<<<<<<< Updated upstream
    //commands.entity(exit_button).despawn_recursive();
=======

    //let player_entity = player_entity_query.single_mut();

    //commands.entity(player_entity).despawn_recursive();

    let player_card = player_card_query.single_mut();

    commands.entity(player_card).despawn_recursive();

    let com = com_entity_query.single_mut();

    commands.entity(com).despawn_recursive();
}

fn process_player_turn(
    current_player: usize,
    state: &mut ResMut<PokerTurn>,
    player_entity_query: &mut Query<(Entity, &mut Player)>,
    player_count: &ResMut<NumPlayers>,
    mut last_action: ResMut<LastPlayerAction>,
) {
    let mut player_raised = false;
    for (_entity, mut player) in player_entity_query.iter_mut() {
        if player.player_id == current_player {
            if player.player_id != 0 {
                //once the generate move is completely working this should be the code for the AI decisions
                /*if !player.has_folded && !player.is_all_in {
                    let player_move: String = generate_move(&player);
                    if player_move == "Raise" {
                        raise_action(state, player, player_count, &mut last_action);
                    }
                    else if player_move == "Call" {
                        call_action(state, player, player_count, &mut last_action);
                    }
                    else if player_move == "Fold" {
                        fold_action(state, player, player_count, &mut last_action);
                    }
                    else {
                        check_action(state, player, player_count, &mut last_action);
                    }
                }*/
                if !player.has_folded && !player.is_all_in {
                    let mut rng = rand::thread_rng();
                    if rng.gen_bool(0.2) {
                        player_raised = raise_action(state, player, player_count, &mut last_action,);
                    } else {
                        check_action(state, player, player_count, &mut last_action);
                    }
                    break;
                } else {
                    state.current_player = (current_player + 1) % player_count.player_count;
                    player.has_moved = true;
                }
            } else {
                if !player.has_folded && !player.is_all_in {
                    if let Some(PlayerAction::Check) = last_action.action {
                        check_action(state, player, player_count, &mut last_action);
                        break;
                    } else if let Some(PlayerAction::Raise) = last_action.action {
                        player_raised = raise_action(state, player, player_count, &mut last_action,);
                        break;
                    } else if let Some(PlayerAction::Fold) = last_action.action {
                        fold_action(state, player, player_count, &mut last_action);
                        break;
                    } else if let Some(PlayerAction::Call) = last_action.action {
                        call_action(state, player, player_count, &mut last_action);
                        break;
                    }
                } else {
                    state.current_player = (state.current_player + 1) % player_count.player_count;
                    player.has_moved = true;
                }
            }
        }
    }
    if player_raised {
        for (_entity, mut player) in player_entity_query.iter_mut() {
            if player.player_id != current_player {
                player.has_moved = false;
            } else {
                player.has_raised = false;
            }
        }
    }
}

pub fn check_action (
    state: &mut ResMut<PokerTurn>,
    mut player: Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
) {
    if state.current_top_bet > 0 {
        println!("Cannot check since top_bet is > {}!", state.current_top_bet);
        if player.player_id == 0 {
            last_action.action = Some(PlayerAction::None);
        }
    } else {
        println!("Player {} has checked!", player.player_id);
        player.has_moved = true;
        last_action.action = Some(PlayerAction::None);
        state.current_player = (state.current_player + 1) % player_count.player_count;
    }
}

pub fn raise_action (
    state: &mut ResMut<PokerTurn>,
    mut player: Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
) -> bool {
    if player.cash >= (state.current_top_bet + 50) - player.current_bet {
        state.pot += (state.current_top_bet + 50) - player.current_bet;
        state.current_top_bet += 50;
        println!("Player {} has raised the bet to {}", player.player_id, state.current_top_bet);
        player.has_moved = true;
        player.has_raised = true;
        player.cash -= state.current_top_bet - player.current_bet;
        if player.cash == 0 {
            player.is_all_in = true;
            println!("Player {} has gone all in!", player.player_id);
        }
        player.current_bet = state.current_top_bet;
        last_action.action = Some(PlayerAction::None);
        state.current_player = (state.current_player + 1) % player_count.player_count;
        return true;
    } else {
        println!("Player {} cannot raise due to going negative", player.player_id);
        if player.player_id == 0 {
            last_action.action = Some(PlayerAction::None);
        }
        return false;
    }
}

pub fn fold_action(
    state: &mut ResMut<PokerTurn>,
    mut player: Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
) {
    println!("Player {} has folded!", player.player_id);
    player.has_moved = true;
    player.has_folded = true;
    if player.player_id == 0 {
        last_action.action = Some(PlayerAction::None);
    }
    state.current_player = (state.current_player + 1) % player_count.player_count;
}

pub fn call_action(
    state: &mut ResMut<PokerTurn>,
    mut player: Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
) {
    if player.cash >= state.current_top_bet - player.current_bet {
        println!("Player {} has called!", player.player_id);
        player.has_moved = true;
        if player.player_id == 0 {
            last_action.action = Some(PlayerAction::None);
        }
        state.pot += state.current_top_bet - player.current_bet;
        player.cash -= state.current_top_bet - player.current_bet;
        if player.cash == 0 {
            player.is_all_in = true;
            println!("Player {} has gone all in!", player.player_id);
        }
        player.current_bet = state.current_top_bet;
        state.current_player = (state.current_player + 1) % player_count.player_count;
    } else {
        println!("Player {} has gone all in!", player.player_id);
        player.has_moved = true;
        player.is_all_in = true;
        if player.player_id == 0 {
            last_action.action = Some(PlayerAction::None);
        }
        state.pot += player.cash;
        player.current_bet = player.cash + player.current_bet;
        player.cash = 0;
        state.current_player = (state.current_player + 1) % player_count.player_count;
    }
}

pub fn turn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<PokerTurn>,
    mut player_entity_query: Query<(Entity, &mut Player)>,
    mut player_card_query: Query<Entity, With<VisPlayerCards>>,
    community_query: Query<&CommunityCards>,
    mut com_entity_query: Query<Entity, With<CommunityCards>>,
    mut deck: ResMut<Deck>,
    player_count: ResMut<NumPlayers>,
    last_action: ResMut<LastPlayerAction>,
    mut app_state_next_state: ResMut<NextState<AppState>>
) {

    let current_player_moved = player_entity_query.iter()
        .find_map(|(_entity, player)| {
            if player.player_id == state.current_player {
                Some(player.has_moved)
            } else {
                None
            }
        }).unwrap_or(false);

    
    let players_no_cash = player_entity_query.iter().filter(|(_entity, player)| player.cash == 0).count();
        if players_no_cash == player_count.player_count - 1{
        println!("Only one player with money left game over");
        app_state_next_state.set(AppState::MainMenu);
    }
    
    // If only one player left go straight to showdown phase
    let active_players_count = player_entity_query.iter().filter(|(_entity, player)| !player.has_folded).count();
    if active_players_count == 1 {
        state.phase = PokerPhase::Showdown;
    }

    match state.phase {
        PokerPhase::PreFlop => {
            if !state.round_started {
                println!("Phase is now in PreFlop!");
                let cards = &mut deck.cards;
                shuffle_cards(cards);
                let players_hands = deal_hands(player_count.player_count, cards);
                spawn_player_cards(&mut commands, &asset_server, &players_hands, &mut player_entity_query);
                state.round_started = true;
            }

            if !current_player_moved {
                process_player_turn(state.current_player, &mut state, &mut player_entity_query, &player_count, last_action);
            }
            next_player_turn(&mut state, &mut player_entity_query, player_count.player_count);
        }
        PokerPhase::Flop => {
            if community_query.iter().count() < 3 {
                println!("Phase is now in flop!");
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, &asset_server, flop, &community_query);
            }
            if !current_player_moved {
                process_player_turn(state.current_player, &mut state, &mut player_entity_query, &player_count, last_action);
            }
            next_player_turn(&mut state, &mut player_entity_query, player_count.player_count);
        }
        PokerPhase::Turn => {
            if community_query.iter().count() < 4 {
                println!("Phase is now in Turn!");
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, &asset_server, flop, &community_query);
            }
            if !current_player_moved {
                process_player_turn(state.current_player, &mut state, &mut player_entity_query, &player_count, last_action);
            }
            next_player_turn(&mut state, &mut player_entity_query, player_count.player_count); 
        }
        PokerPhase::River => {
            if community_query.iter().count() < 5 {
                println!("Phase is now in River!");
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, &asset_server, flop, &community_query);
            }
            if !current_player_moved {
                process_player_turn(state.current_player, &mut state, &mut player_entity_query, &player_count, last_action);
            }
            next_player_turn(&mut state, &mut player_entity_query, player_count.player_count);
        }
        PokerPhase::Showdown => {
            // Check the winners using poorly named card_function, the players is derived from the Entity Player query and iterated over to just return the players
            // and remove the entities so that player_entity_query can be used in this instance
            card_function(&community_query, &player_entity_query.iter().map(|(_, player)| player).collect::<Vec<&Player>>());

            // This is all to reinitialize the cards so another round may begin
            deck.cards = init_cards();
            let player_card_bundle = player_card_query.single_mut();
            commands.entity(player_card_bundle).despawn_recursive();

            for entity in com_entity_query.iter() {
                commands.entity(entity).despawn();
            }

            println!("The pot won equals {}", state.pot);

            state.pot = 0;
            state.current_top_bet = 0;
            state.current_player = (state.current_player + 1) % player_count.player_count;


            for (_, mut player) in player_entity_query.iter_mut() {
                player.has_folded = false;
                player.current_bet = 0;
                player.has_moved = false;
                player.is_all_in = false;
                player.has_raised = false;
            }

            state.round_started = false;
            state.phase = PokerPhase::PreFlop;
        }
    }
}

fn next_player_turn(
    state: &mut ResMut<PokerTurn>,
    player_entity_query: &mut Query<(Entity, &mut Player)>,
    _total_players: usize,
) {

    let players_moved_count = player_entity_query.iter().filter(|(_entity, player)| player.has_moved && !player.has_folded).count();

    let active_players_count = player_entity_query.iter().filter(|(_entity, player)| !player.has_folded).count();

    if players_moved_count == active_players_count && player_entity_query.iter().count() > 0{
        match state.phase {
            PokerPhase::PreFlop => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                    player.current_bet = 0;
                    player.has_raised = false;
                }        
                state.phase = PokerPhase::Flop;
                state.current_top_bet = 0;
            }
            PokerPhase::Flop => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                    player.current_bet = 0;
                    player.has_raised = false;
                }
                state.phase = PokerPhase::Turn;
                state.current_top_bet = 0;
            }
            PokerPhase::Turn => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                    player.current_bet = 0;
                    player.has_raised = false;
                }
                state.phase = PokerPhase::River;
                state.current_top_bet = 0;
            }
            PokerPhase::River => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                    player.current_bet = 0;
                    player.has_raised = false;
                }
                state.phase = PokerPhase::Showdown;
                state.current_top_bet = 0;
            }
            PokerPhase::Showdown => {}
        }
    }
>>>>>>> Stashed changes
}