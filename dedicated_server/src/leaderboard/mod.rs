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

    Ok(leaderboard)
}

pub fn add_leaderboard_data(in_name: &str, in_score: u32) -> io::Result<()>
{
    let mut leaderboard = read_leaderboard()?;
    leaderboard.reverse();

    if let Some(existing_entry) = leaderboard.iter_mut().find(|e| e.name == in_name) {
        existing_entry.score += in_score;
    } else {
        leaderboard.push(LeaderboardEntry::new(in_name.to_string(), in_score));
    }

    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(leaderboard_path)?;

    for entry in leaderboard {
        writeln!(file, "{},{}", entry.name, entry.score)?;
    }

    Ok(())
}