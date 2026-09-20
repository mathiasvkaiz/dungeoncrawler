# 03 — Generate a solid voxel sphere

Status: complete and reviewed for this increment (2026-09-20).
Prerequisite: lesson 02 complete; its four exercise files remain in place.
Target: `learn/bevy-ecs`, `examples/learning/planet.rs`.
Version context: Bevy 0.19.0, Rust edition 2024, Rust/Cargo 1.97.0.

## Outcome

Generate a centered solid sphere in a 9 × 9 × 9 grid during Startup. Read its
center, surface and surrounding air after insertion into the ECS world. The
square remains the visual placeholder; this step produces and checks data only.

## Apply it yourself

Replace **all of `examples/learning/planet.rs`** with the following. Keep
`main.rs`, `scene.rs`, `voxel.rs` and dependencies unchanged.

```rust
use bevy::prelude::*;

use crate::voxel::VoxelGrid;

pub struct PlanetPlugin;

/// The ECS world owns this resource; it owns the plain Rust voxel grid.
#[derive(Resource)]
struct PlanetVoxels {
    grid: VoxelGrid,
}

impl Plugin for PlanetPlugin {
    /// Bevy calls this when main adds the plugin, before run().
    /// Register generation followed by its reader; neither runs here.
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_planet, report_planet).chain());
    }
}

/// Bevy runs this once in Startup, supplying a deferred command queue.
/// Build the data locally, then queue transfer of ownership to the world.
fn generate_planet(mut commands: Commands) {
    let grid = solid_sphere([9, 9, 9], 3.0);
    commands.insert_resource(PlanetVoxels { grid });
}

/// Called by generate_planet or tests; return fresh, centered sphere data.
/// Radius is in cell spacings. Oversized spheres are clipped by the grid.
/// Panics for invalid dimensions (via VoxelGrid) or a non-finite/negative radius.
fn solid_sphere(size: [usize; 3], radius: f32) -> VoxelGrid {
    assert!(radius.is_finite() && radius >= 0.0, "invalid radius");
    let mut grid = VoxelGrid::new(size);
    // Integer coordinates denote cell centers; even dimensions center between cells.
    let center = size.map(|axis| (axis as f32 - 1.0) * 0.5);
    let radius_squared = radius * radius;

    for z in 0..size[2] {
        for y in 0..size[1] {
            for x in 0..size[0] {
                // Convert before subtracting: offsets can be negative.
                let dx = x as f32 - center[0];
                let dy = y as f32 - center[1];
                let dz = z as f32 - center[2];
                if dx * dx + dy * dy + dz * dz <= radius_squared {
                    // These loop bounds guarantee that the write is valid.
                    assert!(grid.set([x, y, z], 1));
                }
            }
        }
    }
    grid
}

/// Bevy runs this once after generation and its queued insertion have completed.
/// Bevy supplies shared access to the resource declared by the parameter.
fn report_planet(planet: Res<PlanetVoxels>) {
    info!(
        "Sphere: center={:?}, surface={:?}, outside={:?}",
        planet.grid.get([4, 4, 4]),
        planet.grid.get([7, 4, 4]),
        planet.grid.get([8, 4, 4]),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cargo checks a hand-countable sphere: center plus six axial neighbors.
    #[test]
    fn radius_one_has_seven_cells() {
        let grid = solid_sphere([5, 5, 5], 1.0);
        let mut occupied = 0;
        for z in 0..5 {
            for y in 0..5 {
                for x in 0..5 {
                    if grid.get([x, y, z]) == Some(1) {
                        occupied += 1;
                    }
                }
            }
        }
        assert_eq!(occupied, 7);
        assert_eq!(grid.get([2, 2, 2]), Some(1));
        assert_eq!(grid.get([3, 2, 2]), Some(1));
        assert_eq!(grid.get([3, 3, 2]), Some(0));
    }

    /// Cargo checks half-cell centering on all axes of a non-cubic even grid.
    #[test]
    fn even_grid_centers_between_cells() {
        let grid = solid_sphere([4, 6, 8], 0.9);
        for z in 0..8 {
            for y in 0..6 {
                for x in 0..4 {
                    let inside = (1..=2).contains(&x)
                        && (2..=3).contains(&y)
                        && (3..=4).contains(&z);
                    assert_eq!(grid.get([x, y, z]), Some(if inside { 1 } else { 0 }));
                }
            }
        }
    }

    /// Cargo drives a windowless app to check the producer/consumer startup path.
    #[test]
    fn startup_inserts_sphere_before_reader() {
        let mut app = App::new();
        app.add_plugins(PlanetPlugin);
        app.update();
        let planet = app.world().resource::<PlanetVoxels>();
        assert_eq!(planet.grid.get([4, 4, 4]), Some(1));
        assert_eq!(planet.grid.get([7, 4, 4]), Some(1));
        assert_eq!(planet.grid.get([8, 4, 4]), Some(0));
    }
}
```

## Verify

From the repository root:

```sh
cargo test --offline --locked --example learning
cargo run --offline --locked --example learning
```

Expect six passing tests: three existing storage tests and three new tests above.
The startup log should contain
`Sphere: center=Some(1), surface=Some(1), outside=Some(0)` once. The square should
still rotate. A sphere is not yet drawn. Close the window to exit.

If only three tests run, check that you replaced the correct example file.
A missing-resource error points to missing generation or its ordering dependency.
`Some(0)` is valid air; `None` would indicate an out-of-bounds sample.

Tutor verification (2026-09-20): extracted the exact Rust block into a temporary
project with copied Cargo.toml/Cargo.lock and the existing main.rs, scene.rs and
voxel.rs. `cargo test --offline --locked --example learning` (using that temporary
manifest and the shared target cache) compiled and passed all six tests, including
the windowless startup test. Checked chain/deferred-insertion behavior against
installed Bevy 0.19.0 sources. At preparation time, no learner source edits,
actual lesson 03 exercise run, startup-log observation or GUI validation had been
performed. Later implementation review and user observations are recorded below.

## Design and concepts

> ℹ️ **The algorithm — solid_sphere in simple steps**

`solid_sphere` takes a grid size and a radius, then returns a grid containing a
solid ball surrounded by air.

1. Check that the radius is valid and non-negative.
2. Create an empty grid: every cell starts as `0` (air).
3. Find its center. For `[9, 9, 9]`, this is `[4, 4, 4]`.
4. Visit every cell using the z, y and x loops.
5. Work out how far that cell is from the center. `dx`, `dy` and `dz` are
   its offsets along the three axes.
6. If it is within the radius, including the boundary, set it to `1` (solid).
   The comparison uses squared distances so it does not need a square root.
   Cells farther away stay air.
7. Return the filled grid with the final `grid` expression.

For example, `[7, 4, 4]` is 3 cell spacings from the center: solid with radius 3,
air with radius 2. Think of a ball inside a box: the ball is solid and the space
around it is air.

> ℹ️ **Bevy — Startup ordering and deferred ownership**

```text
add_plugins → build registers the chain
run → Startup: generate_planet → apply queued insertion → report_planet
    → Update: the existing square animation continues
```

Bevy supplies `Commands` and `Res` from system parameter declarations. Our code
calls `solid_sphere` as an ordinary Rust function; Bevy does not schedule it.
Generation executes once during Startup, not on every frame. Moving it out of
`build` separates app configuration from world initialization.

Unlike the previous `app.insert_resource`, `commands.insert_resource` queues a
world change. Until Bevy applies that queue, the resource is not available.
With the default Bevy 0.19 schedule settings, `.chain()` both orders these
systems and provides a deferred-application point for the producer's commands
before the consumer. Tuple order alone does not establish this dependency.
The camera/square startup system remains independent of this chain.

Ownership travels from a local grid into a queued `PlanetVoxels`, then into the
world. `report_planet` only borrows it. The cells remain plain data, with no entity
per cell. The headless test uses `app.update()` to drive schedules once without
installing window/render plugins; it exercises the actual registered chain.

> ℹ️ **Geometry — Cell centers define the sampling rule**

The center of indices 0 through 8 is 4, so `(axis - 1) / 2` centers an odd grid
on a cell and an even grid between cells. The formula is applied independently
to all three dimensions. A cell is solid when its center is on or inside the
sphere: `dx² + dy² + dz² <= radius²`. Squared distances avoid needing a square
root. Every qualifying cell, including the interior, is filled; this is not a
hollow shell or a surface mesh.

This sampling rule differs from filling every cube that intersects the sphere.
It gives a discrete, stepped boundary. Radius is measured in cell spacings here;
future rendering will choose a mapping to world units. Material 1 still has no
assigned color. A future image or mesh will be derived from this data.

> ℹ️ **Rust — Local mutation before a move**

`let mut grid` permits `set` to borrow the owned grid through `&mut self`.
Returning `grid` moves it out; the final expression needs no `return` or semicolon.
The caller's `let grid` needs no `mut` because it only moves that value onward.
`mut commands` permits methods requiring mutable access to the command wrapper;
it does not grant direct mutable access to the world. `app: &mut App` already
provides a mutable reference, so its binding need not be `mut`.

Array `.map` returns a new three-element array. The closure `|axis| ...` converts
each dimension to `f32` using `as`. Coordinates are also converted before
subtraction to allow negative offsets without unsigned underflow. `0..size[0]`
excludes the endpoint. `use super::*` brings the parent module's names into its
test module. These tiny dimensions are exactly representable; this is not a
large-world precision strategy.

The generator is synchronous and visits every cell, with storage proportional
to the grid volume. Startup still blocks while it runs. Keep this simple for
729 cells; measure larger workloads before adding background generation. The
programmer-chosen sizes/radius and fixed material are teaching constraints, not
a validated external-data interface. Revisit budgets and error handling before
accepting external inputs. Keep the helper beside its only owner for now; move
generation into its own module when multiple generators justify that boundary.

## Experiment and discuss

Change only `3.0` to `2.0` in `generate_planet`. Before running, predict all three
logged values. Which sample changes, and why? Run the app, compare your prediction,
then restore `3.0` before rerunning the tests and requesting review.

## Checkpoint

- [x] Lesson prepared and isolated validation recorded.
- [x] User replaced planet.rs; reviewed with no correctness findings.
- [x] Actual exercise tests passed; user confirmed restored startup log and rotation.
- [x] Radius experiment and simple function walkthrough discussed.

Decision: generate plain data during Startup, then expose it through one resource
with an explicit producer/consumer dependency. Keep rendering and geographic
generation for later increments. Stop here for implementation and discussion.

Review (2026-09-20): actual exercise passed all six tests under
`cargo test --offline --locked --example learning`. Radius 3.0 is restored in
source. User reported the radius-2 output; discussed fixed samples and the sphere
algorithm. User confirmed the restored radius-3 startup output
(center=Some(1), surface=Some(1), outside=Some(0)) and continued square rotation,
then requested closing this lesson. No assistant GUI validation.
At the user's explicit request, the simple walkthrough also appears in the
exercise function's doc comment; executable code still matches this lesson.
