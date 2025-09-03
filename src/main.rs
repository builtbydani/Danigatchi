mod git;
mod mood;

use chrono::{Local, Timelike};
use git::git_activity_summary;
use mood::{ascii_for, choose_mood, detail_line, headline_for};

fn main() {
    let event = std::env::args().nth(1).unwrap_or_else(|| "motd".into());
    let (streak, commits_today, in_repo) = git_activity_summary(45);
    let hour = Local::now().hour();

    let mood = choose_mood(streak, commits_today, hour, &event, in_repo);
    let ascii = ascii_for(&mood);
    let line1 = headline_for(&mood, streak, commits_today, in_repo);
    let line2 = detail_line(streak, commits_today, in_repo);
    let aff = mood::copy::random_affirmation();

    println!("{ascii}");
    println!("{line1}");
    if let Some(l2) = line2 {
        println!("{l2}");
    }
    println!("{aff}");
}
