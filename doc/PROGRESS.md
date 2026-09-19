# Current learning checkpoint

Updated: 2026-09-19. This file is the mutable handoff; see LEARNING_PLAN.md for
the curriculum and AGENTS.md for session rules.

## Verified repository context

- Branch at inspection: `planet`.
- Reference commit: `a5b2dbb` — `feat: voxel planet/helicopter`.
- Cargo.toml: Rust edition 2024; Bevy requirement 0.19.0, dynamic linking.
- Existing showcase source is implemented; it was not built hands-on by the user.
- Learning branch: not created or named. Preserve the reference while deciding
  the exercise location with the user; do not reset the existing source.
- Documentation changes are uncommitted. Always inspect actual Git state next time.

## Active lesson

None prepared yet. Next: `01-app-and-plugins.md` (planned, not an existing file).
No guided lesson has been implemented or completed by the user.

Immediate next action: prepare lesson 01 using `doc/lessons/TEMPLATE.md`.
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
- Pending exercise: none assigned yet.
- Pending questions: learning branch / exercise location when starting lesson 01.

## Update after each checkpoint

Record the lesson filename and exact substep; whether prepared, applied by user,
verified, or discussed; branch/commit context; actual checks and results; questions
or blockers; and one concrete next action. Keep this file short and current.
