use super::game_state::*;
use shrev::{EventChannel, ReaderId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerUpdateEvent(pub Player);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokerTurnUpdateEvent(pub PokerTurn);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCardsUpdateEvent(pub CommunityCards);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastPlayerActionUpdateEvent(pub LastPlayerAction);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumPlayersUpdateEvent(pub NumPlayers);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokerPhaseUpdateEvent(pub PokerPhase);

pub fn init_player_update_channel() -> (EventChannel<PlayerUpdateEvent>, ReaderId<PlayerUpdateEvent>) {
    let event_channel = EventChannel::new();
    let reader_id = event_channel.register_reader();
    (event_channel, reader_id)
}

pub fn init_poker_turn_update_channel() -> (EventChannel<PokerTurnUpdateEvent>, ReaderId<PokerTurnUpdateEvent>) {
    let event_channel = EventChannel::new();
    let reader_id = event_channel.register_reader();
    (event_channel, reader_id)
}

pub fn init_community_cards_update_channel() -> (EventChannel<CommunityCardsUpdateEvent>, ReaderId<CommunityCardsUpdateEvent>) {
    let event_channel = EventChannel::new();
    let reader_id = event_channel.register_reader();
    (event_channel, reader_id)
}

pub fn init_last_player_action_update_channel() -> (EventChannel<LastPlayerActionUpdateEvent>, ReaderId<LastPlayerActionUpdateEvent>) {
    let event_channel = EventChannel::new();
    let reader_id = event_channel.register_reader();
    (event_channel, reader_id)
}

pub fn init_num_players_update_channel() -> (EventChannel<NumPlayersUpdateEvent>, ReaderId<NumPlayersUpdateEvent>) {
    let event_channel = EventChannel::new();
    let reader_id = event_channel.register_reader();
    (event_channel, reader_id)
}

pub fn init_poker_phase_update_channel() -> (EventChannel<PokerPhaseUpdateEvent>, ReaderId<PokerPhaseUpdateEvent>) {
    let event_channel = EventChannel::new();
    let reader_id = event_channel.register_reader();
    (event_channel, reader_id)
}
