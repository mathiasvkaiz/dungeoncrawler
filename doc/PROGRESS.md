# Current learning checkpoint

Updated: 2026-09-20. This file is the mutable handoff; see LEARNING_PLAN.md for
the curriculum and AGENTS.md for session rules.

## Verified repository context

- Branch at inspection: `learn/bevy-ecs`, tracking `origin/learn/bevy-ecs`.
- Latest commit: `a926156` — `doc: revised learning plan and additional docs`.
- Showcase reference commit: `a5b2dbb` — `feat: voxel planet/helicopter`.
- Cargo.toml: Rust edition 2024; Bevy requirement 0.19.0, dynamic linking.
- Existing showcase source is implemented; it was not built hands-on by the user.
- Learning branch exists. Exercise location is still undecided; preserve the
  showcase source until the user chooses where to apply lesson 01.
- Documentation changes are uncommitted. Always inspect actual Git state next time.

## Active lesson

None prepared yet. Next: `01-app-and-plugins.md` (planned, not an existing file).
No guided lesson has been implemented or completed by the user.

After the pending project rename: prepare lesson 01 using `doc/lessons/TEMPLATE.md`.
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
- Current setup: documentation review / `git diff --check`; no app source changed.
- Pending exercise: apply the project naming edits below, then run `cargo check`.
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
- Local folder rename confirmed on 2026-09-20. Cargo.toml and Cargo.lock still
  name the package `dungeoncrawl`; src/main.rs still uses the old window title.
- Next user edits: in Cargo.toml replace `name = "dungeoncrawl"` with
  `name = "the_game"`; in src/main.rs replace the window title with `The Game`.
  Run `cargo check` to refresh the lockfile package entry and verify compilation.
- Inspected origin: `https://github.com/mathiasvkaiz/dungeoncrawler.git`.
  Remote repository rename is unverified; discuss it after the local edits.
- This session: source inspection only; no compilation, tests or visual run.
  No application files changed by the assistant; no lesson prepared or completed.
- Inspect actual branch and worktree before proceeding.
- User performs changes hands-on; provide instructions, do not edit
  application files or perform renames automatically.

## Session workflow decisions

- No automatic session renaming, commits, or source changes.
- A small read-only, branch-aware SessionStart hook was proposed to load
  the learning checkpoint. It has NOT been implemented.
- Verify installed-client hook support before configuring it.
- “Wrap up” means record progress, outstanding work, and the next action.
- After project renaming, resume the existing learning roadmap:
  reconstruct the showcase step by step, then production improvements.
