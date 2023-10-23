use bevy::prelude::*;
use super::components::*;
use super::cards::*; 
use super::buttons::*;
use rand::Rng;
// use super::preflop_eval::*;
use crate::AppState;

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
    let ai_blind_pos: Vec<(f32, f32, f32)> = vec![(225., 215., 2.), (435., 55., 2.), (-140., -220., 2.), (-435., 55., 2.), (-225., 215., 2.)];

    //spawn player in the same spot every game
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(PLAYER_POS.0, PLAYER_POS.1, PLAYER_POS.2),
        ..default()
    }).insert(VisPlayers)
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

    commands.spawn(Text2dBundle {
        text: Text::from_section("SB", TextStyle {
            font: asset_server.load("fonts/Lato-Black.ttf"),
            font_size: 25.,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(PLAYER_BLIND_POS.0, PLAYER_BLIND_POS.1, PLAYER_BLIND_POS.2),
        ..default()
    }).insert(Blinds);

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
        }).insert(VisPlayers)
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

        commands.spawn(Text2dBundle {
            text: Text::from_section("SB", TextStyle {
                font: asset_server.load("fonts/Lato-Black.ttf"),
                font_size: 25.,
                color: Color::WHITE,
            }),
            transform: Transform::from_xyz(ai_blind_pos[i].0, ai_blind_pos[i].1, ai_blind_pos[i].2),
            ..default()
        }).insert(Blinds);
    }
    
}

pub fn tear_down_game_screen(
    mut commands: Commands, 
    mut background_query: Query<Entity, With<Background>>, 
    mut node_query: Query<Entity, With<NBundle>>,
    player_entity_query: Query<Entity,  With<Player>>,
    mut player_card_query: Query<Entity, With<VisPlayerCards>>,
    com_entity_query: Query<Entity, With<CommunityCards>>,
    vis_player_query: Query<Entity, With<VisPlayers>>,
    blinds_query: Query<Entity, With<Blinds>>,
    vis_cash_query: Query<Entity, With<VisPlayerCash>>,
) {
    let node = node_query.single_mut();

    commands.entity(node).despawn_recursive();
    let background = background_query.single_mut();
    
    commands.entity(background).despawn_recursive();

    if blinds_query.iter().next().is_some() {
        for entity in blinds_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if player_entity_query.iter().next().is_some() {
        for entity in blinds_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if vis_cash_query.iter().next().is_some() {
        for entity in vis_cash_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if player_card_query.iter().next().is_some() {
        let player_card = player_card_query.single_mut(); 
        commands.entity(player_card).despawn_recursive();
    }

    if vis_player_query.iter().next().is_some() {
        for entity in vis_player_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
   
    if com_entity_query.iter().next().is_some() {
        for entity in com_entity_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
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
                        call_action(state, player, player_count, &mut last_action,);
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
    com_entity_query: Query<Entity, With<CommunityCards>>,
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
    if players_no_cash == 1 {
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
            let winner = card_function(&community_query, &player_entity_query.iter().map(|(_, player)| player).collect::<Vec<&Player>>());

            // This is all to reinitialize the cards so another round may begin
            deck.cards = init_cards();
            let player_card_bundle = player_card_query.single_mut();
            commands.entity(player_card_bundle).despawn_recursive();

            for entity in com_entity_query.iter() {
                commands.entity(entity).despawn();
            }

            for (_, mut player) in player_entity_query.iter_mut() {
                if winner == 0 {
                    if player.player_id == 0 {
                        println!("Player 0 wins and gains a pot of {}\n", state.pot);
                        player.cash += state.pot;
                    }
                } else if winner == 1 {
                    if player.player_id == 1 {
                        println!("Player 1 wins and gains a pot of {}\n", state.pot);
                        player.cash += state.pot;
                    }
                } else {
                    println!("Player {} ties and gains a pot of {}\n", player.player_id, state.pot/player_count.player_count);
                    player.cash += state.pot/player_count.player_count;
                }
           }

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
}
