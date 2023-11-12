use bevy::prelude::*;
use std::thread;

#[path = "./client.rs"]
mod client;

use super::components::*;
use crate::AppState;

pub fn on_entry(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands // Parent node containing the 3 button widgets (create, join, ip text input)
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(NBundle)
        .with_children(|parent| {
            // Join Server Button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(270.0),
                        height: Val::Px(140.0),
                        border: UiRect::all(Val::Px(4.0)),

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                    ..default()
                })
                .insert(JoinServerButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Join Server",
                        TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            // Text input button for server ip
            parent
                .spawn(NodeBundle {
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
                        "Input IP",
                        TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
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
                            ..default()
                        },
                    ));
                });
        });
}

pub fn fill_textboxes(
    query: Query<(Entity, &TextBox), Added<TextBox>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, _textbox) in &query {
        let child = commands
            .spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/Lato-Black.ttf"),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        }],
                        ..default()
                    },
                    ..default()
                },
                TextBoxTag {},
            ))
            .id();

        commands.entity(entity).add_child(child);
    }
}

pub fn client_on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Code is mostly similar to main games menu/uijust for easiness and to keep the style consistent //
    // Creating buttons - just one for now "Exit Server"
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(NBundle)
        .with_children(|parent| {
            //spawn the exit server button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(230.0),
                        width: Val::Px(330.0),
                        height: Val::Px(130.0),
                        border: UiRect::all(Val::Px(3.0)),

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,

                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                    ..default()
                })
                .insert(ExitServerButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit Server",
                        TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn client_on_update() {}

pub fn join_server_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<JoinServerButton>),
    >,

    // For finding and reading ip address textbox
    in_textboxes: Query<(Entity, &TextBox)>,
    mut in_textbox_tags: Query<&mut Text, With<TextBoxTag>>,
    in_children: Query<&Children>,

    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;

                app_state_next_state.set(AppState::OnlineClient);

                // TODO: find a better way to query the ip address textbox
                for (textbox, _entity) in &in_textboxes {
                    for descendant in in_children.iter_descendants(textbox) {
                        if let Ok(text) = in_textbox_tags.get_mut(descendant) {
                            let ip_address = text.sections[0].value.clone();
                            println!("Joining server with ip address {}", &text.sections[0].value);
                            thread::spawn(move || {
                                client::client_tick(ip_address);
                            });
                        } else {
                            println!("Error retrieving textbox data");
                        }

                        break;
                    }
                }
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

pub fn ip_textbox_button_interaction(
    in_textboxes: Query<(Entity, &TextBox)>,
    mut in_textbox_tags: Query<&mut Text, With<TextBoxTag>>,
    mut in_event_reader: EventReader<ReceivedCharacter>,
    in_children: Query<&Children>,
) {
    for (textbox, _entity) in &in_textboxes {
        for descendant in in_children.iter_descendants(textbox) {
            if let Ok(mut text) = in_textbox_tags.get_mut(descendant) {
                for event in in_event_reader.iter() {
                    // TODO: limit certain charcters
                    text.sections[0].value.push(event.char);
                }
            }
        }
    }
}

pub fn exit_server_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ExitServerButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    // Code again mostly similar to main game's button interactions to keep style
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;

                app_state_next_state.set(AppState::MainMenu);
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

pub fn remove_gui(mut commands: Commands, mut node_query: Query<Entity, With<NBundle>>) {
    let node = node_query.single_mut();
    commands.entity(node).despawn_recursive();
}
