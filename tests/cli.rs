use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use chrono::Local;
use predicates::prelude::*;
use std::process::Command;

fn init_git_repo(dir: &assert_fs::TempDir) {
    let git = |args: &[&str]| {
        let mut c = std::process::Command::new("git");
        c.current_dir(dir.path()).args(args);
        let out = c.output().expect("git failed");
        assert!(out.status.success(), "git {:?} failed: {:?}", args, out);
    };
    git(&["init", "-q"]);
    git(&["config", "user.name", "Test User"]);
    git(&["config", "user.email", "test@example.com"]);
}

fn commit_with_date(dir: &assert_fs::TempDir, date: chrono::NaiveDate, msg: &str) {
    let file = dir.child("file.txt");

    // overwrite (or create) the file with new content each commit
    file.write_str(&format!("{}\n", msg)).unwrap();

    // stage and commit with fixed dates so streak math is deterministic
    let mut add = std::process::Command::new("git");
    add.current_dir(dir.path())
        .args(["add", "."])
        .status()
        .unwrap();

    let mut commit = std::process::Command::new("git");
    commit
        .current_dir(dir.path())
        .env("GIT_AUTHOR_DATE", format!("{} 12:00:00", date))
        .env("GIT_COMMITTER_DATE", format!("{} 12:00:00", date))
        .args(["commit", "-m", msg])
        .status()
        .expect("git commit failed");
}

#[test]
fn runs_outside_git_repo_and_says_not_a_repo() {
    let temp = assert_fs::TempDir::new().unwrap();
    // Not a git repo on purpose
    Command::cargo_bin("danigatchi")
        .unwrap()
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("not in a git repo"));
}

#[test]
fn shows_today_and_streak_inside_repo() {
    let temp = assert_fs::TempDir::new().unwrap();
    init_git_repo(&temp);

    let today = Local::now().date_naive();
    let yesterday = today - chrono::Duration::days(1);

    // Create two commits on consecutive days to produce streak=2 and today=1
    commit_with_date(&temp, yesterday, "yesterday");
    commit_with_date(&temp, today, "today");

    Command::cargo_bin("danigatchi")
        .unwrap()
        .current_dir(temp.path())
        .assert()
        .success()
        // the detail line should include these counts:
        .stdout(predicate::str::contains("streak: 2"))
        .stdout(predicate::str::contains("today: 1"));
}

#[test]
fn commit_event_sets_slay_after_a_commit_today() {
    let temp = assert_fs::TempDir::new().unwrap();
    init_git_repo(&temp);

    let today = Local::now().date_naive();
    commit_with_date(&temp, today, "today");

    Command::cargo_bin("danigatchi")
        .unwrap()
        .current_dir(temp.path())
        .args(["commit"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SLAY").or(predicate::str::contains("SLAY!")));
}
