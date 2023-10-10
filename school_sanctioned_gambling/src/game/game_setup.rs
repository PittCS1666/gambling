use bevy::prelude::*;
use super::components::*;
use super::cards::*; 
use super::buttons::*;

pub fn load_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(init_cards_resource());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("game_screen.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Background);
    spawn_buttons(&mut commands, &asset_server);
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
    //commands.entity(exit_button).despawn_recursive();
}