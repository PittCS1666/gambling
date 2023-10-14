use bevy::prelude::*;
use super::components::*;
use crate::AppState;



pub fn load_options(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_buttons(&mut commands, &asset_server);
}

fn spawn_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
                    font_size: 50.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                })
            );

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
    ),
    (Changed<Interaction>, With<PlayButton>),
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