use bevy::prelude::*;
use super::components::*;
// use crate::AppState;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn load_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(init_cards_resource());
    commands.spawn(Camera2dBundle::default()).insert(Camera);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("game_screen.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Background);
    spawn_buttons(&mut commands, &asset_server);
}

pub fn spawn_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }).insert(NBundle)
        .with_children(|parent| {
            //spawn check button
            parent.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(600.),
                    width: Val::Px(230.0),
                    height: Val::Px(90.0),
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
            }).insert(CheckButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Check",
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        });
}

pub fn tear_down_game_screen(
    mut commands: Commands, 
    mut background_query: Query<Entity, With<Background>>, 
    mut button_query: Query<Entity, With<Button>>,
    mut node_query: Query<Entity, With<NBundle>>,
    mut camera_query: Query<Entity, With<Camera>>,) 
{
    for button in &mut button_query {
        commands.entity(button).despawn_recursive();
    }

    let node = node_query.single_mut();

    commands.entity(node).despawn_recursive();

    let background = background_query.single_mut();
    
    commands.entity(background).despawn_recursive();
    //commands.entity(exit_button).despawn_recursive();

    let camera_entity = camera_query.single_mut();
    commands.entity(camera_entity).despawn_recursive();
}

pub fn check_button_interaction(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<CheckButton>),
    >,
    player_card_query: Query<&PlayerCards>,
    community_query: Query<&CommunityCards>,
    mut deck: ResMut<Deck>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                deal_cards(&mut commands, &asset_server, &community_query, &player_card_query, &mut deck,);
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.133, 0.188, 0.659).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgb(0.071, 0.141, 0.753).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}


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
        let card_value = if self.value < 11 && self.value > 1 {
            let card_value_str = self.value.to_string();
            card_value_str
        } else if self.value == 11 {
            "Jack of".to_string()
        } else if self.value == 12 {
            "Queen of".to_string()
        } else if self.value == 13 {
            "King of".to_string()
        } else {
            "Ace of".to_string()
        };

        String::from(format!("{value} {suit}", 
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

fn shuffle_cards(cards: &mut Vec<Card>) {
    cards.shuffle(&mut thread_rng());        
}

fn deal_hands(player_count: u8, cards: &mut Vec<Card>) -> Vec<Vec<Card>> {
    let mut result: Vec<Vec<Card>> = Vec::with_capacity(player_count as usize);
    for _ in 0..player_count {
        let hand: Vec<Card> = cards.drain(0..2).collect();
        result.push(hand);
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

fn deal_cards(commands: &mut Commands, asset_server: &Res<AssetServer>, community_query: &Query<&CommunityCards>, player_card_query: &Query<&PlayerCards>, deck: &mut Deck) {
    let cards = &mut deck.cards;
    if player_card_query.is_empty() {
        shuffle_cards(cards);
        let hands = deal_hands(1, cards);
        spawn_player_cards(commands, &asset_server, hands);
    }else if community_query.iter().count() < 5 {
        let flop = deal_community_cards(cards, community_query);
        spawn_community_cards(commands, &asset_server, flop, community_query);
    }
    println!("Amount of cards left: {}", cards.len());
}

fn spawn_player_cards(commands: &mut Commands, asset_server: &Res<AssetServer>, hands: Vec<Vec<Card>>) {
    for hand in hands {
        for (index, card) in hand.iter().enumerate() {
            let left_shift = 1280. - (((index as f32) + 1.) * 230.);
            commands.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(600.),
                    left: Val::Px(left_shift),
                    width: Val::Px(230.0),
                    height: Val::Px(90.0),
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
            }).insert(PlayerCards)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    card.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 35.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        }
    }
}

fn spawn_community_cards(commands: &mut Commands, asset_server: &Res<AssetServer>, com_cards: Vec<Vec<Card>>, community_query: &Query<&CommunityCards>) {
    for cards in com_cards {
        for (index,card) in cards.iter().enumerate() {
            let left_shift = 364. + ((((community_query.iter().count() as f32) + 1.) * ((index  as f32) + 1.)) * 81.);
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
            }).insert(CommunityCards)
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