# Current learning checkpoint

Updated: 2026-09-20. See LEARNING_PLAN.md for the curriculum and root AGENTS.md
for shared rules and workflow skill routing.

## Verified repository context

- Branch: `learn/bevy-ecs`, tracking `origin/learn/bevy-ecs`.
- Pre-close HEAD: `c730060` — `chore: added skills`. Reinspect Git on resume.
- Push destination: `origin`, https://github.com/mathiasvkaiz/the-game.git,
  branch `learn/bevy-ecs`. Delivery outcome must be verified after this handoff.
- Cargo.toml/Cargo.lock: Bevy 0.19.0; Rust edition 2024; dynamic linking.
- Showcase in `src/` remains reference code, not built hands-on by the user.
- User implemented `examples/learning/main.rs` and `scene.rs`. Assistant edited
  lesson/workflow documentation only; no exercise source or dependencies changed.
- Closing scope: these two exercise files, lesson 01, lesson template, AGENTS.md,
  LEARNING_PLAN.md, continue-learning skill and this checkpoint.

## Completed checkpoint and next action

[Lesson 01 — An app and a scene plugin](lessons/01-app-and-plugins.md) is complete
for this increment: user implemented the example, assistant reviewed it without
findings, compilation passed, the Spin-removal experiment was discussed, and the
user confirmed the square rotates again after restoring Spin.

Next action on **Continue learning**: prepare only lesson 02, the first small A2
step introducing voxel data layout/indexing and its ownership boundary. Build on
the existing scene example; do not jump to a whole planet generator or renderer.
Lesson 02 has not been prepared or implemented.

- Pending user exercise: none from lesson 01.
- Unresolved questions: none raised at close; earlier concepts can be revisited.
- No automatic advancement or new lesson preparation during closing.

## Teaching decisions and discussion

- Code first: brief outcome/prerequisites, complete commented code, run/check,
  then broader concepts and a small experiment. User performs source edits.
- Function doc comments explain purpose, caller and timing; inline comments
  explain non-obvious details. Avoid repeating these in the concepts section.
- Explain Bevy control flow explicitly: registration versus execution, who calls
  hooks/systems, when/how often, and where parameters come from. Include a short
  execution sequence and contextual Rust explanations, especially mut versus &mut.
- Visible Bevy/Rust info notes suit terminal reading. Essential explanations
  should not depend on HTML folding. Template and continue skill reflect this.
- Discussed Cargo example discovery, shared dependencies and offline mode;
  Plugin::build versus scheduled systems; query matching and an empty query.
- User reported removing Spin stopped rotation without error. Explained that
  the system still runs but no entities match, so its loop has zero iterations.
  Spin is restored at 1.0 rad/s; user confirmed rotation at close.
- Preserve the showcase. Reconstruct it before GPU/production improvements.
- Continue/review do not authorize Git delivery. Explicit Close session authorizes
  the relevant commit/push; this session invoked that workflow.

## Checks and evidence

- Assistant source review: both exercise files match lesson logic. At close the
  user had added the teaching comments; both files match the lesson code exactly.
- Assistant ran `cargo check --offline --locked --example learning` on the actual
  exercise: passed. Reused this result at close because executable code is unchanged.
- Exact lesson Rust code was previously compiled in a temporary project using
  copies of the repository manifest/lockfile, with Rust/Cargo 1.97.0. Later changes
  only added comments; non-comment code was compared with that compiled version.
- User visually confirmed restored rotation; assistant did not run or visually
  inspect the GUI. Source review, compilation and user observation are separate evidence.
- Continue-learning skill validator passed after its instruction updates.
- `git diff --check` passed during documentation/review work; final staged check
  is part of closing. No unrelated application tests rerun.
- Installed Bevy 0.19.0 sources/examples checked for sprite, rotation and plugin
  lifecycle APIs. No dependency upgrades.
- Historical showcase evidence: four tests and a screenshot inspected on the Mac;
  not rerun for this lesson. Historical rename check passed offline.
- SessionStart hook marker observed in this session. No claim about other clients
  or all lifecycle events; hook configuration unchanged.

## Handoff maintenance

Record implementation, checks, user observations and discussion separately.
Reinspect Git after closing; the pre-close hash above is intentionally historical.
Keep future lessons small and record partial work without inferring understanding
from generated code or compilation alone.
