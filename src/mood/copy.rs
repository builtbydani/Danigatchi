use rand::seq::SliceRandom;

pub fn random_affirmation() -> String {
    let pool = [
        "you don’t debug, you hex problems ✨",
        "small commits, big magic 🔮",
        "tests green? mood green ✅",
        "push the spell, young witch 🧙‍♀️",
        "one file at a time. you got this.",
    ];
    let mut rng = rand::thread_rng();
    format!("💬 {}", pool.choose(&mut rng).unwrap())
}
