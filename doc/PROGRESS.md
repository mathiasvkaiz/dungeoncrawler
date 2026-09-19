# Current learning checkpoint

Updated: 2026-09-20. This file is the mutable handoff; see LEARNING_PLAN.md for
the curriculum and AGENTS.md for session rules.

## Verified repository context

- Branch at inspection: `learn/bevy-ecs`, tracking `origin/learn/bevy-ecs`.
- Latest commit: `64a26f5` — `chore: renamed repo`.
- Showcase reference commit: `a5b2dbb` — `feat: voxel planet/helicopter`.
- Cargo.toml: Rust edition 2024; Bevy requirement 0.19.0, dynamic linking.
- Existing showcase source is implemented; it was not built hands-on by the user.
- Learning branch exists. Exercise location is still undecided; preserve the
  showcase source until the user chooses where to apply lesson 01.
- Worktree was clean at resume; this checkpoint update is uncommitted.

## Active lesson

None prepared yet. Next: `01-app-and-plugins.md` (planned, not an existing file).
No guided lesson has been implemented or completed by the user.

After the pending fresh-session hook test: prepare lesson 01 using `doc/lessons/TEMPLATE.md`.
Its outcome is a modular Bevy app with a camera and visible scene, introducing
plugins, startup/update schedules and component queries at an experienced
developer's level. Resolve where the user will paste the code before any source
replacement. Explain and provide code; let the user apply it.

## Latest decisions

- Bevy/ECS first; Rust explanations alongside, no generic programming course.
- Hands-on reconstruction of the entire showcase precedes production upgrades.
- One just-in-time Markdown lesson per reviewable increment.
- Assistant edits lesson docs by default, not exercise code.
- The earlier GPU-first and territory-first next steps are superseded.

## Evidence and outstanding work

- Prior showcase run: four tests passed and a screenshot was inspected on the Mac.
  Historical evidence only, not rerun as part of this documentation setup.
- Rename verification: `cargo check --offline` passed on 2026-09-20.
- Pending exercise: none; session-start hook setup/test precedes lesson 01.
- Pending questions: exercise location when starting lesson 01.

## Update after each checkpoint

Record the lesson filename and exact substep; whether prepared, applied by user,
verified, or discussed; branch/commit context; actual checks and results; questions
or blockers; and one concrete next action. Keep this file short and current.

## Latest handoff — project rename

- User renamed the local folder:
  `/Users/mvk/development/gaming/dungeoncrawl`
  → `/Users/mvk/development/gaming/the-game`.
- Reopen Codex in the new folder; the previous session used the old path.
- Intended naming:
  - Folder / repository: `the-game`
  - Rust package: `the_game`
  - Display title: `The Game`
- Rename complete: folder `the-game`, Cargo.toml/Cargo.lock package `the_game`,
  window title and README heading `The Game` verified locally.
- Origin verified: `https://github.com/mathiasvkaiz/the-game.git`.
  User confirmed the GitHub repository rename; no separate remote query made.
- Assistant made naming edits with explicit one-time permission. Normal
  hands-on source-editing rules resume after that exception.
- Compilation passed; no tests or visual run in this verification.
  No lesson prepared or completed.
- Inspect actual branch and worktree before proceeding.
- User performs changes hands-on; provide instructions, do not edit
  application files or perform renames automatically.

## Session workflow decisions

- No automatic session renaming, commits, or source changes.
- Implemented the read-only SessionStart hook in `.codex/hooks.json` and
  `.codex/hooks/session_start.py` at the user's request. It loads actual Git
  state, the learning plan, progress and Cargo.toml on startup/resume/clear/compact.
- CLI reports `codex-cli 0.155.1` with `hooks` enabled. The configured command
  passed direct execution checks from both repository root and `src/`: valid
  JSON, actual branch, complete checkpoint and context below the configured limit.
- Next: review/trust the hook via CLI `/hooks`, then verify the injected marker
  in a fresh session using [SESSION_START.md](SESSION_START.md). Automatic client
  execution and desktop support have not been verified. After that, lesson 01.
- “Wrap up” means record progress, outstanding work, and the next action.
- After project renaming, resume the existing learning roadmap:
  reconstruct the showcase step by step, then production improvements.
