# 01 — An app and a scene plugin

Status: completed and reviewed for this increment (2026-09-20).
Prerequisite checkpoint at lesson preparation: no prior lessons; `examples/` was absent.
Target: branch `learn/bevy-ecs`; create the two files below.
Version context: Cargo.toml and Cargo.lock use Bevy 0.19.0, Rust edition 2024.

## Outcome

Run a separate learning example and see a blue square rotating at the center of
a window. This establishes the app and scene boundary we will use while
reconstructing the showcase. The square is a temporary presentation placeholder;
replace it when we reach the voxel planet display.

## Apply it yourself

From the repository root, create `examples/learning/`. Create both files with the
complete contents below. No Cargo.toml changes or external assets are needed:
Cargo discovers `examples/learning/main.rs` as the example named `learning`.

### Create `examples/learning/main.rs`

```rust
mod scene;

use bevy::prelude::*;
use scene::ScenePlugin;

/// Program entry point: configure the learning app, then start Bevy's runner.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenePlugin)
        .run();
}
```

### Create `examples/learning/scene.rs`

```rust
use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    /// Bevy calls this when main adds ScenePlugin, before run().
    /// Register the scene's systems; do not execute them here.
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_systems(Update, spin_squares);
    }
}

#[derive(Component)]
/// Per-entity rotation speed, read by the rotation system.
struct Spin {
    radians_per_second: f32,
}

/// Bevy runs this once during Startup, supplying Commands to queue world changes.
/// Create the camera and the square that the update system will rotate.
fn setup_scene(mut commands: Commands) {
    // Spawns are queued; Bevy applies them before our Update system runs.
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(120.0)),
        Transform::default(),
        Spin {
            radians_per_second: 1.0,
        },
    ));
}

/// Bevy runs this during Update and supplies time and component access.
/// Rotate entities with both Spin and Transform; no matches means no iterations.
fn spin_squares(time: Res<Time>, mut squares: Query<(&Spin, &mut Transform)>) {
    for (spin, mut transform) in &mut squares {
        // Convert radians per second into rotation for this frame.
        transform.rotate_z(spin.radians_per_second * time.delta_secs());
    }
}
```

## Verify

Run from the repository root after creating both files:

```sh
cargo check --offline --example learning
cargo run --offline --example learning
```

The first command checks compilation. The second should open a window with a
centered blue square, 120 world units wide, rotating counterclockwise about its
center. A full rotation takes about 6.28 seconds at normal speed. Close the window
to exit. Plain `cargo run` still launches the showcase.

If Cargo cannot find `learning`, check that the entry point is exactly
`examples/learning/main.rs`. If Rust cannot find `scene`, check that `scene.rs` is
alongside it. A blank window suggests checking the camera and sprite spawns. A
stationary square suggests checking its `Spin` component and the `Update`
registration. Offline dependency errors mean required crates are not cached.

Tutor verification (2026-09-20): APIs inspected against installed Bevy 0.19.0
sources and examples. The exact two Rust blocks were extracted into a temporary
project with copies of this repository's Cargo.toml/Cargo.lock;
`cargo check --offline --locked --example learning` passed (using that temporary
manifest and the repository's target cache). Rust/Cargo version: 1.97.0. No learner
source files were created by the assistant. No assistant GUI execution or visual
validation was performed; later user confirmation is recorded below. Visual observation is the useful acceptance check here; a
unit test duplicating the setup would not establish that rendering works.

## Design and concepts

> ℹ️ **Bevy — Who calls what, and when?**

`App` holds the ECS world and schedules. `DefaultPlugins` supplies the window,
renderer, input, time and other standard facilities. A plugin groups app
configuration; a system is a function Bevy executes as part of a schedule.

`build` is required by the `Plugin` trait; systems themselves need no `build`
method. The function comments describe each function's role. The sequence below
connects registration to later execution.

Here is the relevant sequence for this example (other engine stages omitted):

```text
main()
  App::new()                    create the app
  add_plugins(DefaultPlugins)   configure standard engine facilities
  add_plugins(ScenePlugin)      Bevy calls ScenePlugin::build here
    add_systems(Startup, ...)   register setup_scene; do not execute it yet
    add_systems(Update, ...)    register spin_squares; do not execute it yet
  run()                         hand control to the application runner
    Startup                     Bevy executes setup_scene once
    apply queued spawns         camera and square become available
    Update                      Bevy executes spin_squares each app update
    ...                         updates continue while the app runs
```

Passing `setup_scene` without parentheses passes the function to Bevy for later
execution. Writing `setup_scene(...)` would call it immediately. Bevy adapts the
registered function into a system and supplies its declared system parameters
when it runs: `Commands` for queuing changes, `Res<Time>` for accessing time, and
`Query` for accessing matching components. The user does not construct these
arguments or call the systems from `main`.

> ℹ️ **Bevy — Queued changes and scheduling**

`Commands` defers structural world changes until Bevy applies them. The normal
startup/update boundary handles our spawn dependency, so we need no explicit
ordering here. Adding more update systems later will not automatically give
them source-code execution order.

> ℹ️ **Bevy — Who owns the data?**

An entity is an identity with attached components. The square has `Sprite` for
appearance, `Transform` for position/rotation/scale, and our `Spin` data. The ECS
world owns these components. The camera is another entity; it has no `Spin`.
Bevy's required-component mechanism adds supporting components when we spawn
`Camera2d` or `Sprite`, including transform/visibility support where needed.

The query borrows each matching entity's speed read-only and transform mutably;
the camera lacks `Spin`, so it is excluded. Bevy uses these declared accesses to prevent
conflicting system execution. `Res<Time>` borrows a shared time resource: one
world-level value, rather than data attached to each object.

> ℹ️ **Rust — What does each mut allow?**

Rust bindings are immutable by default. `mut name` makes a local binding mutable;
`&mut T` is an exclusive mutable borrow of a value. They serve different roles:

| In this code | What it allows |
| --- | --- |
| `app: &mut App` | Modify the borrowed app. The binding `app` does not need `mut` to modify the value through this reference. |
| `mut commands: Commands` | Mutably borrow the commands wrapper when calling methods such as `spawn`, which queue world changes. |
| `&Spin` in the query | Read each matching entity's speed without modifying it. |
| `&mut Transform` in the query | Declare mutable access to matching transform components. |
| `mut squares: Query<...>` | Mutably borrow the query for iteration using `&mut squares`. |
| `mut transform` in the loop | Mutably borrow Bevy's `Mut<Transform>` wrapper so `rotate_z` can change the component through it. |

The query yields a change-tracking wrapper for the transform, rather than a plain
owned `Transform`. Removing `mut transform` prevents the mutable borrow needed
by `rotate_z`; it does not merely make rotation slower or disable it at runtime.
The ECS world still owns the component. We borrow it during system execution;
we do not move it out of the world or retain a reference between frames.

> ℹ️ **Rust — Traits, derives and module visibility**

`///` documents the following item and can appear in generated Rust documentation;
`//` comments explain implementation details inside the code. Neither changes
runtime behavior.

`impl Plugin for ScenePlugin` implements Bevy's trait contract for our type;
`fn build(&self, app: &mut App)` uses a shared borrow of the plugin and a mutable
borrow of the app. The plugin can configure the app without changing itself.
`#[derive(Component)]` generates the trait implementation that makes `Spin`
usable as an ECS component. `pub` exposes the plugin to the parent module;
the scene's data and systems stay private. `mod scene;` loads the sibling
`scene.rs` module, while `use scene::ScenePlugin;` brings that name into scope.

### Why these boundaries?

We could put all systems in `main.rs`, but a scene plugin gives us a small boundary
for future scene work. We could store one global rotation speed in a resource,
but a component lets each future object have its own speed. Avoid introducing
additional plugins or ordering constraints until there is a concrete need.

## Experiment and discuss

Before changing anything, predict what happens if you remove only the `Spin`
component from the square's spawn tuple. Will it disappear, stop rotating, or
cause an error? Explain your prediction using the query's matching requirements.
Then try it and compare the result with your prediction. Restore `Spin` afterward.

## Checkpoint

- [x] Lesson prepared; validation status recorded above.
- [x] User applied the code; source reviewed and actual example compilation passed.
- [x] Result verified: user confirmed rotation after restoring Spin.
- [x] Spin-removal experiment discussed; further questions welcome.

Review (2026-09-20): user files match the lesson logic. At close, the user had
also added the teaching comments; both files match the lesson code exactly.
No correctness findings; `cargo check --offline --locked --example learning`
passed on the actual exercise. `Spin` is restored. The user reported that removing
it stopped rotation without an error; no assistant GUI validation was performed.

User confirmed restored rotation and requested session close. Implementation,
review, compilation, observation and experiment discussion are recorded. Next
session may prepare the first small voxel-data lesson; see PROGRESS.md.

Decision: use a 2D sprite and one scene plugin to introduce ECS with a visible
result and no asset setup. Revisit the placeholder when displaying voxel data;
this is not yet a voxel model or renderer.
