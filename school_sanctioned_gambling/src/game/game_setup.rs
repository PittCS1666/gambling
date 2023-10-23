use bevy::prelude::*;
use super::components::*;
use super::cards::*; 
use super::buttons::*;
use rand::Rng;
use super::preflop_eval::*;

// After each player move check if only one player has not folded, if only one player left instantly skip to them winnning and resetting the hand
// Add functionality so that if max current_bet > 0 all other players must either fold or have current_bet >= max current_bet
// Add easy AI choices
// Add win state/end of game
// Add checks to make sure player doesn't go to negative on raise/call (call on all in will have special circumstances)
// If player has no more to bet and hits 0 during a round force them to always have_moved and also allow them to pass any additional bets
// Look at logic for additional bets after a player has gone all in if more than one player is matching the all in bet
// Add visuals for spawning other players at table depending on amount of players
// Add visuals to change cash amount in corner and also add pot amount somewhere on screen
// Add visuals to players to signify what action they took
// Add visuals to signify current bet needed
// Add delay of AI to make it seem like AI Players are thinking
// Add logic to pull certain values from options screen
// Add small blind and big blinds plus logic

const PLAYER_SIZE: f32 =  60.;
const PLAYER_POS: (f32, f32, f32) = (140., -175., 2.);
const PLAYER_BLIND_POS: (f32, f32, f32) = (140., -220., 2.);

pub fn load_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_num: Res<NumPlayers>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("game_screen.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Background);
    
    spawn_option_buttons(&mut commands, &asset_server);
    spawn_players(&mut commands, &asset_server, &player_num);
}

fn spawn_players(commands: &mut Commands, asset_server: &Res<AssetServer>, player_num: &Res<NumPlayers>) {
    let ai_pos: Vec<(f32, f32, f32)> = vec![(225., 170., 2.), (435., 10., 2.), (-140., -175., 2.), (-435., 10., 2.), (-225., 170., 2.)];

    //spawn player in the same spot every game
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(PLAYER_POS.0, PLAYER_POS.1, PLAYER_POS.2),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(Text2dBundle {
                text: Text::from_section("You",
                TextStyle {
                    font: asset_server.load("fonts/Lato-Black.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                }),
                transform: Transform::from_xyz(0., 0., 3.),
                ..default()
        });
    });

    

    //spawn the AI players
    for i in 0..player_num.player_count - 1 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(ai_pos[i].0, ai_pos[i].1, ai_pos[i].2),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                    text: Text::from_section(String::from("AI ") + &(i + 1).to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    }),
                    transform: Transform::from_xyz(0., 0., 3.),
                    ..default()
            });
        });

    }
    
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
    if state.current_top_bet > player.current_bet {
        println!("Cannot check since top_bet ({}) is > your current bet ({})!", state.current_top_bet, player.current_bet);
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
    com_entity_query: Query<Entity, With<CommunityCards>>,
    mut deck: ResMut<Deck>,
    player_count: ResMut<NumPlayers>,
    last_action: ResMut<LastPlayerAction>,
    mut blind_text_query: Query<Entity, With<Blind>>,
) {
    let ai_blind_pos: Vec<(f32, f32, f32)> = vec![(225., 215., 2.), (435., 55., 2.), (-140., -220., 2.), (-435., 55., 2.), (-225., 215., 2.)];

    let current_player_moved = player_entity_query.iter()
        .find_map(|(_entity, player)| {
            if player.player_id == state.current_player {
                Some(player.has_moved)
            } else {
                None
            }
        }).unwrap_or(false);
    
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
                    
                    
                    //loops through the players to find the big and small blinds
                    for (_, mut player) in player_entity_query.iter_mut() {
                        if player.player_id == state.small_blind {
                            player.small_blind = true;
                            player.cash -= state.small_blind_val;
                            player.current_bet = state.small_blind_val;
                            state.pot += state.small_blind_val;
                            
                            //spawn the blind text
                            if player.player_id == 0 {
                                commands.spawn(Text2dBundle {
                                    text: Text::from_section("SB", TextStyle {
                                        font: asset_server.load("fonts/Lato-Black.ttf"),
                                        font_size: 25.,
                                        color: Color::WHITE,
                                    }),
                                    transform: Transform::from_xyz(PLAYER_BLIND_POS.0, PLAYER_BLIND_POS.1, PLAYER_BLIND_POS.2),
                                    ..default()
                                })
                                .insert(Blind);
                            }
                            else {
                                commands.spawn(Text2dBundle {
                                    text: Text::from_section("SB", TextStyle {
                                        font: asset_server.load("fonts/Lato-Black.ttf"),
                                        font_size: 25.,
                                        color: Color::WHITE,
                                    }),
                                    transform: Transform::from_xyz(
                                        ai_blind_pos[player.player_id - 1].0,
                                        ai_blind_pos[player.player_id - 1].1,
                                        ai_blind_pos[player.player_id - 1].2),
                                    ..default()
                                })
                                .insert(Blind);
                            }
                        }
                        else if player.player_id == state.big_blind {
                            player.big_blind = true;
                            player.cash -= state.big_blind;
                            player.current_bet = state.big_blind_val;
                            state.pot += state.big_blind_val;
                            state.current_top_bet = state.big_blind_val;

                            //spawn blind text
                            if player.player_id == 0 {
                                commands.spawn(Text2dBundle {
                                    text: Text::from_section("BB", TextStyle {
                                        font: asset_server.load("fonts/Lato-Black.ttf"),
                                        font_size: 25.,
                                        color: Color::WHITE,
                                    }),
                                    transform: Transform::from_xyz(PLAYER_BLIND_POS.0, PLAYER_BLIND_POS.1, PLAYER_BLIND_POS.2),
                                    ..default()
                                })
                                .insert(Blind);
                            }
                            else {
                                commands.spawn(Text2dBundle {
                                    text: Text::from_section("BB", TextStyle {
                                        font: asset_server.load("fonts/Lato-Black.ttf"),
                                        font_size: 25.,
                                        color: Color::WHITE,
                                    }),
                                    transform: Transform::from_xyz(
                                        ai_blind_pos[player.player_id - 1].0,
                                        ai_blind_pos[player.player_id - 1].1,
                                        ai_blind_pos[player.player_id - 1].2),
                                    ..default()
                                })
                                .insert(Blind);
                            }
                        }
                    }
                    println!("Pot is: {}", state.pot);
                    state.round_started = true;
                    state.current_player = (state.big_blind + 1) % player_count.player_count;
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
            state.small_blind = (state.small_blind + 1) % player_count.player_count;
            state.big_blind = (state.big_blind + 1) % player_count.player_count;
            state.current_player = state.big_blind + 1 % player_count.player_count;

            for blind in blind_text_query.iter_mut() {
                commands.entity(blind).despawn_recursive();
            }


            for (_, mut player) in player_entity_query.iter_mut() {
                player.has_folded = false;
                player.current_bet = 0;
                player.has_moved = false;
                player.is_all_in = false;
                player.has_raised = false;
                player.small_blind = false;
                player.big_blind = false;
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
}