use bevy::prelude::*;
use super::cards::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct CheckButton;

#[derive(Component)]
pub struct NBundle;

#[derive(Component)]
pub struct NCards;

#[derive(Component)]
pub struct PlayerCards {
    pub player_id: u8,
    pub cards: Vec<Card>,
}

#[derive(Component)]
pub struct CommunityCards {
    pub cards: Vec<Card>,
}