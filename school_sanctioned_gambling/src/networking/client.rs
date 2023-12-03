use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;
use super::cards::*;
use super::buttons::*;
use super::components::*;
use super::game_setup::*;
use super::hand_evaluation::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use shrev::EventChannel;
use super update_events::*;


let (player_update_channel, player_update_reader) = init_player_update_channel();
let (poker_turn_update_channel, poker_turn_update_reader) = init_poker_turn_update_channel();
let (community_cards_update_channel, community_cards_update_reader) = init_community_cards_update_channel();
let (last_player_action_update_channel, last_player_action_update_reader) = init_last_player_action_update_channel();
let (num_players_update_channel, num_players_update_reader) = init_num_players_update_channel();
let (poker_phase_update_channel, poker_phase_update_reader) = init_poker_phase_update_channel();

// structure the client to update only at certain times, for example when a players turn is processed
// have the client check event before sending data

pub fn client_init(/*mut in_client: Client*/) {}

pub fn client_tick(server_ip_address: String) {
    //192.168.1.171
    let ip = format!("{}:8888", server_ip_address);
    let mut stream = match TcpStream::connect(ip) {
        Ok(mut stream) => {
            println!("connection successful!");
            loop {
                // updating states, serializing and writing to the server

                for player_update in player_update_channel.read(&mut player_update_reader) {
                    let player_update_data = player_update.0;

                    let serialized_data = serde_json::to_string(&player_update_data).expect("Could not Serialize");
                    stream.write_all(serialized_data.as_bytes()).expect("Could not send data to server");
                }

                for poker_turn_update in poker_turn_update_channel.read(&mut poker_turn_update_reader) {
                    let poker_turn_update_data = poker_turn_update.0;
                    // serialize
                    let serialized_data = serde_json::to_string(&poker_phase_update_data).expect("Could not Serialize");
                    stream.write_all(serialized_data.as_bytes()).expect("Could not send data to server");
 
                }

                for community_cards_update in community_cards_update_channel.read(&mut community_cards_update_reader) {
                    let community_cards_update_data = community_cards_update.0;
                    
                    let serialized_data = serde_json::to_string(&community_cards_update_data).expect("Could not Serialize");
                    stream.write_all(serialized_data.as_bytes()).expect("Could not send data to server");
                }

                for last_player_action_update in last_player_action_update_channel.read(&mut last_player_action_update_reader) {
                    let last_player_action_update_data = last_player_action_update.0;
                    
                    let serialized_data = serde_json::to_string(&last_player_action_update_data).expect("Could not Serialize");
                    stream.write_all(serialized_data.as_bytes()).expect("Could not send data to server");
                }


                for num_players_update in num_players_update_channel.read(&mut num_players_update_reader) {
                    let num_players_update_data = num_players_update.0;

                    let serialized_data = serde_json::to_string(&num_players_update_data).expect("Could not Serialize");
                    stream.write_all(serialized_data.as_bytes()).expect("Could not send data to server");
                }

                for poker_phase_update in poker_phase_update_channel.read(&mut poker_phase_update_reader) {
                    let poker_phase_update_data = poker_phase_update.0;

                    let serialized_data = serde_json::to_string(&poker_phase_update_data).expect("Could not Serialize");
                    stream.write_all(serialized_data.as_bytes()).expect("Could not send data to server");
                }
            }
            


            /*
            loop {
                // Tutorial code used to send and then recieve packs of chars to and from the server
                let mut input = String::new();
                let mut buffer: Vec<u8> = Vec::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");
                stream
                    .write(input.as_bytes())
                    .expect("Failed to write to server");

                let mut reader = BufReader::new(&stream);

                reader
                    .read_until(b'\n', &mut buffer)
                    .expect("Could not read into buffer");

                print!(
                    "{}",
                    str::from_utf8(&buffer).expect("Could not write buffer as string")
                );
            } 
            */
        }
        Err(err) => {
            println!(
                "Failed trying to connect to server, make sure the ip is correct: {}",
                err
            );

            return;
        }
    };
}




