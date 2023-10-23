use std::collections::HashMap;
use super::cards::*;
use super::components::*;
use rand::Rng;

/*
//Declaring Player Struct
#[derive(Clone, Debug)]
struct Player{
    hand: Vec<Card>,
    hand_strength: u16,
    move_dist: HashMap<u16, Vec<u16>>,
}

impl Player{
    //hand clone will be copy of hand bc structs cannot be self-referantial...I think
    fn new(hand: Vec<Card>, hand_clone: &Vec<Card>, move_dist: HashMap<u16, Vec<u16>>) -> Self{
        Player{
            hand,
            hand_strength: generate_hand_strength(hand_clone),
            move_dist,
        }
    }
}*/



//Stealing this for now 
//Adding card_strength field
/*
#[derive(Copy, Clone, Debug)]
struct Card {
    card_id: u8, // unique card id: hearts 0-12, diamonds 13-25, spades 26-38, clubs 39-51
    suit: Suit,
    value: u8, // ace: 1, 2: 2, ..., 10: 10, jack: 11, queen: 12, king: 13
    card_strength: u8
}

//Added a card_strength field
impl Card {
    fn new(card_id: u8, suit: Suit, value: u8) -> Card {
        Card {
            card_id: card_id,
            suit: suit,
            value: value,
            card_strength: generate_card_strength(value),
        }
    }
}*/

//Stolen from systems
/*
#[derive(Copy, Clone, PartialEq, Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}*/

//Stolen from moveset
/*
#[derive(Copy, Clone, PartialEq, Debug)]
enum MoveSet {
    Fold,
    Check,
    Bet,
    Call, 
    Raise,
}*/
//Simply sets Ace to strongest card. All others remain the same
pub fn generate_card_strength(val:u8) -> u8{
    if val == 1{
        14
    }else{
        val
    }
}

//Generates hand strength from starting hand
pub fn generate_hand_strength(vec_hand: &Vec<Card>) -> u16{
    vec_hand[0].card_strength as u16 + vec_hand[1].card_strength as u16
}

//Checks rand number w/in ranges to determine move
pub fn generate_move(player: &Player, poker_turn: PokerTurn) -> String{
    //Check for poker phase
    let mut num = 101;
    let chosen_dist = player.move_dist.get(&player.hand_strength);

    if poker_turn.phase == PokerPhase::PreFlop{
        if player.is_big_blind && !poker_turn.pot_raised {
            let num = rand::thread_rng().gen_range(0..=100);
        }else{
            let num = rand::thread_rng().gen_range(chosen_dist.unwrap()[0]..=100);
        }
    }else if poker_turn.phase == PokerPhase::Flop || poker_turn.phase == PokerPhase::Turn || poker_turn.phase == PokerPhase::River{
        if !poker_turn.bet_made{
            let num = rand::thread_rng().gen_range(0..=100);
        }else{
            let num = rand::thread_rng().gen_range(chosen_dist.unwrap()[0]..=100);
        }
    }else{
        let num = rand::thread_rng().gen_range(0..=100);
    }

    if num <= chosen_dist.unwrap()[0]{
        "Check".to_string()
    }else if num <= chosen_dist.unwrap()[1]{
        "Fold".to_string()
    }else if num <= chosen_dist.unwrap()[2]{
        "Call".to_string()
    }else{
        "Raise".to_string()
    }

}

//We have a vector of value ranges each representing a pre-flop move. 
//Then fill up hashmap with key value pair
//Key: Hand strength, value: ranges
//Returns hashmap
pub fn fill_move_set()->HashMap<u16, Vec<u16>>{
    let mut move_dist = HashMap::new();
    
    //Vector order: check, fold, call, raise
   let mut vec_of_dists: Vec<Vec<u16>> = vec![

        vec![35, 60, 85, 100], //Hand Strength: 2
        vec![35, 58, 85, 100], //4
        vec![34, 58, 85, 100], //5
        vec![34, 56, 84, 100], //6
        vec![34, 54, 83, 100],
        vec![33, 50, 82, 100], //8
        vec![33, 50, 82, 100],
        vec![33, 49, 82, 100], //10
        vec![33, 48, 82, 100],
        vec![31, 46, 82, 100], //12
        vec![30, 45, 81, 100],
        vec![29, 39, 79, 100], //14
        vec![29, 39, 79, 100],
        vec![28, 39, 79, 100], //16
        vec![27, 38, 79, 100],
        vec![27, 38, 78, 100], //18
        vec![25, 38, 78, 100],
        vec![23, 37, 77, 100], //20
        vec![23, 37, 77, 100],
        vec![21, 36, 78, 100], //22
        vec![20, 34, 77, 100],
        vec![18, 30, 75, 100], //24
        vec![18, 28, 75, 100],
        vec![18, 27, 74, 100], //26
        vec![17, 23, 70, 100],
        vec![14, 19, 69, 100], //28

   ];

    let mut i:u16 = 28;
    while i > 3{
        move_dist.insert(
            i,
            vec_of_dists.pop().unwrap(),
        );

        i-=1;
    }

    move_dist.insert(
        2,
        vec_of_dists.pop().unwrap(),
    );
    move_dist
}