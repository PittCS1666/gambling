use bevy::prelude::*;
use bevy::app::AppExit;

use crate::AssetServer;
use super::components::*;
use std::thread;

#[path = "./server.rs"]
mod server;

pub fn on_entry(mut commands: Commands, asset_server: Res<AssetServer>)
{
    println!("Starting to create server");

    // TODO: possibly a better way to do this where we can keep the server in the same
    // thread but for now we have to give the server its own thread so it doesn't block the
    // bevy app and you can still interact with the window
    thread::spawn(move || {
        server::server_tick();
    });

    // Again the code is mostly similar to main games menu just for easiness and to keep the style consistent //
    // Creating buttons - just one for now "Destroy Server"
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

            //spawn the destroy server button
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
            }).insert(DestroyServerButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Destroy Server",
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        });
}

pub fn server_on_update(_commands: Commands, _asset_server: Res<AssetServer>)
{
    
}

pub fn destroy_server_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<DestroyServerButton>),
    >,
    mut app: ResMut<Events<AppExit>>
) {
    // Code again mostly similar to main game's button interactions to keep style
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;

                // TODO: exit server should maybe just destroy the current server and
                // allow for a new server to be created but for simplicity right now
                // this button will just kill the program
                app.send(bevy::app::AppExit);
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