# Start here: Rust, Bevy and ECS learning project

## Session handoff — 2026-09-19

Working branch at handoff: `planet`. Check `git branch --show-current` and
`git status --short` before editing; do not assume the branch or files stayed
unchanged. The previous globe prototype was committed by the user. The latest
requested work is this document and a small voxel appearance showcase.

The user is learning Rust and Bevy and is especially interested in ECS. Their
long-term idea is a Mega lo Mania-inspired territory strategy game involving a
planet, potentially with altered/destructible terrain. The design is deliberately
undecided. The helicopter and missions are only a capabilities demonstration,
not requirements for the future game.

**Next learning step: discuss and build one clickable territory that displays
its population and owner. No lessons have been completed yet.** Do not begin by
implementing an entire strategy game or explaining all the globe renderer's math.
First recap the current state briefly and agree on the next small learning step.

## How to work with the user

- Teach one small feature at a time: explain, make a small change, run it, give
  the user a small modification to try, then discuss the result.
- Introduce Rust concepts where they are used; explain unfamiliar syntax.
- Preserve the globe as a reference. Agree where the learning example will live
  before replacing the demonstration with lesson code.
- Keep changes and explanations economical. The user explicitly does not want
  an expensive, token-heavy full game implementation. No numerical budget was set.
- Explain design choices, especially ECS composition, data ownership, queries,
  scheduling, and performance. Do not optimize speculatively.
- After each lesson update the progress checklist, concepts understood, and exact
  next action below so another session can continue without conversation history.

## Intended tiny game loop

Select a territory, assign workers to gather resources, build a settlement, and
expand into a neighboring territory. This is a proposed learning direction,
not a finalized game specification.

| Stage | Small playable outcome | Rust / Bevy / ECS concepts |
| --- | --- | --- |
| 1 | Click one territory; display population and owner | Structs, functions, entities, components, systems, input |
| 2 | Several territories with different owners | Queries, references, borrowing, enums |
| 3 | Workers produce resources over time | Mutable queries, shared resources, simulation timing |
| 4 | Construct buildings and expand | Composition, system communication, ordering |
| 5 | Damage buildings and change terrain | State transitions, reflecting simulation changes visually |
| 6 | Show territories on a rotating planet | Coordinates, projection, rendering, sphere picking |

Example components: `Population`, `Owner`, `ResourceDeposit`. A production
system queries eligible territories. Prefer composition to a hierarchy of
territory classes. Decide component boundaries from behavior, not every field.

## World representation and appearance

Start with surface tiles or regions. Clearing forests, depleting deposits,
destroying buildings, and changing land to wasteland do not need a voxel engine.
Elevation can add craters later. Tunnels, caves, and overhangs are stronger reasons
to introduce volumetric voxels. None of these features is implemented yet.

Pixel art is an appearance, not a restriction to flat geometry. Options:

1. A low-resolution CPU-rendered sphere with a pixel world texture (original demo).
2. A textured 3D sphere or shader rendered at low resolution (possible GPU path).
3. Colored volume cells forming a blocky planet (current small voxel showcase).

For the learning game, surface tiles remain the recommended starting point.
The voxel showcase is an appearance experiment, not a commitment to its engine
architecture. A deliberate palette and silhouettes matter more than pixelation alone.

## ECS and performance

- Entities/components: territories, units, buildings, interactive objects.
- Resources: shared map data, simulation settings, selected territory.
- Systems: selection, production, construction, damage, visual updates.
- Independent systems can run concurrently when data access permits. The demo
  currently chains update systems for simplicity; that is not a performance model.
- ECS does not automatically accelerate pixel rendering. The small showcase
  renders on the CPU into an image; a GPU renderer is a later, measured decision.
- One entity per territory is reasonable for a small map. Large terrain should
  generally store cells in chunks; don't create an entity/draw call per voxel.

## Current repository / demonstration

- `src/main.rs`: Bevy application, window, plugins.
- `src/world_map.rs`: navigation, six missions, markers, helicopter animation,
  briefing panel, procedural geographic atlas, tests and visual smoke check.
- `src/voxel.rs`: compact CPU voxel rendering and voxel helicopter artwork.
- `src/components/` and existing character assets: retained, currently unused.
- `README.md`: run instructions and this handoff entry point.

The visual showcase uses a small colored voxel volume for the planet and a
pre-rendered voxel helicopter body with animated sprite rotor. Existing spherical
navigation, routes, mission descriptions and controls remain. Flags use approximate
spherical anchors, not voxel-surface picking. There is no mining, terrain editing,
combat, economy, persistence, or playable mission level.

Commands:

Validation at handoff: navigation/projection tests and voxel axis-aligned ray
checks pass; the visual smoke check ran successfully on the Mac's Metal renderer.
The initial globe screenshot was inspected for terrain and helicopter appearance.

```sh
cargo run
cargo test
cargo run -- --smoke-test
```

Controls: WASD/arrows fly, Shift boosts, Tab selects the next mission and engages
autopilot, Space toggles autopilot, Enter near a destination opens its placeholder
briefing, Escape returns. The smoke check opens a GUI window, saves three images
to `target/world-map*.png`, and exits. GUI execution may require environment approval.

## Learning progress

- [ ] Lesson 1: selectable territory, owner and population.
- [ ] Lesson 2: multiple territories and queries.
- [ ] Lesson 3: timed resource production.
- [ ] Lesson 4: building and expansion.
- [ ] Lesson 5: damage / terrain state changes.
- [ ] Lesson 6: planet presentation.

Concepts practiced by the user: none recorded yet. Prior discussion covered ECS
at a conceptual level only. Do not assume fluency with Rust ownership or Bevy APIs.

Suggested fresh-session prompt: "Read doc/LEARNING_PLAN.md, inspect the current
branch, and guide me through lesson 1 in small steps. Explain before implementing."

References: [Bevy ECS introduction](https://bevy.org/learn/quick-start/getting-started/ecs/)
and the locally installed Bevy 0.19 examples. Match examples to Cargo.toml;
Bevy APIs change between releases.
