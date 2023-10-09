use bevy::prelude::*;
use super::components::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

pub fn load_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(init_cards_resource());
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
    mut node_query: Query<Entity, With<NBundle>>,) 
{
    let node = node_query.single_mut();

    commands.entity(node).despawn_recursive();

    let background = background_query.single_mut();
    
    commands.entity(background).despawn_recursive();
    //commands.entity(exit_button).despawn_recursive();
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

pub fn test_evaluator() {
    let card1 = Card::new(0, Suit::Hearts, 2);
    let card2 = Card::new(1, Suit::Spades, 2);
    let card3 = Card::new(2, Suit::Clubs, 3);
    let card4 = Card::new(3, Suit::Diamonds, 3);
    let card5 = Card::new(4, Suit::Hearts, 3);
    let card6 = Card::new(5, Suit::Diamonds, 2);
    let card7 = Card::new(6, Suit::Hearts, 4);

    let cards = vec![card1, card2, card3, card4, card5, card6, card7];
    let hand = find_best_hand(&cards);
    println!("{:?}", Hand::to_string(&hand));
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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

#[derive(Copy, Clone)]
struct Card {
    _card_id: u8, // unique card id: hearts 0-12, diamonds 13-25, spades 26-38, clubs 39-51
    suit: Suit,
    value: u8, // ace: 1, 2: 2, ..., 10: 10, jack: 11, queen: 12, king: 13
}

impl Card {
    fn new(_card_id: u8, suit: Suit, value: u8) -> Card {
        Card {
            _card_id,
            suit,
            value,
        }
    }

    fn to_string(&self) -> String {
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

    fn copy(card: &Card) -> Card {
        let new_card = Card::new(card._card_id, card.suit, card.value);
        return new_card
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
    // println!("Amount of cards left: {}", cards.len());
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


struct Hand {
    cards: Vec<Card>,   //a vector with the 5 cards in a hand
    ranks: Vec<u8>,     //a vector that holds the number of each rank that is in the hand
    suits: u8,          //how many suits are present in the hand
    score: u8           //the final score of the hand -> 8: Straight flush, 7: Four of a kind, 6: Full house, 5: flush, 4: straight, 3: three of a kind, 2: two pair, 1: pair, 0: high card
}


impl Hand {
    fn new_blank() -> Hand {
        Hand {
            cards: Vec::new(),
            ranks: vec![0; 13],
            suits: 0,
            score: 0,
        }
    }
    fn new(cards: Vec<Card>) -> Hand{
        let mut suits: HashSet<Suit> = HashSet::new();
        let mut ranks: Vec<u8> = vec![0; 13];
        let score: u8 = 0;
        for &card in &cards {
            suits.insert(card.suit);
            ranks[usize::from(card.value - 1)] += 1;
        }
        Hand {
            cards,
            ranks,
            suits: u8::try_from(suits.len()).unwrap(),
            score,
        }
    }

    fn to_string(hand: &Hand) -> String {
        let card1 = Card::to_string(&hand.cards[0]);
        let card2 = Card::to_string(&hand.cards[1]);
        let card3 = Card::to_string(&hand.cards[2]);
        let card4 = Card::to_string(&hand.cards[3]);
        let card5 = Card::to_string(&hand.cards[4]);
        
        String::from(format!("{card1}, {card2}, {card3}, {card4}, and {card5} with a score of {score}",
            card1 = card1, card2 = card2, card3 = card3, card4 = card4, card5 = card5, score = hand.score.to_string()))
    }
}

fn find_best_hand(cards: &Vec<Card>) -> Hand{
    //there might be a library to get all combinations easily
    let combinations: Vec<Vec<Card>> = get_all_hands(cards);

    let mut best_hand_val = 0;
    let mut best_hand: Hand = Hand::new_blank();
    
    for combination in combinations {
        let mut cur_hand = Hand::new(combination);
        //println!("Current hand: {:?}", Hand::to_string(&cur_hand));
        evaluate_hand(&mut cur_hand);
        if cur_hand.score > best_hand_val {
            best_hand_val = cur_hand.score;
            best_hand = cur_hand
        }
        else if cur_hand.score == best_hand_val {
            let res = compare_hands(&mut cur_hand, &mut best_hand);
            if res == 1 {
                best_hand_val = cur_hand.score;
                best_hand = cur_hand
            }
        }
        //println!("Best hand: {:?}", Hand::to_string(&best_hand));
        
    }

    return best_hand;
}


fn get_all_hands(cards: &Vec<Card>) -> Vec<Vec<Card>> {
    let mut combinations: Vec<Vec<Card>> = Vec::new();
    
    for i in 0..cards.len() {
        for j in (i + 1)..cards.len() {
            for k in (j + 1)..cards.len() {
                for l in (k + 1)..cards.len() {
                    for m in (l + 1)..cards.len() {
                        let combination = vec![Card::copy(&cards[i]), Card::copy(&cards[j]), Card::copy(&cards[k]), Card::copy(&cards[l]), Card::copy(&cards[m])];
                        combinations.push(combination)
                    }
                }
            }
        }
    }
    return combinations
}

fn evaluate_hand(hand: &mut Hand) {
    let is_flush = if hand.suits == 1 {
        true
    }
    else {
        false
    };
    let is_straight = is_straight(&hand.ranks);
    let mut is_four = false;
    let mut is_three = false;
    let mut is_two_pair = false;
    let mut is_pair = false;

    for i in 0..hand.ranks.len() {
        if hand.ranks[i] == 4 {
            is_four = true;
        }
        else if hand.ranks[i] == 3 {
            is_three = true;
        }
        else if hand.ranks[i] == 2 && is_pair {
            is_two_pair = true;
        }
        else if hand.ranks[i] == 2 {
            is_pair = true;
        }
    }

    if is_straight && is_flush {    //straight flush
        hand.score = 8;
    }
    else if is_four {  //four of a kind
       hand.score = 7;
    }
    else if is_three && is_pair {  //full house
        hand.score = 6;
    }
    else if is_flush {  //flush
        hand.score = 5;
    }
    else if is_straight {  //straight
        hand.score = 4;
    }
    else if is_three {  //three of a kind
        hand.score = 3;
    }
    else if is_two_pair {  //two pair
        hand.score = 2;
    }
    else if is_pair {  //pair
        hand.score = 1;
    }
    else {  //high card
        hand.score = 0;
    }

}

fn is_straight(ranks: &Vec<u8>) -> bool {
    let mut min_rank: usize = 14;
    let mut max_rank: usize = 0;
    let ace = ranks[0];
    let mut unique_ranks: u8 = 0;

    for i in 0..ranks.len() {
        if ranks[i] > 0 {
            unique_ranks += 1;

            if i > max_rank {
                max_rank = i;
            }
            
            if i < min_rank {
                min_rank = i;
            }
        }
    }

    if (max_rank - min_rank) == 4 && unique_ranks == 5 {
        true
    }
    else if ace > 0{
        if (max_rank - min_rank) == 3 && unique_ranks == 5 && (max_rank == 12 || min_rank == 1) {
            true
        }
        else {
            false
        }
    }
    else {
        false
    }
}

//returns 1 if hand1 > hand2, 2 if hand2 > hand1, and 0 if they are equal
fn compare_hands(hand1: &mut Hand, hand2: &mut Hand) -> u8 {
    let score1 = hand1.score;
    let score2 = hand2.score;

    if score1 > score2 {
        return 1;
    }
    else if score2 > score1 {
        return 2;
    }
    else {
        let mut ranks1: Vec<usize> = Vec::new();
        let mut ranks2: Vec<usize> = Vec::new();

        for i in 0..hand1.ranks.len() {
            if hand1.ranks[i] > 0 {
                ranks1.append(&mut vec![i]);
            }
            if hand2.ranks[i] > 0 {
                ranks2.append(&mut vec![i]);
            }
        }

        ranks1.reverse();
        ranks2.reverse();
        for i in 0..ranks2.len() {
            if ranks1[i] > ranks2[i] {
                return 1;
            }
            else if ranks2[i] > ranks1[i] {
                return 2;
            }
        }
    }
    return 0;
}