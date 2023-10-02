<<<<<<< HEAD
use bevy::{prelude::*, window::PresentMode, transform};
use rand::thread_rng;
use rand::seq::SliceRandom;
=======
use bevy::{prelude::*, window::PresentMode};

mod credits;
mod menu;
mod game;

use game::*;
use menu::*;
use credits::*;
>>>>>>> upstream/main

const TITLE: &str = "School Sanctioned Gambling";
const WIN_WIDTH: f32 = 1280.;
const WIN_HEIGHT: f32 = 720.;

<<<<<<< HEAD
#[derive(Component, Deref, DerefMut)]
struct SlideTimer(Timer);

#[derive(Copy, Clone, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

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

=======
>>>>>>> upstream/main

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    LocalPlay,
    OnlinePlay,
    Credits,
}

fn main() {
    let cards: Vec<Card> = init_cards();
    display_cards(cards);

    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Fifo,
                    resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                    title: TITLE.into(),
                    ..default()
                }),
                ..default()
            }),
            MenuPlugin,
            CreditsPlugin,
            GamePlugin,
        ))
        .run();
}