use super::cards::*;
use super::components::*;
use super::hand_evaluation::*;
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

struct TreeNode {
    decision: bool,
    boundary: u16,
    poker_move: String,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(check_boundary: u16, fold_boundary: u16, call_boundary: u16) -> TreeNode{
        TreeNode{
            decision: true,
            boundary: check_boundary,
            poker_move: "".to_string(),
            left: Some(Box::new(TreeNode {
                    decision: false,
                    boundary: 0,
                    poker_move: "Check".to_string(),
                    left: None,
                    right: None,
                })),
            right: Some(Box::new(TreeNode {
                    decision: true,
                    boundary: fold_boundary,
                    poker_move: "".to_string(),
                    left: Some(Box::new(TreeNode {
                            decision: false,
                            boundary: 0,
                            poker_move: "Fold".to_string(),
                            left: None,
                            right: None,
                        })),
                    right: Some(Box::new(TreeNode {
                            decision: true,
                            boundary: call_boundary,
                            poker_move: "".to_string(),
                            left: Some(Box::new(TreeNode {
                                    decision: false,
                                    boundary: 0,
                                    poker_move: "Call".to_string(),
                                    left: None,
                                    right: None,
                                })),
                            right: Some(Box::new(TreeNode {
                                    decision: false,
                                    boundary: 0,
                                    poker_move: "Raise".to_string(),
                                    left: None,
                                    right: None,
                                })),
                        }))
                }))
        }
    }

    fn new_blank() -> TreeNode {
        TreeNode {
            decision: false,
            boundary: 0,
            poker_move: "".to_string(),
            left: None,
            right: None,
        }
    }

    fn get_move(head: TreeNode, num: u16) -> String{
        let mut cur_node = head;
        loop {
            if cur_node.decision == true {
                if num <= cur_node.boundary {
                    cur_node = match cur_node.left {
                        None => TreeNode::new_blank(),
                        Some(i) => *i,
                    };
                }
                else {
                    cur_node = match cur_node.right {
                        None => TreeNode::new_blank(),
                        Some(i) => *i,
                    };
                }
            }
            else {
                return cur_node.poker_move;
            }

        }
    }
    
}

//Simply sets Ace to strongest card. All others remain the same
pub fn generate_card_strength(val: u8) -> u8 {
    if val == 1 {
        14
    } else {
        val
    }
}

pub fn generate_post_flop_hand_strength(
    mut hand: &mut Vec<Card>,
    community_query: &mut Query<&CommunityCards>,
) -> u16 {
    /*let mut hand_and_community = vec_hand.clone();
    for vector in &community.cards{
        hand_and_community.push(*vector);
    }*/

    let mut hand_and_community: Vec<Card> = Vec::new();
    hand_and_community.append(&mut hand.clone());

    for community_cards in community_query.iter() {
        hand_and_community.append(&mut community_cards.cards.to_vec())
    }

    //hand_and_community.append(&mut community_cards.cards);

    let best_hand = find_best_hand(&hand_and_community);
    best_hand.score as u16
}

//Generates hand strength from starting hand
pub fn generate_pre_flop_hand_strength(hand: &Vec<Card>) -> u16 {
    hand[0].card_strength as u16 + hand[1].card_strength as u16
}

//Checks rand number w/in ranges to determine move
pub fn generate_move(
    player: &mut Player,
    poker_turn: &ResMut<PokerTurn>,
    mut community_query: &mut Query<&CommunityCards>,
) -> String {
    //Check for poker phase
    let mut _num: u16 = 101;
    let mut chosen_dist = player.move_dist.get(&player.hand_strength);

    if poker_turn.phase == PokerPhase::PreFlop {
        chosen_dist = player.move_dist.get(&player.hand_strength);
        if player.big_blind && !poker_turn.pot_raised {
            _num = rand::thread_rng().gen_range(0..=100);
        } else {
            _num = rand::thread_rng().gen_range(chosen_dist.unwrap()[0]..=100);
        }
    } else if poker_turn.phase == PokerPhase::Flop
        || poker_turn.phase == PokerPhase::Turn
        || poker_turn.phase == PokerPhase::River
    {
        player.hand_strength =
            generate_post_flop_hand_strength(&mut player.cards, &mut community_query);

        chosen_dist = player.move_dist.get(&(player.hand_strength + 30 as u16));
        if !poker_turn.bet_made {
            _num = rand::thread_rng().gen_range(0..=100);
        } else {
            _num = rand::thread_rng().gen_range(chosen_dist.unwrap()[0]..=100);
        }
    } else {
        chosen_dist = player.move_dist.get(&(player.hand_strength + 30 as u16));
        _num = rand::thread_rng().gen_range(0..=100);
    }

    let decision_tree = TreeNode::new(chosen_dist.unwrap()[0], chosen_dist.unwrap()[1], chosen_dist.unwrap()[2]);
    TreeNode::get_move(decision_tree, _num)

    /*if _num <= chosen_dist.unwrap()[0] {
        "Check".to_string()
    } else if _num <= chosen_dist.unwrap()[1] {
        "Fold".to_string()
    } else if _num <= chosen_dist.unwrap()[2] {
        "Call".to_string()
    } else {
        "Raise".to_string()
    }*/
}

//We have a vector of value ranges each representing a pre-flop move.
//Then fill up hashmap with key value pair
//Key: Hand strength, value: ranges
//Returns hashmap
pub fn fill_move_set() -> HashMap<u16, Vec<u16>> {
    let mut move_dist = HashMap::new();

    //Vector order: check, fold, call, raise
    let mut vec_of_dists: Vec<Vec<u16>> = vec![
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
        vec![15, 15, 63, 100],
        vec![11, 13, 59, 100], //28
        vec![23, 54, 82, 100], //30
        vec![10, 17, 68, 100], //31
        vec![10, 16, 68, 100], //32
        vec![9, 14, 66, 100], //33
        vec![9, 14, 66, 100], //34
        vec![9, 14, 64, 100], //35
        vec![8, 11, 62, 100], //36
        vec![7, 9, 60, 100], //37
    ];

    //Here we are filling out the postflop and after distributions
    let mut i = 8;
    while i > 0 {
        move_dist.insert((i - 1) + 30, vec_of_dists.pop().unwrap());

        i -= 1;
    }

    //Here we are filling out the preflop distributions
    let mut i: u16 = 28;
    while i > 3 {
        move_dist.insert(i, vec_of_dists.pop().unwrap());

        i -= 1;
    }

    move_dist
}
