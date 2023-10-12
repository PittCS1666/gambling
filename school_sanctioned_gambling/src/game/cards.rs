use super::components::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use bevy::prelude::*;
use super::hand_evaluation::*;
pub struct Deck {
    pub cards: Vec<Card>
}

pub fn init_cards_resource() -> Deck {
    Deck {
        cards: init_cards(),
    }
}
impl Resource for Deck {
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

#[derive(Copy, Clone)]
pub struct Card {
    pub _card_id: u8, // unique card id: hearts 0-12, diamonds 13-25, spades 26-38, clubs 39-51
    pub suit: Suit,
    pub value: u8, // ace: 1, 2: 2, ..., 10: 10, jack: 11, queen: 12, king: 13
}

impl Card {
    fn new(_card_id: u8, suit: Suit, value: u8) -> Card {
        Card {
            _card_id,
            suit,
            value,
        }
    }

    pub fn to_string(&self) -> String {
        let card_value = if self.value < 11 && self.value > 1 {
            let card_value_str = self.value.to_string();
            card_value_str
        } else if self.value == 11 {
            "Jack".to_string()
        } else if self.value == 12 {
            "Queen".to_string()
        } else if self.value == 13 {
            "King".to_string()
        } else {
            "Ace".to_string()
        };

        String::from(format!("{value} of {suit}", 
        value={card_value}
        ,suit={
            if self.suit == Suit::Hearts {
                "Hearts"
            } else if self.suit == Suit::Diamonds {
                "Diamonds"
            } else if self.suit == Suit::Spades {
                "Spades"
            } else {
                "Clubs"
            }
        }))
    }

    pub fn copy(card: &Card) -> Card {
        let new_card = Card::new(card._card_id, card.suit, card.value);
        return new_card
    }
}

pub fn init_cards() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    let mut total: u8 = 0;
    let suits: Vec<Suit> = vec![Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
    for suit in suits {
        for value in 1..14 {
            cards.push(Card::new(total, suit, value));
            total += 1;
        }
    }
    cards
}

pub fn shuffle_cards(cards: &mut Vec<Card>) {
    cards.shuffle(&mut thread_rng());        
}

pub fn deal_hands(player_count: usize, cards: &mut Vec<Card>) -> Vec<Player> {
    let mut result: Vec<Player> = Vec::with_capacity(player_count as usize);
    for player_id in 0..player_count {
        let hand: Vec<Card> = cards.drain(0..2).collect();
        result.push(Player { player_id, cards: hand, cash: 500, current_bet: 0, has_folded: false, has_moved: false});
    }
    result
}

pub fn deal_com_function(cards: &mut Vec<Card>, community_query: &Query<&CommunityCards>,) -> Vec<Vec<Card>> {
    let mut result: Vec<Vec<Card>> = Vec::with_capacity(5);
    if community_query.iter().count() == 0 {
        let flop: Vec<Card> = cards.drain(0..3).collect();
        result.push(flop);
    } else if community_query.iter().count() == 3 {
        let turn = cards.drain(0..1).collect();
        result.push(turn)
    } else if community_query.iter().count() == 4 {
        let river = cards.drain(0..1).collect();
        result.push(river)
    }
    result
}

pub fn card_function(
    community_query: &Query<&CommunityCards>,
    players: &Query<&Player>,
) -> Vec<usize> {
    let community_cards: Vec<Card> = community_query.iter().flat_map(|cards| &cards.cards).cloned().collect();
    
    let mut player_hands: Vec<(usize, Hand)> = Vec::with_capacity(players.iter().count());
    for player in players.iter() {
        let player_id = player.player_id;
        let player_cards = &player.cards;

        if player_cards.len() + community_cards.len() >= 5 {
            let play_hand = test_evaluator(player.player_id, player_cards.to_vec(), community_cards.to_vec());
            player_hands.push((player_id, play_hand));
        }
    }

    let mut highest_score = 0;
    let mut highest_scoring_players = Vec::new();

    for (player_id, hand) in &player_hands {
        let score = hand.score;
        if score > highest_score {
            highest_scoring_players.clear();
            highest_scoring_players.push(*player_id);
            highest_score = score;
        } else if score == highest_score {
            // Found another hand with the highest score, add the player_id to the list
            highest_scoring_players.push(*player_id);
        }
    }

    highest_scoring_players
}

pub fn spawn_player_cards(commands: &mut Commands, asset_server: &Res<AssetServer>, players: &Vec<Player>) {
    for player in players {
        commands.spawn(Player {
            player_id: player.player_id,
            cards: player.cards.clone(),
            cash: player.cash,
            current_bet: player.current_bet,
            has_folded: player.has_folded,
            has_moved: player.has_moved,
        });

        if player.player_id == 0 {
            let top_shift = 690. - (90. * ((player.player_id as f32) + 1.));
            commands
                .spawn(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(top_shift),
                        left: Val::Px(820.),
                        width: Val::Px(460.0),
                        height: Val::Px(90.0),
                        border: UiRect::all(Val::Px(3.0)),
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    for (index, card) in player.cards.iter().enumerate() {
                        let left_shift = 10. + 230. * (index as f32);
                        parent.spawn(TextBundle::from_section(
                            card.to_string(),
                            TextStyle {
                                font: asset_server.load("fonts/Lato-Black.ttf"),
                                font_size: 30.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .insert(Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(left_shift),
                            ..Default::default()
                        });
                    }
                });
        }
            commands.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("Cash: ${}", player.cash),
                            style: TextStyle {
                                font: asset_server.load("fonts/Lato-Black.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }
                    ],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
                },
                ..Default::default()
            });
        }
}

pub fn spawn_community_cards(commands: &mut Commands, asset_server: &Res<AssetServer>, com_cards: Vec<Vec<Card>>, community_query: &Query<&CommunityCards>) {
    for cards in com_cards {
        for (index,card) in cards.iter().enumerate() {
            let left_shift = 368. + ((((community_query.iter().count() as f32) + 1.) * ((index  as f32) + 1.)) * 81.);
            commands.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(317.),
                    left: Val::Px(left_shift),
                    width: Val::Px(58.0),
                    height: Val::Px(93.0),
                    border: UiRect::all(Val::Px(3.0)),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                ..default()
            }).insert(CommunityCards {cards: vec![card.clone()],})
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    card.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 13.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        }
    }
}