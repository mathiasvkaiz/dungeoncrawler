# Current learning checkpoint

Updated: 2026-09-20. See LEARNING_PLAN.md for the curriculum and root AGENTS.md
for shared rules and workflow skill routing.

## Verified repository context

- Branch: `learn/bevy-ecs`, tracking `origin/learn/bevy-ecs`.
- Pre-close HEAD: `ad61586` — `feat: complete first Bevy learning lesson`.
  Working tree was clean before lesson 02 documentation preparation; local HEAD
  matches the local origin tracking ref. No live remote fetch performed.
- Push destination: `origin`, https://github.com/mathiasvkaiz/the-game.git,
  branch `learn/bevy-ecs`. User explicitly requested Close session; commit/push
  outcome must be verified after this handoff is saved.
- Cargo.toml/Cargo.lock: Bevy 0.19.0; Rust edition 2024; dynamic linking.
- Showcase in `src/` remains reference code, not built hands-on by the user.
- User implemented lesson 02 in `examples/learning/main.rs`, `planet.rs` and
  `voxel.rs`; `scene.rs` remains unchanged from lesson 01. Assistant edits are
  documentation only; dependencies remain unchanged.
- Review HEAD remains `ad61586`. Unstaged: main.rs and PROGRESS.md; untracked:
  planet.rs, voxel.rs and lesson 02. Nothing staged at review.

## Active checkpoint and next action

[Lesson 01 — An app and a scene plugin](lessons/01-app-and-plugins.md) is complete
for this increment: user implemented the example, assistant reviewed it without
findings, compilation passed, the Spin-removal experiment was discussed, and the
user confirmed the square rotates again after restoring Spin.

Active: [Lesson 02 — A voxel grid owned by the world](lessons/02-voxel-data.md),
first A2 increment, complete for this lesson. User implementation reviewed with
no correctness findings; all three tests passed on the actual exercise. User
confirmed restored Some(1) output and continued square rotation at close.
Introduces plain Rust dense storage, per-axis bounds/indexing, and a resource
wrapper with one startup reader.

Next action on Continue learning: prepare only lesson 03, a small A2 step that
fills the existing voxel grid with a simple solid sphere and checks its data.
Build on lesson 02; defer rendering and a full geographic planet generator.
No lesson 03 has been prepared or implemented.

- Pending learning exercise: none from lesson 02. Cosmetic cleanup remains:
  trailing whitespace on planet.rs line 23, preserved because closing does not
  authorize source edits. This does not affect the completed learning outcome.
- User reported None from the [3, 0, 0] experiment and asked why. Explained valid
  axis ranges, Option propagation, and how flattening alone aliases [0, 1, 0].
  No further questions raised; do not infer broader understanding from tests.
- No planet generator, renderer or subsequent lesson prepared.

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
- Continue/review do not authorize Git delivery. The previous session invoked
  Close session; this session invoked Continue learning, Review learning and
  explicitly requested Close session after confirming the restored run.
- Lesson 02 keeps VoxelGrid independent of Bevy, owned by one PlanetVoxels
  resource for now. Multiple planets may motivate per-planet components later.
  Tiny fixture creation during plugin build is temporary; revisit for generation.

## Checks and evidence

Lesson 02 user implementation review (2026-09-20):

- main.rs, planet.rs and voxel.rs match the lesson's executable logic; scene.rs
  is unchanged. Reviewed bounds-before-flattening, checked dimension arithmetic,
  get/set behavior, resource ownership and insertion before Startup. No
  correctness findings within this lesson's scope.
- `cargo test --offline --locked --example learning` compiled the actual example
  test target and passed all three tests. Bevy 0.19.0 and Rust/Cargo 1.97.0
  reconfirmed. No unrelated application tests or GUI run performed.
- Tracked `git diff --check` passed. Explicit check of untracked source found
  trailing whitespace at planet.rs:23; voxel.rs had none. Left cleanup to user.
- User observed None during the bounds experiment. Source confirms restoration;
  user confirmed restored Some(1) output and current visual rotation at close.
- Final source logic still matches the reviewed/tested version. Reused the three
  passing tests; no repeat test run or assistant GUI execution at close.
- Closing scope: main.rs, planet.rs, voxel.rs, lesson 02 and this checkpoint.
  Final `git diff --cached --check` reported only the known planet.rs:23
  trailing whitespace (exit 2). Preserved this cosmetic issue in the commit.

Current lesson 02 preparation (2026-09-20):

- Confirmed Bevy 0.19.0 in manifest/lockfile and Rust/Cargo 1.97.0 locally.
- Exact three lesson Rust blocks plus unchanged learner scene.rs compiled in a
  temporary project with copied manifest/lockfile and shared target cache.
  Offline locked Cargo check and test for example learning passed: three tests
  cover non-cubic strides, write isolation and invalid-coordinate rejection.
- Installed Bevy 0.19.0 resource and App insertion source inspected.
- No learner source/dependency edits, user lesson 02 test run, startup-log
  observation or GUI validation. Passing isolated tests is preparation evidence,
  not evidence that the user implemented or completed this lesson.

Historical lesson 01 / previous close evidence:

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
Reinspect Git on resume; the recorded HEAD is the lesson 02 preparation baseline.
Keep future lessons small and record partial work without inferring understanding
from generated code or compilation alone.
