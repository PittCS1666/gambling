use bevy::prelude::*;
use std::collections::HashMap;
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
pub struct VisPlayerCards;

#[derive(Component)]
pub struct Player {
    pub player_id: usize,
    pub cards: Vec<Card>,
    pub cash: usize,
    pub current_bet: usize,
    pub has_folded: bool,
    pub has_moved: bool,
    pub is_all_in: bool,
    pub has_raised: bool,
    pub hand_strength: u16,
    pub move_dist: HashMap<u16, Vec<u16>>,
    pub is_big_blind: bool,
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

pub struct PokerTurn {
    pub current_player: usize,
    pub phase: PokerPhase,
    pub round_started: bool,
    pub pot: usize,
    pub current_top_bet: usize,
    pub pot_raised: bool,
    pub left_of_dealer: bool,
    pub bet_made: bool,
    pub all_checked: bool,
}
impl Resource for PokerTurn {
}

pub struct NumPlayers {
    pub player_count: usize
}
impl Resource for NumPlayers {
}

#[derive(Default, Debug)]
pub struct LastPlayerAction {
    pub action: Option<PlayerAction>,
}
impl Resource for LastPlayerAction {
}

#[derive(Debug)]
pub enum PlayerAction {
    Raise,
    Check,
    Fold,
    Call,
    None,
}