use bevy::{prelude::*, window::PresentMode, transform};
use rand::thread_rng;
use rand::seq::SliceRandom;

const TITLE: &str = "School Sanctioned Gambling";
const WIN_WIDTH: f32 = 1280.;
const WIN_HEIGHT: f32 = 720.;

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



fn main() {
    let cards: Vec<Card> = init_cards();
    display_cards(cards);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Fifo,
                resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                title: TITLE.into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, next_slide)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let paths = vec!["matts_slide.png", "sams_slide.png", "garretts_slide.png", "marias_slide.png", "griffins_slide.png", "alexs_slide.png", "makyes_slide.png"];
    let mut timer = 0.;

    for path in paths {
        if timer == 0. {
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                ..default()
            });
        }
        else {
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            }).insert(SlideTimer(Timer::from_seconds(timer,TimerMode::Once)));
        }
        timer += 3.;
    }
}

fn next_slide(time: Res<Time>, mut timer: Query<(&mut SlideTimer, &mut Transform)>) {
    let mut position = 2.;

    for (mut timer, mut transform) in timer.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = position;
            position += 1.;   
        }
    }
    
}
