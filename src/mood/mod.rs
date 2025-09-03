pub mod copy;

#[derive(Clone, Copy)]
pub enum Mood {Slay, Happy, Sleepy, Neutral, Sad, NotARepo }

pub fn choose_mood(streak: i64, today: uSize, hour: u32, event: &str, in_repo: bool) -> Mood {
    if !in_repo { return Mood::NotARepo; }
    if matches!(event, "commit" | "push") && today > 0 { return Mood::Slay; }
    if hour <= 5 && today == 0 { return Mood::Sleepy; }
    if streak >= 7 { return Mood::Slay; }
    if streak >= 3 || today > 0 { return Mood::Happy; }
    if streak == 0 { return Mood::Sad; }
    Mood::Neutral
}

pub fn ascii_for(mood: &Mood) -> &'static str {
    match mood {
        Mood::Slay => r#"   ✨  (=^･ᴥ･^=)  ✨
   /)  /)
  (  • •)  yasss!
   \▾▾/
"#,
        Mood::Happy   => "  (=^･ω･^=) ♡\n",
        Mood::Sleepy  => "  (=-_-=) zzz\n",
        Mood::Sad     => "  ( ; _ ; )\n",
        Mood::Neutral | Mood::NotARepo => "  (=^･ㅅ･^=)\n",
    }
}

pub fn headline_for(mood: &Mood, streak: i64, today: usize, in_repo: bool) -> String {
    match mood {
        Mood::NotARepo => "(=^･ω･^=)  DaniGatchi: not in a git repo; just vibing ✨".into(),
        Mood::Slay => format!("(=^･ᴥ･^=)ﾉ彡☆  SLAY! {}-day streak, {} commit(s) today 🔮",
                              streak.max(1), today),
        Mood::Happy => format!("(=^･ㅅ･^=)  nice! {}-day streak{}", streak,
                               if today>0 { " + fresh commits" } else { "" }),
        Mood::Sleepy => "(=-_-=) zzz… it’s late witch… push one tiny commit then rest 🌙".into(),
        Mood::Sad => "(-﹏-)  no commits lately… summon a tiny PR spell?".into(),
        Mood::Neutral => format!("(=^･ω･^=) keep going! streak: {}", streak),
    }
}

pub fn detail_line(streak: i64, today: usize, in_repo: bool) -> Option<String> {
    if !in_repo { return None; }
    let repo = crate::git::current_dir_name().unwrap_or_else(|| "?".into());
    Some(format!("repo: {} | today: {} | streak: {}", repo, today, streak))
}
