# Hands-on Bevy / ECS learning roadmap

## Start / resume

Read root `AGENTS.md`, then this plan, then [PROGRESS.md](PROGRESS.md) and the
active lesson recorded there. Inspect the real branch and files before continuing.
The checkpoint records partial work so a fresh session does not need chat history.

Fresh-session prompt: **"Resume our hands-on Bevy/ECS learning path from
doc/PROGRESS.md. Prepare or continue only the next small step."**

## Learner and objective

The user has 20 years of software development experience across several languages
and some basic Rust/Bevy experience. **Bevy and ECS are the primary subjects;
Rust is the sidekick.** Explain Rust-specific ownership, borrowing, traits and
concurrency where they matter, without a generic programming introduction.

Learn modular, extensible, high-performance engineering by reconstructing the
voxel planet showcase hands-on, then improving it toward production quality.
Explain choices, alternatives, limitations and measurable outcomes at each step.
Production quality is a goal to verify, not a label to attach to tutorial code.

The longer-term idea is a Mega lo Mania-inspired planet strategy game, potentially
with altered/destructible terrain. Its design remains open. Helicopters, missions
and waypoints are demonstration features, not mandatory future gameplay.

## Learning workflow

1. Tutor prepares one small Markdown lesson using [the template](lessons/TEMPLATE.md).
2. Discuss the engineering decision and relevant Bevy/ECS mechanisms.
3. User applies complete copyable code with explicit paths and replacements.
4. Run meaningful checks and make a small experiment or prediction.
5. Review together, record progress and optionally make a user-owned checkpoint commit.

Default: the tutor edits documentation, not exercise source, unless explicitly
asked to implement or fix code. Prepare lessons just in time rather than generating
a whole course. Keep each increment runnable, reviewable and economical in tokens.
No numerical token budget is set. Stop at learning checkpoints, not after silently
implementing several stages ahead.

Keep `planet` as the reference. A separate learning branch is recommended, but
its creation/name and the exercise location are not yet decided. Preserve existing
work. Carry AGENTS.md and these documents onto the learning branch so instructions
remain available there. No branch changes or source replacement are authorized
merely by reading this plan.

## Part A — Reconstruct the showcase

Each phase may require multiple lesson files. They are milestones, not giant lessons.

| Phase | Observable result | Main concepts |
| --- | --- | --- |
| A1 | Modular app, camera and visible object | App lifecycle, plugins, startup/update schedules, components, queries |
| A2 | Voxel data and generated planet | Data layout, coordinates, indexing, palette, generation; pure data vs ECS ownership |
| A3 | Displayed, rotating planet | CPU voxel renderer, image assets, navigation, time, presentation boundaries |
| A4 | Helicopter and animated rotor | Voxel model, entity hierarchy, local/global transforms, animation systems |
| A5 | Waypoints, route and autopilot | Geographic projection, visibility, selection state, system dependencies |
| A6 | Mission panel, controls and briefing | UI/presentation state, change-driven updates, interaction and resizing |

Recreate the reference behavior and appearance, not its monolithic organization.
Use focused modules/plugins from the start where they represent real boundaries.
Explain the CPU renderer as an intentional small-showcase implementation with
known limits. UI and marker shortcuts should likewise have explicit boundaries.
No beginner territory/population/economy detour.

## Part B — Production-oriented improvements

Begin after Part A is built and reviewed with the user. GPU migration is **not**
the immediate next lesson; the earlier GPU-first plan is superseded.

| Phase | Improvement | Engineering topics |
| --- | --- | --- |
| B1 | Measured GPU-rendered voxel planet | Baseline; exposed-face mesh builder; normals, winding, colors; mesh/material assets; 3D camera, lighting and pixel styling |
| B2 | Strong data/ECS boundaries | Authoritative voxels vs derived geometry; lifecycle, asset ownership, plugins, necessary ordering and parallelism |
| B3 | Correct response to terrain changes | Dirty tracking/revisions, mesh invalidation; localized rebuilds or background tasks when justified |
| B4 | Accurate picking and placement | Screen/world/local spaces, inverse transforms, voxel ray hits, normals and terrain occlusion |
| B5 | Verified performance and robustness | CPU/GPU frame costs, memory/geometry budgets, regression checks, resizing and asset cleanup |

Validation and design apply throughout, not only in B5. For a small globe, start
with one exposed-face mesh; rotation updates a transform, not terrain geometry.
GPU raycasting is an alternative to discuss. Greedy meshing, chunking, asynchronous
generation, level of detail and streaming should follow evidence and requirements.
Do not create an entity or draw call per voxel. A one-mesh rebuild on an edit is
acceptable initially; it is not the same as localized chunk rebuilding.

Surface regions may suffice for future forests, deposits and damaged land.
Elevation can support craters; caves/overhangs are stronger reasons for volume
editing. No full terrain engine, economy, combat or playable missions are in scope.

## Verification and teaching quality

- Match APIs to Cargo.toml/Cargo.lock and version-matched official/local examples.
  Current reference uses Rust edition 2024 and Bevy 0.19.0.
- Give complete small files or exact replacements. State prerequisites, paths and
  imports. Distinguish tested code from illustrative or unverified code.
- Test useful invariants: indexing/bounds, projection, mesh faces and navigation.
  Use visual checks for camera/UI; don't add tests that just mirror the code.
- Verify lesson code in isolation where practical, never by overwriting the user's
  exercise source. Report actual checks, not assumed success.
- Track what the user has applied and discussed separately from what the tutor
  generated. A passing test does not establish learner understanding.
- Capture short design decisions (choice, reason, revisit condition) in the lesson.
  Avoid a separate heavyweight decision-log system until it becomes useful.

## Reference implementation

- `src/main.rs`: application and plugins.
- `src/world_map.rs`: navigation, six waypoints, markers, animation, panel,
  procedural geographic atlas, tests and smoke check.
- `src/voxel.rs`: small CPU voxel renderer and helicopter volume.
- `src/components/` and character assets: retained, currently unused.

The planet is CPU-rendered into an image; the helicopter body is pre-rendered
from voxels with an animated sprite rotor. Flags use approximate spherical anchors.
No terrain editing or GPU mesh migration has been implemented yet.

Reference commands: `cargo run`, `cargo test`, `cargo run -- --smoke-test`.
The smoke check opens a GUI, captures `target/world-map*.png`, and exits.
GUI execution may require environment approval. Controls: WASD/arrows fly, Shift
boosts, Tab chooses the next mission/autopilot, Space toggles autopilot, Enter opens
a nearby briefing, Escape returns. Lesson-specific commands may differ.

Current progress, historical verification and the exact next action live only in
[PROGRESS.md](PROGRESS.md). No hands-on lessons have been completed at plan creation.
