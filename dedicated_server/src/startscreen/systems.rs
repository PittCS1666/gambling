use bevy::prelude::*;
use super::components::*;
use crate::AppState;

pub fn on_entry(mut commands: Commands, asset_server: Res<AssetServer>)
{
    // Code mostly similar to main games menu just for easiness and to keep the style consistent //
    commands.spawn(Camera2dBundle::default());

    // Creating background
    commands.spawn(SpriteBundle {
        texture: asset_server.load("main_menu.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(StartScreen);

    // Creating buttons - just one for now "Create New Server"
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

            //spawn the online game button
            parent.spawn(ButtonBundle{
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
            }).insert(CreateServerButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Create New Server",
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        });
}

pub fn create_server_button_interaction(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<CreateServerButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut commands: Commands, 
    mut start_screen_query: Query<Entity, With<StartScreen>>, 
    mut node_query: Query<Entity, With<NBundle>>
) {
    // Code again mostly similar to main game's button interactions to keep style
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;

                // Removing ui interface manually here
                let start_screen = start_screen_query.single_mut();
                commands.entity(start_screen).despawn_recursive();
            
                let node = node_query.single_mut();
                commands.entity(node).despawn_recursive();

                // Set new app state so we know a server has been created
                app_state_next_state.set(AppState::ServerRunning);
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