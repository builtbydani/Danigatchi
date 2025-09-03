## ✨ DaniGatchi 🦊🐾

A witchy terminal companion that cheers you on while you code.
DaniGatchi lives in your MOTD and Git hooks, tracking your Git streaks, commits, and moods with cute ASCII vibes + affirmations.

# Features

- 🐾 ASCII foxcat moods (happy, slay, sleepy, sad, neutral)

- 📊 Git streak tracker — counts consecutive commit days & today’s commits

- 🧙‍♀️ Event-aware hype — celebrates commit/push with SLAY energy

- 🌸 Random affirmations like “you don’t debug, you hex problems ✨”

- 🎀 MOTD mode — auto-print DaniGatchi on shell login

- 🔮 Hooks integration — drop DaniGatchi into post-commit or pre-push

# 🚀 Install

```
git clone https://github.com/youruser/danigatchi
cd danigatchi
cargo install --path .
```

# 🕹 Usage

Run Anywhere!

```
danigatchi
```

With event hint:

```
danigatchi commit
danigatchi push
```

# 🔧 Integration

MOTD / shell login → add to ~/.bashrc or ~/.zshrc:

```
if command -v danigatchi >/dev/null 2>&1; then
  danigatchi
fi
```

Git hook: post-commit

```
echo '#!/bin/sh
danigatchi commit' > .git/hooks/post-commit
chmod +x .git/hooks/post-commit
```

Git hook: pre-push

```
echo '#!/bin/sh
danigatchi push' > .git/hooks/pre-push
chmod +x .git/hooks/pre-push
```

# 📸 Example Output

```
  (=^･ω･^=) ♡

(=^･ㅅ･^=)  nice! 2-day streak + fresh commits
repo: danigatchi | today: 2 | streak: 2
💬 small commits, big magic 🔮
```

# 🧪 Dev setup
```
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

CI runs all of the above on every push

# 📝 License

MIT
