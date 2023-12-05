/*  ATTENTION WHOEVER STARTS WORKING ON THIS:
*   Things to note:
*       - This is just a loose guide to how I think the client side of the game could work
*       - This is the only file in this folder I modified for this specific purpose. That means that 
          the others may need to be modified or may not even be needed at all. They are simply there just in case
*       - The client will need all the player entities and the PokerTurn(state) object at a minimum
*       - The client will need to know which player_id belongs to it
*   
*   How I think it'll work:
*       - So obviously there is no turn_system function. This will not be necessary as the server will do that calculation
*       - The process_player_turn method will run every update, but the player can only make a move when it is their turn
*         which is indicated by the current_player variable in the state object
*       - After each move from another player happens, there needs to be some method that can update the visuals for all other clients
*       - When it is the players turn, they will make their move using the same buttons and the functions will all cary out like normal
*         but instead of going onto the next turn locally, it will then send the new game state to the server for processing
*
*   If you guys need any other help or have any questions please let me know, I'll do whatever I can to get this done. Goodluck
*/


use super::buttons::*;
use super::cards::*;
use super::components::*;
use super::hard_ai_logic::select_action_for_hand;
use crate::options::components::OptionsResult;
use crate::screen::GameSigned;
use crate::screen::Users;
use bevy::prelude::*;
use super::easy_ai_logic::*;
use crate::AppState;
use bevy::input::keyboard::KeyboardInput;
use bevy::text::BreakLineOn;
use serde_json::*;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

const PLAYER_SIZE: f32 = 60.;
const PLAYER_POS: (f32, f32, f32) = (140., -175., 2.);
const PLAYER_BLIND_POS: (f32, f32, f32) = (140., -220., 2.);

pub fn load_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_num_mut: ResMut<NumPlayers>,
    mut poker_turn: ResMut<PokerTurn>,
    options_result: Res<OptionsResult>,
    // added users resource
    users: Res<Users>,
) {
    let mut player_money = options_result.money_per_player;
    let mut player_bet = 0;
    let pot = 0;
    let top_bet = 0;

    // get first user's name in users vec
    let first_user_name = if let Some(first_user) = users.lock().unwrap().first() {
        first_user.name.clone()
    } else{
        "unknown".to_string()
    }

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("game_screen.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Background);

    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: format!("Cash: ${}\n", player_money),
                        style: TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                    TextSection {
                        value: format!("Your Current Bet: ${}\n", player_bet),
                        style: TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                    TextSection {
                        value: format!("Current Pot: ${}\n", pot),
                        style: TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                    TextSection {
                        value: format!("Current Top Bet: ${}", top_bet),
                        style: TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                ],
                alignment: TextAlignment::Center,
                linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
            },
            ..Default::default()
        })
        .insert(VisText);

    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                //align_items: AlignItems::Center,
                //justify_content: JustifyContent::Center,
                left: Val::Px(540.),
                width: Val::Px(400.),
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: format!("It is {}'s Turn!\n", first_user_name),//"It is AI 1's Turn!\n".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                    TextSection {
                        value: String::new(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                ],
                alignment: TextAlignment::Center,
                linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
            },
            ..Default::default()
        })
        .insert(VisText);

    spawn_option_buttons(&mut commands, &asset_server);
    spawn_players(&mut commands, &asset_server, &player_num_mut, &users);
}

fn spawn_players(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_num: &ResMut<NumPlayers>,
    // added users to be passed in so we can differentiate them
    users: &Res<Users>, 
) {
    let player_pos: Vec<(f32, f32, f32)> = vec![
        (225., 170., 2.),
        (435., 10., 2.),
        (140., -175., 2.),
        (-140., -175., 2.),
        (-435., 10., 2.),
        (-225., 170., 2.),
    ];

    // TODO: need some way for the client to know what ID they are
    //       for the spawning logic and other things

    //spawn the players
    for i in 0..player_num.player_count {

        let unique_client_id = {
            let users_data = users.users.lock().unwrap();
            let user = &users_data[i];
            user.name.clone()
        };

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(player_pos[i].0, player_pos[i].1, player_pos[i].2),
                ..default()
            })
            .insert(VisPlayers)
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        format!("{}", unique_client_id), // will display name of client //+ &(i + 1).to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ),
                    transform: Transform::from_xyz(0., 0., 3.),
                    ..default()
                });
            });
    }
}

pub fn tear_down_game_screen(
    mut commands: Commands,
    mut background_query: Query<Entity, With<Background>>,
    mut node_query: Query<Entity, With<NBundle>>,
    player_entity_query: Query<Entity, With<Player>>,
    mut player_card_query: Query<Entity, With<VisPlayerCards>>,
    com_entity_query: Query<Entity, With<CommunityCards>>,
    vis_player_query: Query<Entity, With<VisPlayers>>,
    mut blinds_query: Query<Entity, With<Blind>>,
    vis_text_query: Query<Entity, With<VisText>>,
    mut state: ResMut<PokerTurn>,
) {
    //let node = node_query.single_mut();
    for node in node_query.iter_mut() {
        commands.entity(node).despawn_recursive();
    }

    //commands.entity(node).despawn_recursive();

    let background = background_query.single_mut();

    commands.entity(background).despawn_recursive();

    for entity in blinds_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    if player_entity_query.iter().next().is_some() {
        for entity in player_entity_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if vis_text_query.iter().next().is_some() {
        for entity in vis_text_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if !player_card_query.is_empty() {
        //let player_card = player_card_query.single_mut();
        for player_card in player_card_query.iter_mut() {
            commands.entity(player_card).despawn_recursive();
        }
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
    commands: &mut Commands,
    current_player: usize,
    state: &mut ResMut<PokerTurn>,
    player_entity_query: &mut Query<(Entity, &mut Player)>,
    player_count: &ResMut<NumPlayers>,
    mut last_action: ResMut<LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
    community_query: &mut Query<&CommunityCards>,
) {
    let mut player_raised = false;
    for (_entity, mut player) in player_entity_query.iter_mut() {
        if player.player_id == current_player {
            let mut text_iter = text_query.iter_mut();
            let _money_text = text_iter.next();
            let mut turn_text = text_iter.next().unwrap();

            if !player.has_folded && !player.is_all_in {
                //only allow actions when it is the players turn
                if player.player_id == self {   // TODO: again this will need to change based on what ID the specific client is
                    turn_text.sections[0].value = format!("It is your turn!\n");
                    if let Some(PlayerAction::Check) = last_action.action {
                        check_action(state, player, player_count, &mut last_action, text_query);
                        break;
                    } else if let Some(PlayerAction::Raise) = last_action.action {
                        player_raised =
                            raise_action(state, player, player_count, &mut last_action, text_query);
                        break;
                    } else if let Some(PlayerAction::Fold) = last_action.action {
                        fold_action(state, player, player_count, &mut last_action, text_query);
                        break;
                    } else if let Some(PlayerAction::Call) = last_action.action {
                        call_action(state, player, player_count, &mut last_action, text_query);
                        break;
                    }
                }
                else {
                    turn_text.sections[0].value = format!("It is player {}'s turn!\n", player.player_id);
                }    
            } else {
                state.current_player = (state.current_player + 1) % player_count.player_count;
                player.has_moved = true;
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
    mut player: &mut Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
) {
    let mut text_iter = text_query.iter_mut();
    let _money_text = text_iter.next();
    let mut turn_text = text_iter.next().unwrap();

    if state.current_top_bet > player.current_bet {
        turn_text.sections[1].value = format!("You cannot check");
        println!("Cannot check since top_bet ({}) is > your current bet ({})!", state.current_top_bet, player.current_bet);
        last_action.action = Some(PlayerAction::None);
    } else {
        turn_text.sections[1].value = format!("You have checked");
        println!("Player {} has checked!", player.player_id);
        player.has_moved = true;
        last_action.action = Some(PlayerAction::None);
        state.current_player = (state.current_player + 1) % player_count.player_count;
    }
}

pub fn raise_action (
    state: &mut ResMut<PokerTurn>,
    mut player: &mut Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
) -> bool {
    let mut text_iter = text_query.iter_mut();
    let mut money_text = text_iter.next().unwrap();
    let mut turn_text = text_iter.next().unwrap();

    if player.cash >= state.current_top_bet - player.current_bet {
        state.pot += state.current_top_bet - player.current_bet;
        println!("Player {} has raised the bet to {}", player.player_id, state.current_top_bet);
        turn_text.sections[1].value = format!("You raised the bet to {}", state.current_top_bet);

        player.has_moved = true;
        player.has_raised = true;
        player.cash -= state.current_top_bet - player.current_bet;
        player.current_bet = state.current_top_bet;
        money_text.sections[2].value = format!("Current Pot: ${}\n", state.pot);
        money_text.sections[3].value = format!("Current Top Bet: ${}\n", state.current_top_bet);
        money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
        money_text.sections[1].value = format!("Your Current Bet: ${}\n", player.current_bet);

        if player.cash == 0 {
            player.is_all_in = true;
            turn_text.sections[1].value = format!("You have gone all in!");
            println!("Player {} has gone all in!", player.player_id);
        }
        
        last_action.action = Some(PlayerAction::None);
        state.current_player = (state.current_player + 1) % player_count.player_count;
        return true;
    } else {

        turn_text.sections[1].value = format!("You cannot raise due to going negative");

        // This might be really complicated but since we set the current_top_bet before this function is called
        // this just pulls the previously set top bet by pulling from the text already set that does not get updated until the action is determined to be valid
        let section_value = &money_text.sections[3].value;
        if let Some(dollar_pos) = section_value.find('$') {
            let number_part = &section_value[dollar_pos + 1..].trim();
            let end_pos = number_part.find('\n').unwrap_or(number_part.len());
            let number_str = &number_part[..end_pos].trim();
            match number_str.parse::<usize>() {
                Ok(num) => {
                    state.current_top_bet = num;
                },
                Err(e) => {
                    eprintln!("Failed to parse number: {}", e);
                }
            }
        }

        last_action.action = Some(PlayerAction::None);

        return false;
    }
}

pub fn fold_action(
    state: &mut ResMut<PokerTurn>,
    mut player: &mut Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
) {
    let mut text_iter = text_query.iter_mut();
    let _money_text = text_iter.next();
    let mut turn_text = text_iter.next().unwrap();
    
    turn_text.sections[1].value = format!("You folded!");
    println!("Player {} has folded!", player.player_id);
    
    player.has_moved = true;
    player.has_folded = true;

    last_action.action = Some(PlayerAction::None);

    state.current_player = (state.current_player + 1) % player_count.player_count;
}

pub fn call_action(
    state: &mut ResMut<PokerTurn>,
    mut player: &mut Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
) {
    let mut text_iter = text_query.iter_mut();
    let mut money_text = text_iter.next().unwrap();
    let mut turn_text = text_iter.next().unwrap();

    if player.cash >= state.current_top_bet - player.current_bet {
        turn_text.sections[1].value = format!("You have called!");
        println!("Player {} has called!", player.player_id);
        player.has_moved = true;
        last_action.action = Some(PlayerAction::None);
        
        state.pot += state.current_top_bet - player.current_bet;
        player.cash -= state.current_top_bet - player.current_bet;
        player.current_bet = state.current_top_bet;

        money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
        money_text.sections[1].value = format!("Your Current Bet: ${}\n", player.current_bet);
        
        if player.cash == 0 {
            player.is_all_in = true;

            turn_text.sections[1].value = format!("You have gone all in!");
            println!("Player {} has gone all in!", player.player_id);
        }
        state.current_player = (state.current_player + 1) % player_count.player_count;
    } else {
        turn_text.sections[1].value = format!("You have gone all in!");
        println!("Player {} has gone all in!", player.player_id);
        player.has_moved = true;
        player.is_all_in = true;

        last_action.action = Some(PlayerAction::None);

        state.pot += player.cash;
        player.current_bet = player.cash + player.current_bet;
        player.cash = 0;
        state.current_player = (state.current_player + 1) % player_count.player_count;

        money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
        money_text.sections[1].value = format!("Your Current Bet: ${}\n", player.current_bet);

    }
    money_text.sections[2].value = format!("Current Pot: ${}\n", state.pot);
    money_text.sections[3].value = format!("Current Top Bet: ${}\n", state.current_top_bet);
}


// I have no idea what the methods below this comment do, ask garrett. I included them just in case

pub fn handle_keyboard(
    mut events: EventReader<KeyboardInput>,
    mut text_query: Query<&mut Text, With<TextBoxTag>>,
    mut char_events: EventReader<ReceivedCharacter>,
    text_input_query: Query<(Entity, &TextBox)>,
    children_query: Query<&Children>,
) {
    for (input_entity, textbox) in &text_input_query {
        if !textbox.active {
            continue;
        }

        for descendant in children_query.iter_descendants(input_entity) {
            if let Ok(mut text) = text_query.get_mut(descendant) {
                for event in char_events.iter() {
                    if !(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&event.char)) {
                        continue;
                    }

                    text.sections[0].value.push(event.char);
                }

                for event in events.iter() {
                    match event.key_code {
                        Some(KeyCode::Return) => {
                            if event.state.is_pressed() {
                                return;
                            }; // repeats for some reason without this
                            debug!("result = {}", text.sections[0].value);
                        }
                        Some(KeyCode::Back) => {
                            text.sections[0].value.pop();
                        }
                        _ => {} // produces a compile error without this
                    }
                }
            }
        }
    }
}

pub fn activate(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_query: Query<(Entity, &mut TextBox, &mut BackgroundColor)>,
) {
    // if a thingy is clicked, set it to active and make all the other ones inactive
    // idk if we have a color scheme or something so it's just gonna be kinda greyed out if inactive
    for (target_entity, interaction) in &interaction_query {
        debug!("{:?} ----- {:?}", target_entity, interaction);
        match *interaction {
            Interaction::Pressed => {
                for (entity, mut text_box, mut color) in &mut text_query {
                    if target_entity == entity {
                        // if this one was clicked, set it active and highlight it
                        *color = Color::WHITE.into();
                        text_box.active = true;
                    } else {
                        // darken and deactivate all the other ones
                        *color = Color::rgb(0.7, 0.7, 0.7).into();
                        text_box.active = false;
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn make_scrolly(mut commands: Commands, query: Query<(Entity, &TextBox), Added<TextBox>>) {
    /*
    aight so basically this pretty much only runs once
    it gets called every loop because its tied to the update event in mod.rs but Added<TextBox>
    is only nonempty once (at the beginning, after the text boxes are spawned)
    this is the easiest way i could think of to be able to run this query in order to loop over all
    the text boxes
    my b if this makes absolutely no sense and theres an easier way to do it
    */

    // why is box a reserved keyword
    for (entity, textbox) in &query {
        commands.entity(entity).insert(Interaction::None); // make it responsive to click interactions

        // make the area for the text to be in and identify it with the TextBoxTag component
        let text_area = commands
            .spawn((
                TextBundle {
                    text: Text {
                        linebreak_behavior: BreakLineOn::NoWrap,
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: textbox.text_style.clone(),
                        }],
                        ..default()
                    },
                    ..default()
                },
                TextBoxTag { id: textbox.id },
            ))
            .id();

        // define overflow behavior
        let overflow_fixer = commands
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::FlexEnd, // shove it all to the left
                    max_width: Val::Percent(100.),            // make it go all the way to the end
                    overflow: Overflow::clip(),               // cut it off so it ain't visible
                    ..default()
                },
                ..default()
            })
            .id();

        // add the s c r o l l e r to the textbox
        commands.entity(overflow_fixer).add_child(text_area);
        commands.entity(entity).add_child(overflow_fixer);
    }
}