use bevy::prelude::*;
use super::components::*;
use super::cards::*; 
use super::buttons::*;
use rand::Rng;

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

    commands.spawn(Text2dBundle {
        text: Text::from_section("SB", TextStyle {
            font: asset_server.load("fonts/Lato-Black.ttf"),
            font_size: 25.,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(PLAYER_BLIND_POS.0, PLAYER_BLIND_POS.1, PLAYER_BLIND_POS.2),
        ..default()
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

        commands.spawn(Text2dBundle {
            text: Text::from_section("SB", TextStyle {
                font: asset_server.load("fonts/Lato-Black.ttf"),
                font_size: 25.,
                color: Color::WHITE,
            }),
            transform: Transform::from_xyz(ai_blind_pos[i].0, ai_blind_pos[i].1, ai_blind_pos[i].2),
            ..default()
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
    for (_entity, mut player) in player_entity_query.iter_mut() {
        if player.player_id == current_player {
            if player.player_id != 0 {
                if !player.has_folded{
                    player.has_moved = true;
                    let mut rng = rand::thread_rng();
                    if rng.gen_bool(0.2) {
                        player.has_folded = true;
                        println!("Player {} has folded!", player.player_id);
                    }
                    state.current_player = (current_player + 1) % player_count.player_count;
                    break;
                } else {
                    state.current_player = (current_player + 1) % player_count.player_count;
                    player.has_moved = true;
                }
            } else {
                if !player.has_folded {
                    if let Some(PlayerAction::Check) = last_action.action {
                        println!("Player 0 has checked!");
                        player.has_moved = true;
                        last_action.action = Some(PlayerAction::None);
                        state.current_player = (state.current_player + 1) % player_count.player_count;
                        break;
                    } else if let Some(PlayerAction::Raise) = last_action.action {
                        state.pot += (state.current_top_bet + 50) - state.current_top_bet;
                        state.current_top_bet += 50;
                        println!("Player 0 has raised the bet to {}", state.current_top_bet);
                        player.has_moved = true;
                        player.cash -= state.current_top_bet - player.current_bet;
                        player.current_bet = state.current_top_bet;
                        last_action.action = Some(PlayerAction::None);
                        state.current_player = (state.current_player + 1) % player_count.player_count;
                        break;
                    } else if let Some(PlayerAction::Fold) = last_action.action {
                        println!("Player 0 has folded!");
                        player.has_moved = true;
                        player.has_folded = true;
                        last_action.action = Some(PlayerAction::None);
                        state.current_player = (state.current_player + 1) % player_count.player_count;
                        break;
                    } else if let Some(PlayerAction::Call) = last_action.action {
                        println!("Player 0 has called!");
                        player.has_moved = true;
                        last_action.action = Some(PlayerAction::None);
                        state.pot += state.current_top_bet;
                        player.cash -= state.current_top_bet;
                        player.current_bet = state.current_top_bet;
                        break;
                    }
                } else {
                    state.current_player = (state.current_player + 1) % player_count.player_count;
                    player.has_moved = true;
                }
            }
        }
        //println!("Player {} cash: {}, current bet: {}, folded?: {}, moved? {}", player.player_id, player.cash, player.current_bet, player.has_folded, player.has_moved);   
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
) {
    let current_player_moved = player_entity_query.iter()
        .find_map(|(_entity, player)| {
            if player.player_id == state.current_player {
                Some(player.has_moved)
            } else {
                None
            }
        }).unwrap_or(false);

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
            // for (_entity, player) in player_entity_query.iter_mut() {
            //     println!("Player {} has moved: {}", player.player_id, player.has_moved);
            // }   
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
            // for (_entity, player) in player_entity_query.iter_mut() {
            //     println!("Player {} has moved: {}", player.player_id, player.has_moved);
            // } 
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
            // for (_entity, player) in player_entity_query.iter_mut() {
            //     println!("Player {} has moved: {}", player.player_id, player.has_moved);
            // } 
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
            // for (_entity, player) in player_entity_query.iter_mut() {
            //     println!("Player {} has moved: {}", player.player_id, player.has_moved);
            // } 
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

            state.pot = 0;
            state.current_top_bet = 0;
            state.current_player = 2;


            for (_, mut player) in player_entity_query.iter_mut() {
                player.has_folded = false;
                player.current_bet = 0;
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
                }        
                state.phase = PokerPhase::Flop;
            }
            PokerPhase::Flop => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                }
                state.phase = PokerPhase::Turn;
            }
            PokerPhase::Turn => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                }
                state.phase = PokerPhase::River;
            }
            PokerPhase::River => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                }
                state.phase = PokerPhase::Showdown;
            }
            PokerPhase::Showdown => {}
        }
    }
}