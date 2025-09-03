mod mood;
mod git;

use chrono::Local;
use mood::{choose_mood, ascii_for, headline_for, detail_line, Mood};
use git::git_activity_summary;

fn main() {
    let event = std::env::args().nth(1).unwrap_or_else(|| "motd".into());
    let (streak, commits_today, in_repo) = git_activity_summary(45);
    let hour = Local::now().hour();

    let mood = choose_mood(streak as i64, commits_today, hour, &event, in_repo);
    let ascii = ascii_for(&mood);
    let line1 = headline_for(&mood, streak as i64, commits_today, hour, &event, in_repo);
    let line2 = detail_line(streak as i64, commits_today, in_repo);
    let aff = mood::copy::random_affirmation();

    println!("{ascii}");
    println!("{line1}");
    if let Some(l2) = line2 { println("{l2}"); }
    println!("{aff}");
}
