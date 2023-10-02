use bevy::prelude::*;
use super::components::*;
use crate::AppState;

pub fn load_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(Camera);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("game_screen.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Background);
    spawn_buttons(&mut commands, &asset_server);
}

fn spawn_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<CheckButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
                app_state_next_state.set(AppState::MainMenu);
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