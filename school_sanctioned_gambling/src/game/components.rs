use bevy::prelude::*;
use super::cards::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct DealButton;

#[derive(Component)]
pub struct CheckButton;

#[derive(Component)]
pub struct CallButton;

#[derive(Component)]
pub struct RaiseButton;

#[derive(Component)]
pub struct FoldButton;

#[derive(Component)]
pub struct NBundle;

#[derive(Component)]
pub struct NCards;

#[derive(Component)]
pub struct Player {
    pub player_id: usize,
    pub cards: Vec<Card>,
    pub cash: usize,
    pub current_bet: usize,
    pub has_folded: bool,
    pub has_moved: bool,
}

#[derive(Component)]
pub struct CommunityCards {
    pub cards: Vec<Card>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PokerPhase {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

#[derive(Component)]
pub struct PokerTurn {
    pub current_player: usize,
    pub phase: PokerPhase,
}
impl Resource for PokerTurn {
}

#[derive(Component)]
pub struct NumPlayers {
    pub player_count: usize
}
impl Resource for NumPlayers {
}

pub struct PlayerTrigger;
impl Event for PlayerTrigger {
}