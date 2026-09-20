# Current learning checkpoint

Updated: 2026-09-20. See LEARNING_PLAN.md for the curriculum and root AGENTS.md
for shared rules and workflow skill routing.

## Verified repository context

- Branch: `learn/bevy-ecs`, tracking `origin/learn/bevy-ecs`.
- Lesson 05 preparation baseline: `be2ef69` — `feat: complete sphere materials learning lesson`.
  Clean worktree at start; HEAD equals the local origin/learn/bevy-ecs ref.
  Lesson 04 delivery commit is present locally. No live remote fetch performed;
  matching tracking refs are local evidence, not a fresh remote verification.
  Earlier pre-close HEADs below are historical.
- Cargo.toml/Cargo.lock: Bevy 0.19.0; Rust edition 2024; dynamic linking.
- Showcase in `src/` remains reference code, not built hands-on by the user.
- User implemented lesson 02 in `examples/learning/main.rs`, `planet.rs` and
  `voxel.rs`; `scene.rs` remains unchanged from lesson 01. Assistant edits are
  documentation only; dependencies remain unchanged.
- User implemented lesson 04 in planet.rs; executable lines match the lesson.
  Radius 3.0 and thickness 1.0 are restored. Lesson 05 now adds material.rs and updates main.rs and planet.rs.
  Scene, voxel storage, and dependencies remain unchanged from HEAD.
  Assistant review edits documentation only.

## Active checkpoint and next action

[Lesson 01 — An app and a scene plugin](lessons/01-app-and-plugins.md) is complete
for this increment: user implemented the example, assistant reviewed it without
findings, compilation passed, the Spin-removal experiment was discussed, and the
user confirmed the square rotates again after restoring Spin.

[Lesson 02 — A voxel grid owned by the world](lessons/02-voxel-data.md),
first A2 increment, complete for this lesson. User implementation reviewed with
no correctness findings; all three tests passed on the actual exercise. User
confirmed restored Some(1) output and continued square rotation at close.
Introduces plain Rust dense storage, per-axis bounds/indexing, and a resource
wrapper with one startup reader.

[Lesson 03 — Generate a solid voxel sphere](lessons/03-solid-sphere.md),
second small A2 increment, complete for this lesson. User implementation reviewed
with no correctness findings; all six actual-exercise tests passed. The radius
experiment and simple function walkthrough were discussed. User confirmed the
restored radius-3 output and continued square rotation, then requested closing
the lesson. Lesson 04 has now been implemented and reviewed.

Latest completed: [Lesson 04 — Assign materials inside the sphere](lessons/04-sphere-materials.md),
one small A2 increment. Assign rock to the core and soil to a fixed radial layer,
keeping air outside and the same resource ownership. User implementation reviewed
with no correctness findings; all eight actual-exercise tests pass. Thickness
experiment reported and discussed; radius 3.0 and thickness 1.0 restored in source.

User confirmed the restored expected startup output and continued square rotation
at close. Lesson 04 is complete for this increment. No assistant GUI validation.

Active: [Lesson 05 — Map material IDs to colors](lessons/05-material-palette.md),
implemented and reviewed; session saved with experiment color still applied.
User supplied the expected original palette log and confirmed continued rotation,
then supplied the experiment log: only surface changed to [60,160,70,255].
This demonstrates the palette's effect on displayed color values; voxel IDs and
square rendering are independent of the palette. No assistant GUI validation.

Resume check: HEAD `c2fb335` (`feat: reverted experiment checkpoint`), clean
learn/bevy-ecs worktree at start. User restored the RGB values but accidentally
changed `Some` to `ome` at material.rs:12. Current example fails compilation
(E0425: cannot find function `ome`); no tests executed on this revision.

Next action: user changes the line to `SOIL => Some([140, 95, 55, 255]),`, saves,
and runs the example tests/app. Finish lesson 05 restoration before advancing.
No lesson 06 prepared. Assistant updated documentation only; no source edits.

Lesson 05 close: user requested closing after reporting the experiment.
Authorized scope: main.rs, planet.rs, material.rs, lesson 05, and this checkpoint.
Pre-close HEAD: be2ef69. Destination: origin, learn/bevy-ecs
(https://github.com/mathiasvkaiz/the-game.git). Commit/push outcome will be verified
by Git after delivery; this entry does not claim a push has already succeeded.

Lesson 04 Close session authorized committing three session files and pushing
learn/bevy-ecs to origin (https://github.com/mathiasvkaiz/the-game.git).
Pre-close HEAD: f84cca9. At lesson 05 start, delivery commit be2ef69 is present
and matches the local origin tracking ref; no live remote verification performed.

The following observations and delivery notes concern completed lesson 03:

- User observed radius 2.0 output center=Some(1), surface=Some(0), outside=Some(0).
  Discussed distances 0/3/4 from [4,4,4], the fixed surface label, and valid air
  versus an invalid coordinate. Explained the generator in simple steps.
- Radius 3.0 is restored in source. User now confirms center=Some(1),
  surface=Some(1), outside=Some(0), and continued square rotation.
- Source review, six passing tests and user visual observations are separate
  evidence. No assistant GUI validation or inference of broader understanding.
- After the documentation-only lesson close, user explicitly requested Close
  session, authorizing commit and push of the five session files. Destination:
  origin (https://github.com/mathiasvkaiz/the-game.git), learn/bevy-ecs.
  At lesson 04 start, the lesson commit and subsequent documentation commit are
  present locally and HEAD matches the origin tracking ref; no live fetch done.
- No sphere rendering or geographic generator prepared.

## Teaching decisions and discussion

- User requested short, simple walkthroughs for complex functions in this and
  future lessons, in function comments or concepts as appropriate. Recorded in
  AGENTS.md and the lesson template; added solid_sphere's step-by-step explanation
  to lesson 03 concepts. Documentation only; executable lesson code unchanged.
- Discussed solid_sphere as creating air, finding the center, visiting cells,
  filling those within the radius, and returning the grid. Further questions
  welcome; discussion does not establish completion or replace source review.
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
- Continue/review do not authorize Git delivery. The lesson 03 session invoked
  Continue learning, Review learning, then Close session. A separate explicit
  request authorized the source walkthrough comment. The current lesson 04
  requests authorized preparation/review and now explicit Close session delivery;
  no assistant executable source edits.
- Lesson 04 keeps byte IDs and selects materials during sphere generation.
  The fixed soil layer is a temporary rule, not geography or exposed-face detection.
  Palette/rendering work remains deferred. User tried thickness 2 and reported
  unchanged logged samples. Discussed why distances 0/2/3/4 retain materials,
  while unlogged [5,4,4] at distance 1 changes from rock to soil.
- Discussed nested loops as layers/rows/cells, size indices [x,y,z] = [0,1,2],
  exclusive ranges and x-fastest storage. Explained squared distance with simple
  ASCII diagrams and concrete offsets. Radius 3 spans seven axial cell centers;
  radius 6 in a 9-cubed grid clips at the box, leaves far corners air (48 > 36),
  and exposes rock at face centers. Discussion is recorded, not an inference of
  broader understanding; further questions remain welcome.
- Lesson 02 keeps VoxelGrid independent of Bevy, owned by one PlanetVoxels
  resource for now. Multiple planets may motivate per-planet components later.
  In lesson 03 the user replaced the build-time fixture with Startup generation;
  the pure grid and single-resource ownership stayed the same.

## Checks and evidence

Lesson 05 resume (2026-09-20):

- Inspected c2fb335 and actual material.rs: RGB restoration committed with `ome`
  typo. Worktree initially clean; Cargo remains Bevy 0.19.0 / edition 2024.
- `cargo test --offline --locked --example learning` failed compilation with
  E0425 at material.rs:12. Previous nine passing tests apply to earlier versions.
- Exact correction supplied to learner; no exercise source edits or advancement.


Lesson 05 close (2026-09-20):

- User reported original center=[110,115,125,255], surface=[140,95,55,255],
  air=[0,0,0,0], all wrapped in Some, and continued rotation. Experiment log
  changed only surface to [60,160,70,255]. Source confirms that experiment remains.
- Final source differs from reviewed code only in SOIL's RGB palette entry.
  Ran `cargo test --offline --locked --example learning` again because that code
  changed: all nine tests passed. No assistant GUI run; app observations are user evidence.
- No staged changes or existing unpushed commits at close start. Reviewed five-file
  session scope; dependencies, showcase, scene, and storage remain unchanged.
  No assistant source edits. Save partial checkpoint with restoration pending.


Lesson 05 user implementation review (2026-09-20):

- Branch learn/bevy-ecs, HEAD be2ef69. No staged changes. Worktree scope:
  main.rs, planet.rs, new material.rs, lesson 05, and this checkpoint.
  Bevy 0.19.0, edition 2024, Rust/Cargo 1.97.0 reconfirmed.
- Reviewed shared IDs, palette cases/alpha, unknown-ID handling, module wiring,
  Option chaining, unchanged resource ownership and Startup order, and tests.
  Executable changes match the lesson. No correctness findings within lesson scope.
  Radius 3.0, thickness 1.0, and original soil RGBA [140,95,55,255] are present.
- `cargo test --offline --locked --example learning` on the actual exercise passed
  all nine tests. `git diff --check` passed; new material.rs has no trailing whitespace.
- No assistant GUI run or startup-log observation. User experiment and app
  observations have not yet been reported. No source/dependency edits, commit,
  push, or advancement. Prior cosmetic generator comment typo remains unchanged.


Lesson 05 preparation (2026-09-20):

- Read plan, checkpoint, completed lesson 04, template, and actual exercise files.
  Verified clean learn/bevy-ecs at be2ef69, matching local tracking ref; Bevy 0.19.0,
  edition 2024, Rust/Cargo 1.97.0. Reconciled historical pre-close checkpoint prose.
- Applied exact lesson blocks to a temporary exercise copy in
  /private/tmp/lesson05-3pskxtvx with copied manifest/lockfile and shared target cache.
  Offline locked Cargo test for example learning compiled and passed nine tests.
  New test covers generator/palette compatibility, alpha policy, invalid coordinate,
  and unknown ID; existing storage, geometry, and Startup tests also passed.
- Consulted installed Bevy 0.19.0 app/schedule sources; no new Bevy mechanism.
  Palette is a pure function, with no added resource or system. Runtime palette
  configuration is the condition for reconsidering that choice.
- Documentation-only preparation: no learner source/dependency edits, actual
  lesson 05 review, startup-log observation, GUI run, or learner experiment yet.
  Only lesson 05 and this checkpoint changed; no commits or pushes authorized.


Lesson 04 user implementation review (2026-09-20):

- Branch learn/bevy-ecs, HEAD f84cca9. Changes limited to planet.rs, this checkpoint
  and untracked lesson 04; no staged changes. Bevy 0.19.0, Rust/Cargo 1.97.0.
- Reviewed actual planet/main/scene/voxel integration, loop bounds and centering,
  squared-distance comparisons, core clamp, material IDs, resource ownership and
  chained Startup insertion/reader. Executable lines match lesson 04. No correctness
  findings within this lesson's small-grid scope. Radius 3.0/thickness 1.0 restored.
- `cargo test --offline --locked --example learning` on the actual exercise passed
  all eight tests. `git diff --check` passed. No GUI run or unrelated tests.
- Cosmetic comment typo at planet.rs:33: thicknexx should read thickness. Left to
  the user; no behavioral impact. No source or dependency edits by assistant.
- User reported the unchanged expected startup log during the thickness experiment.
  At review, restored-run output and visual confirmation were pending. User has
  now confirmed both at close. Experiment/distance discussion recorded above.
- Close: final executable code still matches the reviewed/tested lesson. Reused
  eight passing tests; no redundant test or GUI run. User source preserved,
  including the cosmetic comment typo. Closing scope: planet.rs, lesson 04 and
  this checkpoint. No next lesson prepared.


Lesson 04 preparation (2026-09-20):

- Read plan, checkpoint, completed lesson 03, template and actual exercise files.
  Reconfirmed clean learn/bevy-ecs at f84cca9, matching local origin tracking ref,
  Bevy 0.19.0 and Rust/Cargo 1.97.0; no dependency changes.
- Exact lesson 04 Rust block compiled in /private/tmp/lesson04-uq52kxm1 with
  copied manifest/lockfile and unchanged learner main/scene/voxel files.
  `cargo test --offline --locked --example learning` using that manifest and
  shared target cache passed all eight tests: storage, occupied geometry,
  material boundaries, tiny radii and windowless Startup integration.
- Consulted installed Bevy 0.19.0 scheduling/Commands sources. No new Bevy API.
- At preparation time, only lesson 04 and this checkpoint changed. No actual
  learner implementation review, lesson 04 log/visual observation, source edit or
  GUI run had occurred. Exercise and discussion were then pending; isolated tests
  establish preparation only. Later review and close evidence appear above.


Lesson 03 close: final executable code matches the reviewed/tested lesson. Reuse
the six passing tests and user-confirmed output/rotation. Scope: AGENTS.md,
PROGRESS.md, lesson template, lesson 03, and examples/learning/planet.rs.
No further source edits or GUI execution at close.

Lesson 03 user implementation review (2026-09-20):

- Repeat review after walkthrough addition: no new correctness findings. Current
  planet.rs executable lines still match the tested lesson; other exercise files
  and dependencies remain unchanged from HEAD. Reused the six passing tests;
  no redundant test or GUI run. Whitespace check passed. Final user observations
  were pending at repeat review and are now confirmed at lesson close.

- Branch learn/bevy-ecs, HEAD b834a89. Reviewed planet.rs and its existing
  main/scene/voxel integration against lesson 03. Executable lines match the
  lesson. Checked centering, squared-distance boundary inclusion, valid writes,
  pure-grid ownership and chained Startup insertion/reader. No correctness findings
  within this lesson's small-grid scope; radius 3.0 restored.
- `cargo test --offline --locked --example learning` on the actual exercise passed
  all six tests, including even-grid geometry and the windowless Startup chain.
  Rust/Cargo 1.97.0. `git diff --check` passed.
- User then explicitly requested the simple walkthrough in the existing example
  code. Added it to solid_sphere's doc comment and fixed non-finize to non-finite.
  Executable lines remain identical to the tested lesson; reused passing tests.
- User's radius-2 log is prior observation. At lesson close, the user confirmed
  the restored radius-3 log and continued rotation. No assistant GUI run. Reused
  passing tests because executable code remains unchanged; lesson 04 not prepared.


Lesson 03 preparation (2026-09-20):

- Verified clean branch/HEAD and local tracking ref at b834a89, Bevy 0.19.0 in
  manifest/lockfile, Rust/Cargo 1.97.0. Read the completed lesson and actual source.
- Exact lesson Rust block extracted into a temporary project with copied
  manifest/lockfile and unchanged learner main.rs, scene.rs and voxel.rs.
  Offline locked Cargo test for example learning compiled and passed all six
  tests using the shared target cache: three prior storage tests, a seven-cell
  sphere fixture, even/non-cubic centering, and the actual startup chain headlessly.
- Installed Bevy 0.19.0 chain and Commands insertion sources inspected for
  ordering and automatic deferred application under default schedule settings.
- No exercise source/dependency edits, learner implementation review, lesson 03
  startup-log observation or GUI validation. Tests establish preparation only.


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

Historical lesson 02 preparation (2026-09-20):

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
Reinspect Git on resume; the recorded HEAD is the pre-close baseline.
Keep future lessons small and record partial work without inferring understanding
from generated code or compilation alone.
