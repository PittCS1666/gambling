use std::collections::HashSet;
use super::cards::*;

struct Hand {
    cards: Vec<Card>,   //a vector with the 5 cards in a hand
    ranks: Vec<u8>,     //a vector that holds the number of each rank that is in the hand
    suits: u8,          //how many suits are present in the hand
    score: u8           //the final score of the hand -> 8: Straight flush, 7: Four of a kind, 6: Full house, 5: flush, 4: straight, 3: three of a kind, 2: two pair, 1: pair, 0: high card
}

impl Hand {
    fn _new_blank() -> Hand {
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
        if hand.cards.is_empty() {
            return String::from("No cards in hand");
        }
                
        let card1 = Card::to_string(&hand.cards[0]);
        let card2 = Card::to_string(&hand.cards[1]);
        let card3 = Card::to_string(&hand.cards[2]);
        let card4 = Card::to_string(&hand.cards[3]);
        let card5 = Card::to_string(&hand.cards[4]);
        
        String::from(format!("{card1}, {card2}, {card3}, {card4}, and {card5} with a score of {score}",
            card1 = card1, card2 = card2, card3 = card3, card4 = card4, card5 = card5, score = hand.score.to_string()))
    }
}

pub fn test_evaluator(player_id: u8, player_cards: Vec<Card>, community_cards: Vec<Card>) {
    let cards: Vec<Card> = player_cards.into_iter().chain(community_cards.into_iter()).collect();
    // use println below to see players cards in terminal
    // println!("{}", cards.iter().map(|card| card.to_string()).collect::<Vec<_>>().join(", "));
    let hand = find_best_hand(&cards);
    println!("\nPlayer {}: {}", player_id, Hand::to_string(&hand));
}

fn find_best_hand(cards: &Vec<Card>) -> Hand{
    //there might be a library to get all combinations easily
    let combinations: Vec<Vec<Card>> = get_all_hands(cards);

    let mut best_hand_val = 1;
    let mut best_hand: Hand = Hand::new(cards.to_vec());
    
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
            } else if res == 0 {
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