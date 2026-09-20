# Current learning checkpoint

Updated: 2026-09-20. Mutable handoff; see LEARNING_PLAN.md for the curriculum
and root AGENTS.md for shared rules and workflow skill routing.

## Verified repository context

- Branch at inspection: `learn/bevy-ecs`, tracking `origin/learn/bevy-ecs`.
- HEAD at inspection, before any close-session commit: `27d80f1` —
  `chore: added session start hook`. Reinspect Git on resume.
- Showcase reference commit: `a5b2dbb` — `feat: voxel planet/helicopter`.
- Cargo.toml/Cargo.lock: Bevy 0.19.0; Rust edition 2024; dynamic linking.
- Showcase in `src/` is implemented, not built hands-on by the user.
- Exercise location chosen by user: `examples/learning/main.rs`, modules alongside.
- Current session changes: learning workflow skills and documentation; uncommitted.
  No exercise source or dependencies edited, and no commit/push performed.

## Active lesson and next action

None prepared yet. Next: `doc/lessons/01-app-and-plugins.md` (planned, not present).
No guided lesson has been implemented or completed by the user.

On **"Continue learning"**, prepare lesson 01 using `doc/lessons/TEMPLATE.md`:
a modular Bevy app with camera and visible scene, introducing plugins,
startup/update schedules and component queries. Target the chosen exercise
location. Provide copyable code; the user creates and edits exercise files.

- Pending user exercise: none assigned yet.
- Pending questions: none needed before preparing lesson 01.

## Latest discussion and decisions

- Bevy/ECS first; Rust alongside it. Reconstruct the showcase before GPU/production
  improvements. One small lesson at a time; preserve the existing showcase.
- Assistant teaches, prepares docs, reviews and records progress. User performs
  hands-on implementation. Source/dependency edits need an explicit new request.
- Three repository skills implement continue, review and close. Freestyle questions
  fit between them; no rigid mode or automatic lesson advancement.
- **"Close session"** authorizes saving the handoff, committing relevant session
  code/docs and pushing the current branch. Partial work is saved as partial.
  **"Wrap up"** alone remains documentation-only. No automatic session renaming.
- Skill creation is not invocation; this session has not been closed.

## Checks and evidence

- Historical showcase evidence: four tests passed and a screenshot was inspected
  on the Mac. Not rerun for this workflow work.
- Historical rename verification: `cargo check --offline` passed on 2026-09-20.
  Rename to `the-game` / package `the_game` / title `The Game` was completed;
  the earlier one-time source-edit permission has ended.
- Hook marker and injected checkpoint observed in this session: loading verified
  here, not across all clients or all startup/resume/clear/compact events.
- Three skill files passed the skill-creator validator using cached PyYAML.
- Assistant ran the configured hook command from root and `src/`: valid JSON,
  actual Git state, complete injected docs below the 20,000-character limit.
  File-content and Git-status comparisons confirmed these runs changed nothing.
- `git diff --check` passed for the documentation changes.
- Integration review reconciled AGENTS.md, plan, lesson template, session-start
  guide and this checkpoint. Hook remains read-only; its configuration is unchanged.
- No lesson compilation, application tests or visual checks run for skill setup.
  Fresh-session skill selection and the commit/push workflow have not been executed.

## Handoff maintenance

After meaningful work, record exact lesson/substep, prepared vs user-implemented
vs reviewed/discussed state, checks and who ran them, unresolved issues, pending
exercise and one next action. Honor any explicit no-edits request. Do not infer
learning completion from generated code, Git history or passing tests alone.
