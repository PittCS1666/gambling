use bevy::prelude::*;
use super::components::*;

pub fn spawn_option_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let button_texts = vec!["Check", "Call", "Raise", "Fold"];
    let button_width = 150.0;
    let button_spacing = 10.0;
    
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(NBundle)
    .with_children(|parent| {
        for (index, &text) in button_texts.iter().enumerate() {
            let mut individual_button_entity = parent.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(600.),
                    left: Val::Px((index as f32) * (button_width + button_spacing)),
                    width: Val::Px(button_width),
                    height: Val::Px(90.0),
                    border: UiRect::all(Val::Px(3.0)),
                    align_self: AlignSelf::FlexStart,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                ..Default::default()
            });
    
            match text {
                "Check" => individual_button_entity.insert(CheckButton),
                "Call" => individual_button_entity.insert(CallButton),
                "Raise" => individual_button_entity.insert(RaiseButton),
                "Fold" => individual_button_entity.insert(FoldButton),
                _ => panic!("Unknown button text: {}", text),
            };

            individual_button_entity.with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        }
    });

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
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            top: Val::Px(215.0),
                            left: Val::Px(-565.0),
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
                        id: 1,
                        ..default()
                    },
                ));
            });
        });
}

pub fn check_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<CheckButton>),
    >,
    player_entity_query: Query<(Entity, &mut Player)>,
    state: ResMut<PokerTurn>,
    mut last_action: ResMut<LastPlayerAction>,
)   {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for (_, player) in player_entity_query.iter() {
                    if player.player_id == 0 && state.current_player == 0 {
                        last_action.action = Some(PlayerAction::Check);
                    }
                }
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
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

pub fn raise_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<RaiseButton>),
    >,
    player_entity_query: Query<(Entity, &mut Player)>,
    mut state: ResMut<PokerTurn>,
    mut text_query: Query<&mut Text, With<TextBoxTag>>,
    mut last_action: ResMut<LastPlayerAction>,
)   {

    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {

                for (_, player) in player_entity_query.iter() {
                    if player.player_id == 0 && state.current_player == 0 {
                        for mut text in text_query.iter_mut() {
                            if let Ok(parsed_value) = text.sections[0].value.parse::<usize>() {
                                if parsed_value > 0 {
                                    state.current_top_bet += parsed_value;
                                    println!("Current top bet is now: ${}", state.current_top_bet);
                                    last_action.action = Some(PlayerAction::Raise);
                                } else {
                                    println!("Have to raise by more than 0!");
                                }
                            } else {
                                println!("Not a valid raise");
                            }
                            text.sections[0].value.clear();
                        }
                    }
                }
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
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

pub fn fold_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<FoldButton>),
    >,
    player_entity_query: Query<(Entity, &mut Player)>,
    state: ResMut<PokerTurn>,
    mut last_action: ResMut<LastPlayerAction>,
)   {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for (_, player) in player_entity_query.iter() {
                    if player.player_id == 0 && state.current_player == 0 {
                        last_action.action = Some(PlayerAction::Fold);
                    }
                }
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
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

pub fn call_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<CallButton>),
    >,
    player_entity_query: Query<(Entity, &mut Player)>,
    state: ResMut<PokerTurn>,
    mut last_action: ResMut<LastPlayerAction>,
)   {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for (_, player) in player_entity_query.iter() {
                    if player.player_id == 0 && state.current_player == 0 {
                        last_action.action = Some(PlayerAction::Call);
                    }
                }
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
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