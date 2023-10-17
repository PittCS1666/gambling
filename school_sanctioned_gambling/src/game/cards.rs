use super::components::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use bevy::prelude::*;
use super::hand_evaluation::*;
pub struct Deck {
    cards: Vec<Card>
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

fn init_cards() -> Vec<Card> {
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

fn shuffle_cards(cards: &mut Vec<Card>) {
    cards.shuffle(&mut thread_rng());        
}

pub fn deal_hands(player_count: u8, cards: &mut Vec<Card>) -> Vec<PlayerCards> {
    let mut result: Vec<PlayerCards> = Vec::with_capacity(player_count as usize);
    for player_id in 0..player_count {
        let hand: Vec<Card> = cards.drain(0..2).collect();
        result.push(PlayerCards { player_id, cards: hand });
    }
    result
}

fn deal_community_cards(cards: &mut Vec<Card>, community_query: &Query<&CommunityCards>,) -> Vec<Vec<Card>> {
    let mut result: Vec<Vec<Card>> = Vec::with_capacity(5);
    if community_query.iter().count() == 0 {
        let flop: Vec<Card> = cards.drain(0..3).collect();
        result.push(flop);
    } else if community_query.iter().count() == 3 {
        let river = cards.drain(0..1).collect();
        result.push(river)
    } else if community_query.iter().count() == 4 {
        let turn = cards.drain(0..1).collect();
        result.push(turn)
    }
    result
}

pub fn deal_cards(commands: &mut Commands, asset_server: &Res<AssetServer>, community_query: &Query<&CommunityCards>, player_card_query: &Query<&PlayerCards>, deck: &mut Deck) {
    let cards = &mut deck.cards;
    if player_card_query.is_empty() {
        shuffle_cards(cards);
        let hands = deal_hands(2, cards);
        spawn_player_cards(commands, &asset_server, hands);
    } else if community_query.iter().count() < 5 {
        let flop = deal_community_cards(cards, community_query);
        spawn_community_cards(commands, &asset_server, flop, community_query);
    }
}

pub fn card_function(
    community_query: &Query<&CommunityCards>,
    player_card_query: &Query<&PlayerCards>,
) {
    // Get the community cards; assuming there's only one CommunityCards component
    let community_cards: Vec<Card> = community_query.iter().flat_map(|cards| &cards.cards).cloned().collect();
    let mut hand1: Hand = Hand::_new_blank();
    let mut hand2: Hand = Hand::_new_blank();
    // Iterate through each player
    for player_cards_component in player_card_query.iter() {
        let player_cards = &player_cards_component.cards;

        

        // Ensure there are at least 5 cards between the player and community cards before evaluation
        if player_cards.len() + community_cards.len() >= 5 {
            let hand = test_evaluator(player_cards_component.player_id, player_cards.to_vec(), community_cards.to_vec());
            if player_cards_component.player_id == 0 {
                hand1 = hand;
            }
            else {
                hand2 = hand;
            }
        }
    }
    
    let comparison = compare_hands(&mut hand1, &mut hand2);
    if comparison == 1 {
        println!("Player 0 wins with a score of {}\n", hand1.score);
    }
    else if comparison == 2 {
        println!("Player 1 wins with a score of {}\n", hand2.score);
    }
    else {
        println!("It's a draw!\n");
    }
}

fn spawn_player_cards(commands: &mut Commands, asset_server: &Res<AssetServer>, hands: Vec<PlayerCards>) {
    for hand in hands {
        let top_shift = 690. - (90. * ((hand.player_id as f32) + 1.));
        // Create a single PlayerCards entity for each player with both cards
        commands
            .spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(top_shift),
                    left: Val::Px(820.), // Adjust this value to position the PlayerCards entity as needed
                    width: Val::Px(460.0), // Adjust the width to accommodate both cards
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
            .insert(PlayerCards {
                player_id: hand.player_id,
                cards: hand.cards.clone(), // Insert the entire hand here
            })
            .with_children(|parent| {
                for (index, card) in hand.cards.iter().enumerate() {
                    let left_shift = 10. + 230. * (index as f32); // Shift the card text horizontally based on index
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
}


fn spawn_community_cards(commands: &mut Commands, asset_server: &Res<AssetServer>, com_cards: Vec<Vec<Card>>, community_query: &Query<&CommunityCards>) {
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
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
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