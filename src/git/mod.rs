use chrono::{Duration, Local, NaiveDate};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn compute_streak_from_dates(dates: &HashSet<NaiveDate>, today: NaiveDate) -> i64 {
    let mut streak = 0;
    let mut d = today;
    while dates.contains(&d) {
        streak += 1;
        d -= Duration::days(1);
    }
    streak
}

pub fn git_activity_summary(days: i64) -> (i64, usize, bool) {
    if !is_git_repo() {
        return (0, 0, false);
    }
    let dates = git_dates(days);
    let today = Local::now().date_naive();
    let commits_today = dates.iter().filter(|d| **d == today).count();
    let set: HashSet<NaiveDate> = dates.into_iter().collect();
    let streak = compute_streak_from_dates(&set, today);
    (streak, commits_today, true)
}

pub fn current_dir_name() -> Option<String> {
    std::env::current_dir()
        .ok()
        .and_then(|p: PathBuf| p.file_name().map(|s| s.to_string_lossy().to_string()))
}

fn git_dates(days: i64) -> Vec<NaiveDate> {
    let out = Command::new("git")
        .args([
            "log",
            &format!("--since={days} days ago"),
            "--date=short",
            "--pretty=%ad",
        ])
        .output();

    if let Ok(o) = out
        && o.status.success()
    {
        let s = String::from_utf8_lossy(&o.stdout);
        return s
            .lines()
            .filter_map(|l| NaiveDate::parse_from_str(l.trim(), "%Y-%m-%d").ok())
            .collect();
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

#[cfg(test)]
mod tests {
    use super::compute_streak_from_dates;
    use chrono::NaiveDate;
    use std::collections::HashSet;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn zero_streak_when_today_missing() {
        let mut set = HashSet::new();
        set.insert(d("2025-09-01"));
        let streak = compute_streak_from_dates(&set, d("2025-09-02"));
        assert_eq!(streak, 0);
    }

    #[test]
    fn counts_consecutive_days_backwards() {
        // 2025-09-02 (today), 09-01, 08-31 present → streak = 3
        let dates = ["2025-08-31", "2025-09-01", "2025-09-02"];
        let set: HashSet<_> = dates.iter().map(|s| d(s)).collect();
        let streak = compute_streak_from_dates(&set, d("2025-09-02"));
        assert_eq!(streak, 3);
    }

    #[test]
    fn stops_on_gap() {
        // present on 02 and 31, missing 01 → streak = 1
        let dates = ["2025-08-31", "2025-09-02"];
        let set: HashSet<_> = dates.iter().map(|s| d(s)).collect();
        let streak = compute_streak_from_dates(&set, d("2025-09-02"));
        assert_eq!(streak, 1);
    }
}
