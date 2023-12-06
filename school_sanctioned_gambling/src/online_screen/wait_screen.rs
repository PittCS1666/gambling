use super::{AppState, UserInfo, Users, GameSigned, GameInteraction};
use bevy::prelude::*;
use bevy_egui::{egui::RichText, *};

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const leaderboard_path: &str = "assets/leaderboard.txt";

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct LeaderboardEntry {
    name: String,
    score: u32,
}

impl LeaderboardEntry {
    fn new(name: String, score: u32) -> Self {
        LeaderboardEntry { name, score }
    }
}

fn read_leaderboard() -> io::Result<Vec<LeaderboardEntry>>
{
    let file = File::open(leaderboard_path)?;
    let reader = BufReader::new(file);
    let mut leaderboard = Vec::new();

    for line in reader.lines() {
        let s = line?;
        let parts: Vec<&str> = s.split(',').collect();

        let name = parts[0].trim().to_string();
        let score: u32 = parts[1].trim().parse().unwrap_or(0);

        let entry = LeaderboardEntry::new(name, score);
        leaderboard.push(entry);
    }

    // Last step is to sort based on "score" member variable
    leaderboard.sort_by(|a: &LeaderboardEntry, b: &LeaderboardEntry| a.score.cmp(&b.score));
    leaderboard.reverse();

    Ok(leaderboard)
}

/// wait screen every second update
pub(super) fn wait_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    users: Res<Users>,
    signed: Res<GameSigned>,
    interaction: Res<GameInteraction>,
) {
    if interaction.is_master{
egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
            });
        });
    
        egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
            if ui.button(RichText::new("Start").size(16.0)).clicked() {
    
                signed.sd.send(Some(super::Message::Start));
            }
            if ui.button(RichText::new("Back").size(16.0)).clicked() {
    
                signed.sd.send(None);
            }
            
        });
    
        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("Login")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
    
                        for UserInfo { ip, name } in users.users.lock().unwrap().iter() {
                            
                            ui.label(RichText::new(name).size(24.0))
                                .on_hover_text(format!("target:{ip:?}"));
                            if ui.button(RichText::new("kick").size(16.0).weak()).clicked() {
                                signed.sd.send(Some(super::Message::Kick(ip.to_string())));
                            }
                            ui.end_row();
                        }
                    });
            })
        });
    }
    else{
        egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
            });
        });
    
        egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
            if ui.button(RichText::new("Back").size(16.0)).clicked() {
    
                signed.sd.send(None);
            }
        });
    
        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("Login")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
    
                        for UserInfo { ip, name } in users.users.lock().unwrap().iter() {
                            
                            ui.label(RichText::new(name).size(24.0))
                                .on_hover_text(format!("target:{ip:?}"));
                            ui.end_row();
                        }
                    });
            })
        });
    }
 {
    egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Game Lobby").size(30.0).strong())
        });
    });

    egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
        if ui.button(RichText::new("Back").size(16.0)).clicked() {
            state.set(AppState::OnlineEnd);
        }

        // Printing out the leaderboard of this server
        match read_leaderboard() {
            Ok(data) => {
                for entry in data
                {
                    let s = format!("Player: {}, Chip Score: {}", entry.name, entry.score);
                    ui.label(egui::RichText::new(s).size(15.0).strong());
                }
            }
            Err(err) => {}
        }
    });

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("Login")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for UserInfo { ip, name } in users.users.blocking_read().iter() {
                        ui.label(RichText::new(name).size(24.0))
                            .on_hover_text(format!("target:{ip:?}"));

                        ui.end_row();
                    }
                });
            })
        });
    }
}
