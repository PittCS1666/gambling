use std::collections::HashMap;
use bevy::prelude::*;
use super::components::*;
use rand::Rng;


// Different values for each hand, these are used as for each hand you will want to have a different probability for different actions
const HIGH_CARD: usize = 0;
const ONE_PAIR: usize = 1;
const TWO_PAIR: usize = 2;
const THREE_OF_A_KIND: usize = 3;
const STRAIGHT: usize = 4;
const FLUSH: usize = 5;
const FULL_HOUSE: usize = 6;
const FOUR_OF_A_KIND: usize = 7;
const STRAIGHT_FLUSH: usize = 8;
const ROYAL_FLUSH: usize = 9;

impl CfrData {
    pub fn new() -> CfrData {
        let mut actions = vec!["Fold".to_string(), "Call".to_string(), "Raise".to_string(), "Check".to_string()];
        let mut strategy = HashMap::new();
        let mut cumulative_strategy = HashMap::new();
        let mut regret_sum = HashMap::new();

        let initial_probability = 1.0 / actions.len() as f64;

        
        for action in actions {
            strategy.insert(action.clone(), initial_probability); //For each action, assign a probability of taking that action
            cumulative_strategy.insert(action.clone(), 0.0);
            regret_sum.insert(action.clone(), 0.0); //I think this stores our regret for each action
        }

        CfrData {
            strategy,
            cumulative_strategy,
            regret_sum,
        }
    }
}

//Parameters: game phase, other player moves
pub fn utility_gained(action:PlayerAction, player:Player, game_phase:PokerPhase, other_players:Vec<PlayerAction>, player_count: usize, prev_likelihood: usize) -> usize{
    let mut base_likelihood = prev_likelihood;
    //Look at our updated hand strength and determine if that increases or decreases win likelihood
    if (game_phase == PokerPhase::PreFlop){
        if(player.hand_strength <= 4 && player.hand_strength >= 0){
            if(base_likelihood - 0.12 >= 0){
                base_likelihood -= 0.12;
            }else{
                base_likelihood = 0;
            }
        }else if(player.hand_strength > 4 && player.hand_strength <= 8){
            if(base_likelihood - 0.08 >= 0){
                base_likelihood -= 0.08;
            }else{
                base_likelihood = 0;
            }
        }else if(player.hand_strength > 8 && player.hand_strength <= 12){
            if(base_likelihood - 0.07 >= 0){
                base_likelihood -= 0.07;
            }else{
                base_likelihood = 0;
            }
        }else if(player.hand_strength > 12 && player.hand_strength <= 16){
            continue;
        }else if(player.hand_strength > 16 && player.hand_strength <= 20){
            base_likelihood += 0.12
        }else if(player.hand_strength > 20 && player.hand_strength <= 24){
            base_likelihood += 0.2
        }else if(player.hand_strength > 24 && player.hand_strength <= 28){
            base_likelihood += 0.27
        }
    }else{
        if(player.hand_strength == 30){
            if(base_likelihood - 0.04 >= 0){
                base_likelihood -= 0.04;
            }else{
                base_likelihood = 0;
            }
        }else if(player.hand_strength == 31){
            base_likelihood += 0.01
        }else if(player.hand_strength == 32){
            base_likelihood += 0.02
        }else if(player.hand_strength == 33){
            base_likelihood += 0.03
        }else if(player.hand_strength == 34){
            base_likelihood += 0.04
        }else if(player.hand_strength == 35){
            base_likelihood += 0.05
        }else if(player.hand_strength == 36){
            if(base_likelihood += 6 > 1){
                base_likelihood = 1;
            }else{
                base_likelihood += 0.06
            }
        }else if(player.hand_strength == 37){
            if(base_likelihood += 7 > 1){
                base_likelihood = 1;
            }else{
                base_likelihood += 0.07
            }
        }
    }
    //Each other player move is good or bad to varying degrees depending on my hand
    for (action in other_players){
        if(action == "Fold"){
            base_likelihood += ((1/player_count) - (1/(player_count-1)))
        }else if(action == "Raise"){
            base_likelihood -= 0.04; //This amount needs to be relative to hand strength and raise amount
        }else if(action = "Check"){
            continue; //Should this change anything?
        }else{
            continue; //Not sure how calling should be interpeted
        }
    }
    //If another player...
    //Raises: Decrease win likelihood to a degree depending on hand strength and game phase
    //Folds: Improves our win likelihood
    //Checks or Calls: Depends on our hand
    //Look at the move we made and determine if that increased or decreased win likelihood
    let mut utility = base_likelihood - prev_likelihood;
    //That change is our utility
    utility

}

// Using the regret for each action determine the new probabilities for each action
pub fn update_strategy_for_hand(player: &mut Player, hand_category: usize) {
    if let Some(cfr_data) = player.cfr_data.get_mut(&hand_category) { //Checking if the cfr data for the hand number exists
        let mut normalizing_sum = 0.0;
        for (_, regret) in cfr_data.regret_sum.iter() { //For each element in the regret_sum hashmap...
            let adjusted_regret = regret.max(0.0); //Get the maximum value between regret and 0?
            normalizing_sum += adjusted_regret; //Add our adjusted regret to the normalizing_sum
        }

        //
        for (action, regret) in cfr_data.regret_sum.iter() { //Iterating through our regret sums
            let strategy_value = if normalizing_sum > 0.0 {  
                regret.max(0.0) / normalizing_sum       //I believe this is the action probability we get using our regret
            } else {
                1.0 / cfr_data.regret_sum.len() as f64
            };
            cfr_data.strategy.insert(action.clone(), strategy_value); //This is where we update the strategy with the new probability
        }

        //Get our cumulative regret for the whole game
        for (action, strategy_value) in cfr_data.strategy.iter() {
            let cumulative_value = cfr_data.cumulative_strategy.entry(action.clone()).or_insert(0.0);
            *cumulative_value += strategy_value;
        }
    }
}


// Based on probability from regret return the action that was chosen
pub fn select_action_for_hand(player: &mut Player, hand_category: usize) -> String {
    if let Some(cfr_data) = player.cfr_data.get(&hand_category) {
        let mut rng = rand::thread_rng();
        let mut cumulative_probability = 0.0;
        let random_value = rng.gen::<f64>();

        for (action, probability) in &cfr_data.strategy {
            cumulative_probability += probability;
            if random_value <= cumulative_probability {
                return action.clone();
            }
        }

        cfr_data.strategy.keys().next().unwrap().clone()
    } else {
        "Fold".to_string()
    }
}


// After each decision was made determine what the utilities were for the other decisions and update the corresponding regret for that action
pub fn update_regrets_for_hand(player: &mut Player, hand_category: usize, actual_utility: f64, utilities: HashMap<String, f64>) {
    if let Some(cfr_data) = player.cfr_data.get_mut(&hand_category) {
        for (action, &counterfactual_utility) in utilities.iter() {
            let regret = counterfactual_utility - actual_utility;
            let current_regret = cfr_data.regret_sum.entry(action.clone()).or_insert(0.0);
            *current_regret += regret;
        }
    }
}