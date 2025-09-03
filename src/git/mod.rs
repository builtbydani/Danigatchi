use chrono::{NaiveDate, Duration, Local, Timelike};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

pub fn git_activity_summary(days: i64) -> (i64, usize, bool) {
    if !is_git_repo() { return (0, 0, false); }
    let dates = git_dates(days);
    let today = Local::now().date_naive();
    let commits_today = dates.iter().filter(|d| **d == today).count();
    let mut streak = 0;
    let mut d = today;
    let set: HashSet<NaiveDate> = dates.into_iter().collect();
    while set.contains(&d) {
        streak += 1;
        d = d - Duration::days(1);
    }
    (streak, commits_today, true)
}

pub fn current_dir_name() -> Option<String> {
    std::env::current_dir().ok().and_then(|p: PathBuf| {
        p.file_name().map(|s| s.to_string_lossy().to_string())
    })
}

fn git_dates(days: i64) -> Vec<NaiveDate> {
    let out = Command::new("git")
        .args([
            "log",
            &format!("--since={} days ago", days),
            "--date=short",
            "--pretty=%ad",
        ])
            .output();

    if let Ok(o) = out {
        if o.status.success() {
            let s = String::from_utf8_lossy(&o.stdout);
            return s
                .lines()
                .filter_map(|l| NaiveDate::parse_from_str(l.trim(), "%Y-%m-%d").ok())
                .collect();
        }
    }
    vec![]
}

fn is_git_repo() -> bool {
    if let Ok(o) = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
    {
        return o.status.success() && String::from_utf8_lossy(&o.stdout).trim() == "true";
    }
    false
}
