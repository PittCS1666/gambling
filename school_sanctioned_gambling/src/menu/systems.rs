use bevy::prelude::*;
use super::components::*;
use crate::AppState;

pub fn load_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    spawn_background(&mut commands, &asset_server);
    spawn_buttons(&mut commands, &asset_server);
}

fn spawn_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("main_menu.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Menu);
}

fn spawn_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }).insert(NBundle)
        .with_children(|parent| {
            //spawn local game button
            parent.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(265.),
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
            }).insert(LocalButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Local Game",
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
                
            });

            //spawn the online game button
            parent.spawn(ButtonBundle{
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(407.5),
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
            }).insert(OnlineButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Online Game",
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });

            //spawn exit button
            parent.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(550.),
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
            }).insert(ExitButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Exit",
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        });
}

pub fn tear_down_menu(
    mut commands: Commands, 
    mut menu_query: Query<Entity, With<Menu>>, 
    mut node_query: Query<Entity, With<NBundle>>,) 
{

    let menu = menu_query.single_mut();
    commands.entity(menu).despawn_recursive();

    let node = node_query.single_mut();
    commands.entity(node).despawn_recursive();

}

pub fn local_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<LocalButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
                app_state_next_state.set(AppState::Options);
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

pub fn online_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<OnlineButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
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


pub fn exit_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<ExitButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
                app_state_next_state.set(AppState::Credits);
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