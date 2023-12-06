use super::cards::*;
use super::components::*;
use super::hand_evaluation::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};



pub fn find_winning_hand(
    player_entity_query: &mut Query<(Entity, &mut Player)>,
    mut deck: &mut ResMut<Deck>,
    mut community_query: &mut Query<&CommunityCards>,
) -> (usize, u8, u8) {
    // calculate the player id of the player with the best hand
    // and the value of their hand (and the second highest hand value)

    let mut community_cards: Vec<Card> = community_query
        .iter()
        .flat_map(|cards| &cards.cards)
        .cloned()
        .collect();

    let future_com_count = 5 - community_cards.len();
    let mut future_com = deck.cards[..future_com_count].to_vec();

    let mut winning_id : usize = 0;
    let mut winning_score = 0;
    let mut best_losing_score = 0;

    for (_entity, mut player) in player_entity_query.iter_mut() {
        if player.cash != 0 {
            let mut hand_and_community: Vec<Card> = Vec::new();
            hand_and_community.append(&mut player.cards.clone());
            hand_and_community.append(&mut community_cards);
            hand_and_community.append(&mut future_com);
            let best_hand = find_best_hand(&hand_and_community).score;

            if best_hand > winning_score {
                best_losing_score = best_hand;
                winning_score = best_hand;
                winning_id = player.player_id;
            }
        }
    }

    return (winning_id, winning_score, best_losing_score);
}


pub fn generate_cheating_move(
    player: &mut Player,
    poker_turn: &ResMut<PokerTurn>,
    future_knowledge: (usize, u8, u8),
) -> String {
    // strategy for cheating AI: https://www.desmos.com/calculator/pf8ks2wsry
    let winning_fold_chance = 0.069395; // see the graph
    let losing_fold_chance = 0.242142;
    let mut rng = thread_rng();

    if future_knowledge.0 == player.player_id {
        // AI should win
        if poker_turn.phase == PokerPhase::Showdown {
            let max_bet = f32::powf((future_knowledge.2 as f32) / 8.0, 0.25);
            let random_number : f32 = rng.gen();
            let bet : f32 = (max_bet / 2.0 * (1.0 + random_number)) * (player.cash as f32);
            if (bet as usize) <= poker_turn.current_top_bet {
                return "Call".to_string();
            } else {
                player.raise_amount = ((bet as usize) - poker_turn.current_top_bet).into();
                return "Raise".to_string();
            }
        } else {
            let random_number : f32 = rng.gen();
            if random_number < winning_fold_chance {
                return "Fold".to_string();
            } else {
                return "Call".to_string();
            }
        }
    } else {
        // AI should lose or tie, but might win if bluff successfully
        if poker_turn.phase == PokerPhase::Showdown {
            let max_bet = f32::powf((future_knowledge.2 as f32) / 8.0, 0.25);
            let random_number : f32 = rng.gen();
            let bet : f32 = max_bet / 2.0 * random_number * (player.cash as f32);
            if (bet as usize) <= poker_turn.current_top_bet {
                return "Call".to_string();
            } else {
                player.raise_amount = ((bet as usize) - poker_turn.current_top_bet).into();
                return "Raise".to_string();
            }
        } else {
            let random_number : f32 = rng.gen();
            if random_number < losing_fold_chance {
                return "Fold".to_string();
            } else {
                return "Call".to_string();
            }
        }
    }
    return "Call".to_string();
}


