use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::text::BreakLineOn;

use super::components::*;
use crate::AppState;



pub fn load_options(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_ui(&mut commands, &asset_server);
    let results = OptionsResult {
        money_per_player: 500,
        small_blind_amount: 25,
        big_blind_amount: 50,
        num_players: 2,
        is_loaded_game: false,
    }; // these are gonna be the defaults I guess
    commands.insert_resource(results);
}

fn spawn_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).insert(NBundle)
        .with_children(|parent| {

            //spawn title text
            parent.spawn(TextBundle::from_section(
                "Options Menu",
                TextStyle {
                    font: asset_server.load("fonts/Lato-Black.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                })
            );

            // do all the text boxes
            let mut counter = 1;
            for label in [
                "small blind amount: ",
                "big blind amount: ",
                "starting money per player: ",
                "number of players (2-6): ",
                ] {
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    
                    parent.spawn(TextBundle::from_section(
                        label,
                        TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        }
                    ));
    
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(40.0),
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: Color::rgb(0.7, 0.7, 0.7).into(),
                            ..default()
                        },
                        TextBox {
                            text_style: TextStyle {
                                font: asset_server.load("fonts/Lato-Black.ttf"),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                            id: counter,
                            ..default()
                        },
                    ));
                });
                counter += 1;
            }

            // spawn local game button
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(230.0),
                    height: Val::Px(90.0),
                    border: UiRect::all(Val::Px(3.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    // center the button within its parent container
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                ..default()
            }).insert(PlayButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });

             // spawn load game button
             parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(230.0),
                    height: Val::Px(90.0),
                    border: UiRect::all(Val::Px(3.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    // center the button within its parent container
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                ..default()
            }).insert(LoadButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Load Game",
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        });
}

pub fn tear_down_options(
    mut commands: Commands, 
    mut node_query: Query<Entity, With<NBundle>>,) 
{
    let node = node_query.single_mut();
    commands.entity(node).despawn_recursive();
}

pub fn play_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ), (Changed<Interaction>, With<PlayButton>)>,
    mut text_query: Query<&mut Text, With<TextBoxTag>>,
    text_ent_query: Query<(Entity, &TextBox)>,
    children_query: Query<&Children>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut results: ResMut<OptionsResult>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;

                for (ent, input) in &text_ent_query {
                    for descendant in children_query.iter_descendants(ent) {
                        if let Ok(text) = text_query.get_mut(descendant) {
                            if text.sections[0].value == "" {
                                continue;
                            }
                            let value = text.sections[0].value.parse::<usize>().unwrap(); // should never panic
                            
                            match input.id {
                                1 => results.small_blind_amount = value,
                                2 => results.big_blind_amount = value,
                                3 => results.money_per_player = value,
                                4 => results.num_players = value,
                                _ => {},
                            }

                            // note this doesn't do any sanity checking yet!
                        }
                    }
                }
                app_state_next_state.set(AppState::LocalPlay);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.133, 0.188, 0.659).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgb(0.071, 0.141, 0.753).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}


pub fn load_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ), (Changed<Interaction>, With<LoadButton>)>,
    mut text_query: Query<&mut Text, With<TextBoxTag>>,
    text_ent_query: Query<(Entity, &TextBox)>,
    children_query: Query<&Children>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut results: ResMut<OptionsResult>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
                results.is_loaded_game = true;
                app_state_next_state.set(AppState::LocalPlay);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.133, 0.188, 0.659).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgb(0.071, 0.141, 0.753).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}


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
                    // if ['\u{8}', '\r'].contains(&event.char) { // backspace, carriage return (why windows, we dont use typewriters anymore ffs)
                    //     continue;
                    // }

                    // actually just ban everything except numbers 
                    // prolly gonna need to fix this when users have to pick a name
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

pub fn make_scrolly(
    mut commands: Commands,
    query: Query<(Entity, &TextBox), Added<TextBox>>,
) {
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
                        sections: vec![
                            TextSection {
                                value: "".to_string(),
                                style: textbox.text_style.clone(),
                            },
                        ],
                        ..default()
                    },
                    ..default()
                },
                TextBoxTag {
                    id: textbox.id.clone(),
                },
            ))
            .id();
        
        // define overflow behavior
        let overflow_fixer = commands
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::FlexEnd, // shove it all to the left
                    max_width: Val::Percent(100.), // make it go all the way to the end
                    overflow: Overflow::clip(), // cut it off so it ain't visible
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