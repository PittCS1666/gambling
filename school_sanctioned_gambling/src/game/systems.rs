use bevy::prelude::*;
use super::components::*;


pub fn load_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("game_screen.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Background);
}

// this causes a bunch of unused code warnings for now

#[derive(Copy, Clone, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

#[derive(Copy, Clone)]
struct Card {
    card_id: u8, // unique card id: hearts 0-12, diamonds 13-25, spades 26-38, clubs 39-51
    suit: Suit,
    value: u8, // ace: 1, 2: 2, ..., 10: 10, jack: 11, queen: 12, king: 13
}

impl Card {
    fn new(card_id: u8, suit: Suit, value: u8) -> Card {
        Card {
            card_id,
            suit,
            value,
        }
    }

    fn to_string(&self) -> String {
        String::from(format!("Card({id}, {suit}, {value})", 
        id=self.card_id, value=self.value, suit={
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
}


fn init_cards() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    let mut total: u8 = 0;
    let suits: Vec<Suit> = vec![Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
    for suit in suits {
        for value in 0..13 {
            cards.push(Card::new(total, suit, value));
            total += 1;
        }
    }
    cards
}

fn display_cards(cards: Vec<Card>) {
    for card in cards {
        println!("{}", card.to_string());
    }
}

fn shuffle_cards(mut cards: Vec<Card>) {
    cards.shuffle(&mut thread_rng());        
}

fn deal_cards(player_count: u8, cards: Vec<Card>) -> Vec<Vec<Card>> {
    // this honestly needs like a player struct or something to make any sense
    let mut result: Vec<Vec<Card>> = Vec::with_capacity(player_count as usize);
    for i in 0..player_count-1 {
        let hand: Vec<Card> = vec![ cards.pop(), cards.pop() ];
        result.push(hand);
    }
    result
}

pub fn test_dealing() -> i32 {
    let cards: Vec<Card> = init_cards();
    display_cards(cards.clone());
    shuffle_cards(cards.clone());
    display_cards(cards.clone());
    deal_cards(2, cards.clone());
    0
}