
//Declaring Player Struct
struct Player{
    hand: Vec<Card>,
    hand_strength: u16
}

impl Player{
    //hand clone will be copy of hand bc structs cannot be self-referantial...I think
    fn new(hand: Vec<Card>, hand_clone: &Vec<Card>) -> Self{
        Player{
            hand,
            hand_strength: generate_hand_strength(hand_clone),
        }
    }
}

//Stealing this for now 
//Adding card_strength field
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
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum MoveSet {
    Fold,
    Check,
    Bet,
    Call, 
    Raise,
}
//Simply sets Ace to strongest card. All others remain the same
fn generate_card_strength(val:u8) -> u8{
    if val == 1{
        14
    }else{
        val
    }
}

fn generate_hand_strength(vec_hand: &Vec<Card>) -> u16{
    vec_hand[0].card_strength as u16 + vec_hand[1].card_strength as u16
}

fn main(){

    let card1 = Card::new(5, Suit::Hearts, 1);
    let card2 = Card::new(45, Suit::Clubs, 11);
    let vec1 = vec![card1, card2];
    let vec2 = vec1.clone();
    let _player1 = Player::new(vec1, &vec2);
    println!("Player hand strength is: {}", _player1.hand_strength);

}